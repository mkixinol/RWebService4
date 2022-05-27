use std::fs::File;
use rustls::{Certificate, PrivateKey};
use rustls::server::ServerConfig;
use rustls_pemfile::{certs, rsa_private_keys};

#[derive(Clone, Debug)]
pub struct RWTLS{
    cert: Vec<Certificate>,
    key: PrivateKey   
}

impl RWTLS {
    pub fn factory(cert_file: &str, key_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cert_file = std::io::BufReader::new(
            File::open(cert_file)?
        );
        let mut key_file  = std::io::BufReader::new(
            File::open(key_file)?
        );
        
        let cert = certs(&mut cert_file)?
        .drain(..)
        .map(Certificate)
        .collect();
        let mut keys = rsa_private_keys(&mut key_file)?;
        let key = keys.drain(..).map(PrivateKey).next().unwrap();

        Ok(
            Self {
                cert: cert,
                key: key
            }
        )
    }

    pub fn config(self) -> Result<ServerConfig, Box<dyn std::error::Error>> {
        // Load key files
        let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(self.cert, self.key)?;

        Ok(config)
    }
}