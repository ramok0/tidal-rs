use std::sync::Arc;
use crate::{*, client::*, error::Error, model::*};

pub struct SearchClient {
    client: Arc<ClientImpl>
}

impl SearchClient {
    pub fn new(client: Arc<ClientImpl>) -> Self {
        Self { client }
    }

    pub async fn all(&self, query:&str, _max:Option<usize>) -> Result<SearchResult, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/search", API_BASE);
        let query = &[
            ("query".to_string(), query.to_string())
        ];

        let res = self.client.get::<SearchResponse>(&url, Some(query), self.client.country_code()).await?;

        Ok(SearchResult::from(res))
    }

    pub async fn artist(&self, query:&str, max:Option<usize>) -> Result<Vec<Artist>, Error> 
    {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/search/artists", API_BASE);
        let query = &[
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), max.unwrap_or(50).to_string())
        ];

        let res = self.client.get_items::<Artist>(&url, Some(query.to_vec()), max).await?;
        Ok(res)
    }

    pub async fn track(&self, query:&str, max:Option<usize>) -> Result<Vec<Track>, Error> 
    {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/search/tracks", API_BASE);
        let query = &[
            ("query".to_string(), query.to_string())
        ];

        let res = self.client.get_items::<Track>(&url, Some(query.to_vec()), max).await?;
        Ok(res)
    }

    pub async fn album(&self, query:&str, max:Option<usize>) -> Result<Vec<Album>, Error> 
    {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/search/albums", API_BASE);
        let query = &[
            ("query".to_string(), query.to_string())
        ];

        let res = self.client.get_items::<Album>(&url, Some(query.to_vec()), max).await?;
        Ok(res)
    }

    pub async fn playlist(&self, _query:&str, _max:Option<usize>) -> Result<(), Error> 
    {
        // if self.client.authorization().is_none() {
        //     return Err(Error::Unauthorized);
        // }

        // let url = format!("{}/search/playlists", API_BASE);
        // let query = &[
        //     ("query".to_string(), query.to_string())
        // ];

        todo!("Implement playlists.")
    }
}