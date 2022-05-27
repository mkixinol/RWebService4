use std::sync::mpsc;
use std::collections::{HashMap, HashSet};
use sqlx::{query_as, Postgres, PgPool, types::Json};
use actix_web::{rt, web, http, guard, Error, HttpRequest, HttpResponse, Responder};
use actix_files::NamedFile;
use actix_session::{Session};
use futures_util::TryStreamExt;
use crate::module::{Page, Action, Template, ModuleDbAccess};
use crate::r#type::{UDbId, Method, DataFormat, ContentType, ServiceError};

pub struct RWRouter {}

impl RWRouter {
    pub async fn index(
        /*
        _page_id: UDbId,
        _method: Method,
        _param: Vec<String>,
        */
        request:  HttpRequest,
        session: Session,
        //_body: RequestBody,
    ) -> impl Responder {
        if let Some(count) = session.get::<i32>("counter").unwrap() {
            println!("SESSION value: {}", count);
            // modify the session state
            session.insert("counter", count + 1).unwrap();
        } else {
            session.insert("counter", 1).unwrap();
        }
        println!("index: {:?}", request.path());
        HttpResponse::Ok()
    }

    pub async fn index_file(
        request: HttpRequest, 
        error: Result<(), ServiceError>
    ) -> Result<NamedFile, ServiceError> {

        if let Err(e) = error {
            return Err(e);
        } else {
            // ディレクトリトラバーサル対応
            // 正規化したパスとREQUEST URIが等しければOK
            let home_buf = std::path::PathBuf::from(std::env::current_dir().unwrap());
            let mut path_buf = std::path::PathBuf::from(&home_buf);
            let iter = std::path::Path::new(request.path().trim_start_matches('/')).iter();
            for d in iter {
                // OSによる差分吸収のためdirごとにpush
                path_buf.push(d);
            }
    
            let home = home_buf.as_path();
            let path = path_buf.as_path();
            if let Ok(path_c) = path.canonicalize() {
                let home_c = home.canonicalize().unwrap();
                if path_c.to_str().unwrap().starts_with(&home_c.to_str().unwrap()) 
                && path_c.to_str().unwrap().ends_with(&path.to_str().unwrap()) {
                    return Ok(NamedFile::open(path_c).unwrap());
                }
            }
        }
        
        return Err(ServiceError::HTTPRequestNotFound);
    }

    pub fn route(cfg: &mut web::ServiceConfig, pool: &'static PgPool) {
        let arbiter = actix_rt::Arbiter::new();
        let (tx, rx) = mpsc::channel();

        let f = || async move {
            let mut page_list        = Vec::new();
            let mut page_id_list     = Vec::new();
            let mut template_id_list = Vec::new();
            let mut action_list      = HashMap::<UDbId, Vec<Action>>::new();
            let mut template_list    = HashMap::<UDbId, Template>::new();

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
                    if let Json(template_tree) = &a.template {
                        for template_info in template_tree {
                            template_id_list.extend_from_slice(&template_info.id_list());
                        }
                    }
                    if let Some(list) = action_list.get_mut(&a.page) {
                        list.push(a)
                    } else {
                        action_list.insert(a.page, vec![a]);
                    }
                }

                if template_id_list.len() > 0 {
                    let sql_t = format!(
                        "SELECT * FROM t_template WHERE page IN ({}) ORDER BY page ASC, method ASC",
                        template_id_list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
                    );
                    let mut res_t = Template::fetch(pool, &sql_t);
                    while let Ok(Some(t)) = res_t.try_next().await {
                        template_list.insert(t.id, t);
                    }
                }
            }
        
            tx.send((page_list, action_list, template_list)).unwrap();
        };
        arbiter.spawn(f());

        let (page_list, action_list, template_list) = rx.recv().unwrap();

        for page in page_list {
            if let Some(actions) = action_list.get(&page.id) {
                let mut allow_method     = 0;
                let mut allow_origin     = HashSet::new();
                let mut allow_sub_origin = HashSet::new();
                for action in actions {
                    if action.method > 0 {
                        allow_method |= action.method;
                        let method  = Method::from_flag(action.method & !Method::METHOD_TYPE_OPTIONS);
                        let content: Vec<String> = ContentType::from_flag(action.content).iter().filter_map(
                            |x| String::try_from(*x).ok()
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
                                            content.contains(&content_type.to_str().unwrap_or("").to_string())
                                        } else { false }
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

                        let format  = DataFormat::from_flag(action.format);
                        if format.contains(&DataFormat::STATIC) {
                            cfg.route(
                                &page.path.clone(),
                                route.to(
                                    move |
                                        request: HttpRequest,
                                        path: web::Path<Vec<String>>,
                                        //session: Session,
                                        //body: RequestBody,
                                        _db: web::Data<Option<&PgPool>>
                                    | {
                                        Self::index_file(
                                            request,
                                            (
                                                || {
                                                    //Err(ServiceError::HTTPRequestNotFound)
                                                    Ok(())
                                                }
                                            )()
                                        )
                                    }
                                )
                            );
                        } else {
                            cfg.route(
                                &page.path.clone(),
                                route.to(
                                    move |
                                        request: HttpRequest,
                                        path: web::Path<Vec<String>>,
                                        session: Session,
                                        //body: RequestBody,
                                        _db: web::Data<Option<&PgPool>>
                                    | {
                                        Self::index(request, session)
                                    }
                                )
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
    }
}
