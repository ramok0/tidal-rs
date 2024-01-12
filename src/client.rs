use std::{sync::Arc, ops::Deref, collections::HashMap};
use serde::de::DeserializeOwned;

use crate::USER_AGENT;

use super::{*};


pub struct TidalApi(Arc<ClientImpl>);

impl Deref for TidalApi {
    type Target = ClientImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TidalApi {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(Arc::new(ClientImpl::new()?)))
    }

    pub fn auth(&self) -> AuthClient {
        AuthClient::new(self.0.clone())
    }
}

pub struct AuthClient {
    client:Arc<ClientImpl>
}

impl AuthClient {
    pub fn new(client:Arc<ClientImpl>) -> Self {
        Self { client }
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

pub struct ClientImpl {
    http_client: reqwest::Client,
    authorization: Option<Authorization>
}

impl ClientImpl {
    fn new() -> Result<Self, super::error::Error> {
        let client = reqwest::ClientBuilder::new().user_agent(USER_AGENT).build().map_err(|e| Error::Reqwest(e))?;

        Ok(Self { http_client: client, authorization: None })
    }

    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    async fn get<'a, T>(&self, url: &'a str, query: Option<&[(String, String)]>, country_code:String) -> Result<T, Error>
    where
        T: DeserializeOwned + 'a,
    {
        let authorization = self.authorization().ok_or(Error::Unauthorized)?;
        let country_param = ("countryCode".to_owned(), country_code);
        let mut params = Vec::new();
        if let Some(query) = query {
            params.extend(query);
        }
        params.push(&country_param);
        let req = self
            .http_client
            .get(url)
            .bearer_auth(&authorization.access_token)
            .query(&params);

        let result = req.send().await.map_err(|e| Error::Reqwest(e))?.text().await.map_err(|_| Error::ParseError)?;

        let result = serde_json::from_str::<T>(&result).map_err(|_| Error::ParseError)?;
        Ok(result)
    }

    pub fn set_authorization(&mut self, authorization:Option<Authorization>) {
        self.authorization = authorization;
    }

    pub fn authorization(&self) -> Option<&Authorization> {
        self.authorization.as_ref()
    }
}