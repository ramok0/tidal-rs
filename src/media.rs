use std::{sync::Arc, str::FromStr};
use crate::{*, client::*, error::Error, model::*};


pub struct MediaClient {
    client: Arc<ClientImpl>
}

impl MediaClient {
    pub fn new(client: Arc<ClientImpl>) -> Self {
        Self { client }
    }

    pub async fn get_track(&self, id: &str) -> Result<Track, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/tracks/{}", &API_BASE, id);
        self.client.get::<Track>(&url, None, self.client.country_code()).await
    }

    pub async fn get_stream_url(&self, id: usize, audio_quality:AudioQuality) -> Result<PlaybackManifest, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/tracks/{}/playbackinfopostpaywall", &API_BASE, id);
        let query = &[
            ("audioquality".to_string(), audio_quality.to_string()),
            ("playbackmode".to_string(), PlaybackMode::Stream.to_string()),
            (
                "assetpresentation".to_string(),
                AssetPresentation::Full.to_string(),
            ),
        ];

        let playback_info_post_paywall_res = self
            .client
            .get::<PlaybackInfoPostPaywallRes>(&url, Some(query), self.client.country_code())
            .await?;

         match playback_info_post_paywall_res.manifest_mime_type.as_str() {
             "application/vnd.tidal.bts" => Ok(PlaybackManifest::from_str(&playback_info_post_paywall_res.manifest)?),
             _ => Err(Error::IncorrectMimeType),
        }
    }

    pub async fn get_mixes_items(&self, mix:&str, max:Option<usize>) -> Result<Vec<Track>, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/mixes/{}/items", API_BASE, mix);
        let item = self.client.get_items::<ItemResponseItem<Track>>(&url, None, max).await?;
        let result: Vec<Track> = item.into_iter().map(|i| i.item).collect();

        Ok(result)
    }
}