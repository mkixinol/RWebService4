pub mod r#type;
pub mod core;
pub mod module;

use sqlx::{PgPool};
use once_cell::sync::OnceCell;
use actix_web::{App, web, rt, HttpServer, HttpResponse};
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use crate::core::{index::RWRouter, tls::RWTLS, server::*, database::RWDatabase};

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

fn main() {
    let server_builder = move |config: RWServerConfig| -> Result<_, Box<dyn std::error::Error>> {
        let mut db = RWDatabase::new(
            "admin", //self.postgres_user,
            "admin", //self.postgres_password,
            "127.0.0.1", //self.postgres_host,
            "5433", //self.postgres_port,
            "", //self.postgres_database
            20,
        );

        rt::System::new().block_on(db.connect());
        let _ = DB_POOL.set(db.get_pool().unwrap());

        let config2 = config.clone();
        let mut server = HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(DB_POOL.get()))
            .configure(
                |cfg| {
                    RWRouter::route(cfg, DB_POOL.get().unwrap())
                }
            )
            .default_service(web::to(|| HttpResponse::NotFound()))
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    config2.get_cookie_key()
                )
            )
        });

        if let Ok(host) = config.get_host() {
            server = server.bind(host)?;
        }

        if let Ok(host_tls) = config.get_host_tls() {
            server = server.bind_rustls(
                host_tls.0,
                host_tls.1.config().unwrap()
            )?;
        }

        Ok(server.run())
    };
    
    // db接続
    /*
    let database_builder = move || async {
        RWDatabase::connect("".to_string()).await;
        RWDatabase::getRow("SELECT * FROM t_test".to_string()).await;
    };
    */

    let config = RWServerConfig::new(
        Some("127.0.0.1:8080".to_string()),
        Some(
            (
                "127.0.0.1:8443".to_string(), 
                RWTLS::factory(
                    "files/live/xtrap.app/cert.pem",
                    "files/live/xtrap.app/privkey.pem"
                ).unwrap()
            )
        ),
        5
    );

    let mut server = RWServer::new(config);
    server.run(server_builder);
    /*
    rt::System::new().block_on(
        server_builder().unwrap()
    );
    */
}