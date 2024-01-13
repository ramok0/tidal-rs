use std::time::Instant;

use tidal_rs::{*, client::TidalApi, model::Authorization};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    authorization:Option<Authorization>
}

impl Configuration {
    pub fn new() -> Self {
        let file = std::fs::File::open("config.json");
        if let Ok(file) = file {
            let reader = std::io::BufReader::new(file);
            let config = serde_json::from_reader(reader);
            if let Ok(config) = config {
                return config;
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }

    pub fn flush(&self) -> () {
        let file = std::fs::File::create("config.json");
        if let Ok(file) = file {
            let writer = std::io::BufWriter::new(file);
            let _ = serde_json::to_writer(writer, self);
        }
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        self.flush();
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self { authorization: None}
    }
}

async fn auth(client:&TidalApi, authotization:Option<&Authorization>) -> Option<Authorization> {
    if authotization.is_some() {
        let authorization_zebi = authotization.clone().unwrap();

        if let Some(refresh_token) = &authorization_zebi.refresh_token {
            let authorization = client.auth().login_from_refresh_token(&refresh_token).await;
            if let Ok(authorization) = authorization {
                return Some(authorization);
            }
        }
    
        if let Some(access_token) = &authorization_zebi.access_token {
            if let Ok(working) = client.auth().verify_access_token(&access_token).await {
                if working {
                    return authotization.cloned();
                }
            }
        }
    }
  

    let device_code_opt = client.auth().get_device_code().await.ok();
    let created_at = Instant::now();

    let device_code = device_code_opt?;
    println!("Please go to {}", device_code.verification_uri_complete);

    while created_at.elapsed().as_secs() <= device_code.expires_in {
        let authorization = client.auth().login_from_device_code(&device_code.device_code).await;
        if let Ok(authorization) = authorization {
            return Some(authorization);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(device_code.interval)).await;
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut configuration = Configuration::new();
    let mut client = TidalApi::new()?; 

    let authorization = auth(&client, configuration.authorization.as_ref()).await;
    if authorization.is_none() {
        return Err("Unable to authenticate".into());
    }

    if configuration.authorization.is_none() {
        configuration.authorization = authorization.clone();
    }

    client.set_authorization(authorization);

    assert_eq!(client.auth().verify_access_token(client.access_token().unwrap()).await?, true);

    //let media = client.media().get_track("302246288").await;
    let media = client.media().get_stream_url(302246288, model::AudioQuality::Max).await?;
    

    Ok(())      
}
