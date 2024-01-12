use std::{sync::Arc, collections::HashMap};
use crate::{*, client::*, error::Error, model::*};


pub struct AuthClient {
    client:Arc<ClientImpl>
}

impl AuthClient {
    pub fn new(client:Arc<ClientImpl>) -> Self {
        Self { client }
    }

    pub async fn verify_access_token(&self, access_token:&str) -> Result<bool, Error>
    {
        let req = self
        .client.http_client()
        .get("https://api.tidal.com/v1/sessions")
        .bearer_auth(access_token)
        .send()
        .await.map_err(|e| Error::Reqwest(e))?;
    Ok(req.status().is_success())
    }

    pub async fn get_device_code(&self) -> Result<DeviceAuth, Error> {
        let mut body = HashMap::new();

        body.insert("client_id", CLIENT_ID);
        body.insert("scope", "r_usr+w_usr+w_sub");

        let url = format!("{}/device_authorization", &AUTH_BASE);

        let req = self
            .client.http_client()
            .post(url)
            .form(&body)
            .send()
            .await.map_err(|e| Error::Reqwest(e))?;

        if !req.status().is_success() {
            return Err(Error::FailedToGetDeviceCode);
        }

        let device_auth = req.json::<DeviceAuth>().await.map_err(|_| Error::ParseError)?;
        Ok(device_auth)
    }

    pub async fn login_from_device_code(&self, device_code: &str) -> Result<Authorization, Error> {
        let mut body = HashMap::new();

        body.insert("client_id", CLIENT_ID);
        body.insert("scope", "r_usr+w_usr+w_sub");
        body.insert("device_code", device_code);
        body.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");

        let url = format!("{}/token", &AUTH_BASE);

        let req = self
            .client.http_client()
            .post(url)
            .form(&body)
            .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
            .send()
            .await.map_err(|e| Error::Reqwest(e))?;
        
        if !req.status().is_success() {
            if req.status().is_client_error() {
                return Err(Error::WaitingForUserAction);
            } else {
                return Err(Error::FailedToAuthenticate)
            }
        }

        let authorization = req.json::<Authorization>().await.map_err(|_| Error::ParseError)?;
        Ok(authorization)
    }

    pub async fn login_from_refresh_token(&self, refresh_token: &str) -> Result<Authorization, Error> {
        let mut body = HashMap::new();

        body.insert("client_id", CLIENT_ID);
        body.insert("client_secret", CLIENT_SECRET);
        body.insert("grant_type", "refresh_token");
        body.insert("refresh_token", refresh_token);

        let url = format!("{}/token", &AUTH_BASE);

        let req = self
            .client.http_client()
            .post(url)
            .form(&body)
            .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
            .send()
            .await.map_err(|e| Error::Reqwest(e))?;

        if !req.status().is_success() {
            return Err(Error::InvalidRefreshToken);
        }
        

       let authorization = req.json::<Authorization>().await.map_err(|_| Error::ParseError)?;
       Ok(authorization)
    }
}