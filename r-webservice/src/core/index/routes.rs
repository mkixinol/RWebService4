use std::str::FromStr;
use std::sync::mpsc;
use std::collections::{HashMap, HashSet};
use once_cell::sync::OnceCell;
use mime::Mime;
use sqlx::{PgPool};
use actix_web::{web, http, guard, HttpRequest, HttpResponse};
use actix_session::Session;
use futures_util::TryStreamExt;
use super::RWIndex;
use crate::core::{index::RWAuth, cache::RWCache, RequestValueAny};
use crate::module::{Page, Action, Template, Auth, ModuleDbAccess};
use crate::r#type::{UDbId, Method, DataFormat};

macro_rules! rw_index_closure_routing {
    (
        ($route: ident, $page_id: ident, $action_id: ident),
        $($_typing: ident),+
    ) => {
        $route.to(
            move |
                db: web::Data<Option<&'static PgPool>>,
                cache: web::Data<Option<&'static RWCache>>,
                request: HttpRequest,
                session: Session,
                body: RequestValueAny,
            | {
                RWIndex::index(
                    db,
                    cache,
                    $page_id,
                    $action_id,
                    request,
                    session,
                    body,
                    async {
                        //Err(ServiceError::HTTPRequestNotFound)
                        Ok(RWAuth::new(vec![]))
                    }
                )
            }
        )
    };
}

