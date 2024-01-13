use std::sync::Arc;
use crate::{*, client::*, error::Error, model::*};


pub struct UserClient {
    client: Arc<ClientImpl>
}

impl UserClient {
    pub fn new(client: Arc<ClientImpl>) -> Self {
        Self { client }
    }

    pub async fn get_user(&self, id:usize) -> Result<User, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.client.get::<User>(&format!("{}/users/{}", API_BASE, id), None, self.client.country_code()).await?;
        
        Ok(res)
    }

    pub async fn get_user_subscription(&self, id:usize) -> Result<UserSubscription, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.client.get::<UserSubscription>(&format!("{}/users/{}/subscription", API_BASE, id), None, self.client.country_code()).await?;
        
        Ok(res)
    }

    pub async fn get_current_account_highest_sound_quality(&self) -> Result<AudioQuality, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.get_user_subscription(self.client.authorization().unwrap().user.user_id).await?;
        
        Ok(res.highest_sound_quality)
    }
}