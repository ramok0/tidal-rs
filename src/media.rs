use std::{sync::Arc, str::FromStr};
use crate::{*, client::*, error::Error, model::*};


pub struct MediaClient {
    client: Arc<ClientImpl>
}

impl MediaClient {
    pub fn new(client: Arc<ClientImpl>) -> Self {
        Self { client }
    }

    pub async fn get_artist_albums(&self, id: usize, max:Option<usize>) -> Result<Vec<Album>, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/artists/{}/albums", API_BASE, id);
        let item = self.client.get_items::<Album>(&url, None, max).await?;

        Ok(item)
    }
    
    pub async fn get_artist_singles(&self, id:usize, max:Option<usize>) -> Result<Vec<Album>, Error> 
    {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/artists/{}/albums", API_BASE, id);

        let opts = &[
            ("filter".to_string(), "EPSANDSINGLES".to_string())
        ];

        let item = self.client.get_items::<Album>(&url, Some(opts.to_vec()), max).await?;

        Ok(item)
    }

    pub async fn get_track(&self, id: usize) -> Result<Track, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/tracks/{}", &API_BASE, id);
        self.client.get::<Track>(&url, None, self.client.country_code()).await
    }

    pub async fn get_highest_quality_avaliable_stream_url(&self, id: usize, user_audio_quality:AudioQuality) -> Result<PlaybackManifest, Error> {
        let track = self.get_track(id).await?;
        let mut download_quality = user_audio_quality;

        if download_quality > track.audio_quality {
            download_quality = track.audio_quality;
        }

        self.get_stream_url(id, download_quality).await
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

    pub async fn get_album(&self, id:usize) -> Result<Album, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/albums/{}", API_BASE, id);
        let res = self.client.get::<Album>(&url, None, self.client.country_code()).await?;
        Ok(res)
    }

    pub async fn get_album_tracks(&self, id:usize, max:Option<usize>) -> Result<Vec<Track>, Error> {
        if self.client.authorization().is_none() {
            return Err(Error::Unauthorized);
        }

        let url = format!("{}/albums/{}/items", API_BASE, id);
        let item = self.client.get_items::<ItemResponseItem<Track>>(&url, None, max).await?;
        let result: Vec<Track> = item.into_iter().map(|i| i.item).collect();

        Ok(result)
    }
}