pub struct RWRouter();
impl RWRouter {
    pub fn route(cfg: &mut web::ServiceConfig, pool: &'static PgPool, cache: &'static OnceCell<RWCache>) {
        let arbiter = actix_rt::Arbiter::new();
        let (tx, rx) = mpsc::channel();

        let f = || async move {
            let mut page_list        = Vec::new();
            let mut page_id_list     = Vec::new();
            let mut template_id_list = Vec::new();
            let mut action_map      = HashMap::<UDbId, Vec<Action>>::new();
            let mut template_map    = HashMap::<UDbId, Template>::new();

            let mut res_p = Page::fetch(pool, "SELECT * FROM t_page ORDER BY path DESC");
            while let Ok(Some(p)) = res_p.try_next().await {
                page_id_list.push(p.id);
                page_list.push(p);
            }

            if page_id_list.len() > 0 {
                let sql_a = format!(
                    "SELECT * FROM t_action WHERE page IN ({}) ORDER BY page ASC, method ASC",
                    page_id_list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
                );
                let mut res_a = Action::fetch(pool, &sql_a);
                while let Ok(Some(a)) = res_a.try_next().await {
                    template_id_list.extend_from_slice(&a.template.0.id_list());
                    if let Some(list) = action_map.get_mut(&a.page) {
                        list.push(a)
                    } else {
                        action_map.insert(a.page, vec![a]);
                    }
                }

                if template_id_list.len() > 0 {
                    let sql_t = format!(
                        "SELECT * FROM t_template WHERE id IN ({}) ORDER BY id ASC",
                        template_id_list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
                    );
                    let mut res_t = Template::fetch(pool, &sql_t);
                    while let Ok(Some(t)) = res_t.try_next().await {
                        template_map.insert(t.id, t);
                    }
                }
            }
            tx.send((page_list, action_map, template_map)).unwrap();
        };
        arbiter.spawn(f());

        let (page_list, action_map, template_map) = rx.recv().unwrap();
        let mut cache_temporary = RWCache::new();
        cache_temporary.set_template(template_map);

        for page in page_list {
            if let Some(actions) = action_map.get(&page.id) {
                let mut allow_method     = 0;
                let mut allow_origin     = HashSet::new();
                let mut allow_sub_origin = HashSet::new();
                for action in actions {
                    if action.method > 0 {
                        allow_method |= action.method;
                        let method  = Method::from_flag(action.method & !Method::METHOD_TYPE_OPTIONS);
                        let content: Vec<Mime> = action.content.iter().map(
                                |x| Mime::from_str(x)
                            ).filter(
                                |x| x.is_ok()
                            ).map(
                                |x| x.unwrap()
                            ).collect();
                        let origin: Vec<String> = action.origin.iter().filter_map(
                            |origin| if !origin.starts_with("*") {
                                if action.method & Method::METHOD_TYPE_OPTIONS > 0 {
                                    allow_origin.insert(origin.to_string());
                                }
                                Some(origin.to_string())
                            } else {
                                None
                            }
                        ).collect();
                        let sub_origin: Vec<String> = action.origin.iter().filter_map(
                            |origin| origin.strip_prefix("*").map(
                                |s| {
                                    if action.method & Method::METHOD_TYPE_OPTIONS > 0 {
                                        allow_sub_origin.insert(s.to_string());
                                    }
                                    s.to_string()
                                }
                            )
                        ).collect();

                        let route = web::route().guard(
                            guard::fn_guard(
                                move |ctx| {
                                    let head = ctx.head();
                                    let headers = &head.headers();
                                    method.contains(&Method::from(&head.method)) //許可されたメソッド
                                    && (
                                        &head.method == &http::Method::GET
                                        || if let Some(content_type) = headers.get(http::header::CONTENT_TYPE) {
                                            if let Ok(content_type) = Mime::from_str(content_type.to_str().unwrap_or("")) {
                                                content.iter().find(
                                                    |x| (x.type_() == mime::STAR || x.type_() == content_type.type_())
                                                    && (x.subtype() == mime::STAR || x.subtype() == content_type.subtype())
                                                ).is_some()
                                            } else {
                                                false
                                            }
                                        } else {
                                            //空の場合に何も投げられないことがある
                                            if let Some(content_length) = headers.get(http::header::CONTENT_LENGTH) {
                                                content_length.to_str().unwrap_or("") == "0"
                                            } else {
                                                false
                                            }
                                        }
                                    )
                                    && (
                                        //ヘッダから取得できなければguardは通した後でRequestで処理
                                        if let Some(authority) = &head.uri.authority() {
                                            let authority = authority.to_string();
                                            origin.iter().filter(
                                                |o| authority.ends_with(&format!("://{}",o.as_str()))
                                            ).collect::<Vec<&String>>().len() > 0
                                            || sub_origin.iter().filter(
                                                |o| authority.ends_with(o.as_str())
                                            ).collect::<Vec<&String>>().len() > 0
                                        } else {
                                            if let Some(host) = headers.get(http::header::HOST) {
                                                let host = host.to_str().unwrap_or("").to_string();
                                                origin.iter().filter(
                                                    |o| host.ends_with(&format!("://{}",o.as_str()))
                                                ).collect::<Vec<&String>>().len() > 0
                                                || sub_origin.iter().filter(
                                                    |o| host.ends_with(o.as_str())
                                                ).collect::<Vec<&String>>().len() > 0
                                            } else {
                                                false
                                            }
                                        }
                                    )
                                }
                            )
                        );

                        let page_id = page.id;
                        let action_id = action.id;
                        let format = DataFormat::from_flag(action.format);
                        
                        cache_temporary.insert_auth(
                            &action_id,
                            Auth::new(Auth::AUTH_TYPE_GROUP, action.auth.clone())
                        );
                        if format.contains(&DataFormat::STATIC) {
                            cfg.route(
                                &page.path.clone(),
                                route.to(
                                    move |
                                        _db: web::Data<Option<&'static PgPool>>,
                                        _cache: web::Data<Option<&'static RWCache>>,
                                        _path: web::Path<Vec<String>>,
                                        request: HttpRequest,
                                        _session: Session,
                                        //_body: RequestBody,
                                    | {
                                        RWIndex::index_file(
                                            request,
                                            async {
                                                //Err(ServiceError::HTTPRequestNotFound)
                                                Ok(RWAuth::new(vec![]))
                                            }
                                        )
                                    }
                                )
                            );
                        } else {
                            cache_temporary.insert_tree(&action_id, action.template.0.clone());
                            cfg.route(
                                &page.path.clone(),
                                rw_index_closure_routing!((route, page_id, action_id), typing)
                            );
                        }
                    }
                }

                // preflight
                if allow_method & Method::METHOD_TYPE_OPTIONS > 0 {
                    cfg.route(
                        &page.path.clone(),
                        web::method(http::Method::OPTIONS).to(
                            move |
                                request: HttpRequest,
                            | {
                                let mut response = HttpResponse::NoContent();
                                let mut is_origin_ok = false;
                                let mut is_method_ok = false;

                                let head = request.head();
                                let headers = &head.headers();

                                if let Some(cors_origin) = headers.get(http::header::ORIGIN) {
                                    is_origin_ok = allow_origin.contains(cors_origin.to_str().unwrap_or(""))
                                        || allow_sub_origin.iter().filter(
                                            |o| cors_origin.to_str().unwrap_or("").ends_with(o.as_str())
                                        ).collect::<Vec<&String>>().len() > 0;
                                    if is_origin_ok {
                                        response.insert_header(
                                            (
                                                http::header::ORIGIN,
                                                cors_origin
                                            )
                                        );
                                    }
                                }

                                if let Some(cors_method) = headers.get(http::header::ACCESS_CONTROL_REQUEST_METHOD) {
                                    let method = Method::from_flag(allow_method & !Method::METHOD_TYPE_OPTIONS);
                                    if let Ok(m) = http::Method::from_bytes(cors_method.to_str().unwrap_or("").as_bytes()){
                                        is_method_ok = method.contains(&Method::from(&m));
                                        if is_method_ok {
                                            response.insert_header(
                                                (
                                                    http::header::ACCESS_CONTROL_REQUEST_METHOD,
                                                    method.iter().map(|m| http::Method::from(*m).to_string()).collect::<Vec<String>>().join(",")
                                                )
                                            );
                                        }
                                    }
                                }

                                if is_origin_ok && is_method_ok {
                                    response
                                } else {
                                    HttpResponse::BadRequest()
                                }
                            }
                        )
                    );
                }

            }
        }
        let _ = cache.set(cache_temporary);
    }
}
