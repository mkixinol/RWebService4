use std::sync::mpsc;
use actix_web::{rt, dev, cookie::Key};

use crate::core::tls::RWTLS;

// global vars.
//static SERVER_TX: OnceCell<mpsc::SyncSender<isize>> = OnceCell::new();

pub struct RWServer {
    config: RWServerConfig,
    tx: Option<mpsc::SyncSender<isize>>,
    handle: Option<dev::ServerHandle>
}

#[derive(Clone)]
pub struct RWServerConfig {
    host: Option<String>,
    host_tls: Option<(String, RWTLS)>,
    channel: usize,
    cookie_key: Key,
}

impl RWServer {
    pub fn new(config: RWServerConfig) -> Self {
        Self {
            config: config,
            tx: None,
            handle: None
        }
    }

    pub fn run<F: Clone>(&mut self, builder: F)
    where
        F: Fn(RWServerConfig) -> Result<dev::Server, Box<dyn std::error::Error>> + Send + Clone + 'static
    {
        if self.tx.is_none() {
            let (tx, rx) = mpsc::sync_channel(self.config.get_channel());
            self.tx = Some(tx);

            let mut stop_code: isize = 1;
            while stop_code > 0 {
                self.start(builder.clone());
                stop_code = rx.recv().unwrap();
                println!("Server Stop with: {}", stop_code);
                self.stop();
            }
        }
    }

    pub fn send_stop(&self, code: isize) {
        if self.tx.is_some() {
            let _ = self.tx.as_ref().unwrap().send(code);
        }
    }

    fn start<F: Clone>(&mut self, builder: F) -> bool
    where
        F: Fn(RWServerConfig) -> Result<dev::Server, Box<dyn std::error::Error>> + Send + 'static
    {
        if !self.handle.is_some() {
            let config = self.config.clone();
            let (tx2, rx2) = mpsc::channel();
            std::thread::spawn(move || {
                let system = rt::System::new();
                let server = builder(config);
                tx2.send(server.as_ref().unwrap().handle()).unwrap();
                system.block_on(server.unwrap()).unwrap();
            });
            self.handle = Some(rx2.recv().unwrap());
        }
        self.handle.is_some()
    }

    fn stop(&mut self) {
        if let Some(handle) = &self.handle {
            let system = rt::System::new();
            system.block_on(handle.stop(true));
        }
        self.handle = None;
    }
}


impl RWServerConfig {
    pub fn new(
        host: Option<String>,
        host_tls: Option<(String, RWTLS)>,
        channel: usize
    ) -> Self {
        Self {
            host: host,
            host_tls: host_tls,
            channel: channel,
            cookie_key: Key::generate()
        }
    }

    pub fn get_channel(&self) -> usize {
        self.channel
    }

    pub fn get_host(&self) -> Result<String, ()> {
        if let Some(h) = &self.host {
            Ok(h.to_string())
        } else {
            Err(())
        }
    }

    pub fn get_host_tls(&self) -> Result<(String, RWTLS), ()> {
        if let Some(h) = &self.host_tls {
            Ok((h.0.to_string(),h.1.clone()))
        } else {
            Err(())
        }
    }
    
    pub fn get_cookie_key(&self) -> Key {
        self.cookie_key.clone()
    }
}
