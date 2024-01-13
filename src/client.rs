use std::{sync::Arc, ops::{Deref, DerefMut}, fmt::Debug};
use serde::de::DeserializeOwned;
use crate::{USER_AGENT, auth::AuthClient, media::MediaClient, user::UserClient, search::SearchClient};

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

    pub fn search(&self) -> SearchClient {
        SearchClient::new(self.0.clone())
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
        if result.is_err() {
            dbg!(&result);
        }
        Ok(result.map_err(|_| Error::ParseError)?)
    }

    pub async fn get_items<'a, T>(
        &self,
        url: &str,
        opts: Option<Vec<(String, String)>>,
        max: Option<usize>,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned + Debug + 'a,
    {
        let country_code = self.country_code();

        let mut limit = 50;
        let mut offset = 0;
        let max = max.unwrap_or(usize::MAX);
        let mut params = vec![("limit".to_string(), limit.to_string())];
        if let Some(opt) = opts {
            params.extend(opt);
        };

        let mut result: Vec<T> = Vec::new();
        'req: loop {
            params.push(("offset".to_string(), offset.to_string()));
            let json = self.get::<ItemResponse<T>>(url, Some(&params), country_code.clone()).await?;

            limit = json.limit;
            // the minimum between the items in the response, and the total number of items requested
            let item_limit = usize::min(json.total_number_of_items, max);
            for item in json.items {
                if result.len() >= item_limit {
                    break 'req;
                }
                result.push(item);
            }
            offset += limit;
            params.pop();
            if offset >= json.total_number_of_items {
                break 'req;
            }
        }
        Ok(result)
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