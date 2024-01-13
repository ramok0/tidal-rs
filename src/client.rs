use std::{sync::Arc, ops::{Deref, DerefMut}};
use serde::de::DeserializeOwned;
use crate::{USER_AGENT, auth::AuthClient, media::MediaClient, user::UserClient};

use super::{*};


pub struct TidalApi(Arc<ClientImpl>);

impl Deref for TidalApi {
    type Target = ClientImpl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TidalApi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.0).unwrap()
    }
}

impl TidalApi {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(Arc::new(ClientImpl::new()?)))
    }

    pub fn auth(&self) -> AuthClient {
        AuthClient::new(self.0.clone())
    }

    pub fn media(&self) -> MediaClient {
        MediaClient::new(self.0.clone())
    }

    pub fn user(&self) -> UserClient {
        UserClient::new(self.0.clone())
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

    pub fn access_token(&self) -> Option<&str> {
        self.authorization.as_ref().and_then(|a| a.access_token.as_ref().map(|s| s.as_str()))
    }

    pub async fn get<'a, T>(&self, url: &'a str, query: Option<&[(String, String)]>, country_code:String) -> Result<T, Error>
    where
        T: DeserializeOwned + std::fmt::Debug + 'a,
    {
        let authorization = self.authorization().ok_or(Error::Unauthorized)?;

        if authorization.access_token.is_none() {
            return Err(Error::Unauthorized);
        }

        let country_param = ("countryCode".to_owned(), country_code);
        let mut params = Vec::new();
        if let Some(query) = query {
            params.extend(query);
        }
        params.push(&country_param);
        let req = self
            .http_client
            .get(url)
            .bearer_auth(&authorization.access_token.as_ref().unwrap())
            .query(&params);

        let result = req.send().await.map_err(|e| Error::Reqwest(e))?.text().await.map_err(|_| Error::ParseError)?;
    //    println!("Result : {}", result);
        let result = serde_json::from_str::<T>(&result);
     //   dbg!(&result);
        Ok(result.map_err(|_| Error::ParseError)?)
    }

    pub fn set_authorization(&mut self, authorization:Option<Authorization>) {
        self.authorization = authorization;
    }

    pub fn authorization(&self) -> Option<&Authorization> {
        self.authorization.as_ref()
    }

    pub fn country_code(&self) -> String {
        self.authorization.as_ref().map(|a| a.user.country_code.clone()).unwrap_or_else(|| "US".to_owned())
    }
}