mod auth;
mod routes;
pub use auth::RWAuth;
pub use routes::RWRouter;

use std::future::Future;
use mime::Mime;
use sqlx::{PgPool};
use actix_web::{web, http, HttpRequest, HttpResponse, Responder};
use actix_files::NamedFile;
use actix_session::{Session};

use crate::core::{cache::RWCache, RequestValueAny};
use crate::EngineValue;
use crate::r#type::{UDbId, ServiceError, ServerError};

pub struct RWIndex();
impl RWIndex {
    pub async fn index<Fut>(
        db: web::Data<Option<&'static PgPool>>,
        cache: web::Data<Option<&'static RWCache>>,
        _page_id: UDbId,
        action_id: UDbId,
        request: HttpRequest,
        session: Session,
        body: RequestValueAny,
        auth: Fut
    ) -> Result<impl Responder, ServiceError>
        where Fut: Future<Output=Result<RWAuth, ServiceError>>
    {
        let _ = auth.await?;
        let mut template_tree = cache.get_ref().unwrap().get_tree(&action_id);
        let mut response = Vec::new();
        while let Some(tree) = template_tree {
            if let Some(id) = tree.id() {
                println!("{}", &id);
                if let Some(template) = cache.get_ref().unwrap().get_template(&id) {
                    if let Ok((status, value)) = template.execute(
                        db.get_ref().unwrap(),
                        cache.get_ref().unwrap(),
                        &tree.option(),
                        &request,
                        &session,
                        //&body,
                        &response
                    ).await {
                        response.push(value);
                        if status {
                            template_tree = tree.success();
                        } else {
                            template_tree = tree.fail();
                        }
                    } else {
                        response.push(EngineValue::None);
                        template_tree = tree.fail();
                    }
                    continue;
                } else {
                    return Err(ServerError::TemplateRoutingError.into());
                }
            }
            break;
        }

        if let Some(count) = session.get::<i32>("counter").unwrap() {
            println!("SESSION value: {}", count);
            // modify the session state
            session.insert("counter", count + 1).unwrap();
        } else {
            session.insert("counter", 1).unwrap();
        }
        println!("index: {:?}", request.path());

        if let Some(response) = response.last() {
            //let content_type = request.headers().get(http::header::CONTENT_TYPE);
            if let Some(accept) = request.headers().get(http::header::ACCEPT) {
                if let Ok(mime) = accept.to_str().unwrap_or("").parse::<Mime>() {
                    match mime.subtype() {
                        mime::JSON => {
                            Ok(HttpResponse::Ok().json(response))
                        },
                        _ => {
                            Ok(HttpResponse::Ok().content_type(accept).finish())
                        }
                    }
                } else {
                    Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).finish())
                }
            } else {
                Ok(HttpResponse::Ok().content_type(mime::TEXT_HTML).finish())
            }
        } else {
            Ok(HttpResponse::NoContent().finish())
        }
    }

    pub async fn index_file<Fut>(
        request: HttpRequest,
        auth: Fut
    ) -> Result<impl Responder, ServiceError>
        where Fut: Future<Output=Result<RWAuth, ServiceError>>
    {
        let _ = auth.await?;

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

        return Err(ServiceError::HTTPRequestNotFound);
    }
}