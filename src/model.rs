//This files contains the structs of Tidal's API.

use std::{str::FromStr, sync::Arc};
use base64::{ engine::general_purpose::STANDARD, Engine as _ };
use serde::{ Deserialize, Serialize };

use crate::error::Error;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuth {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinUser {
    #[serde(rename = "userId")]
    pub user_id: usize,
    pub email: String,
    pub username: String,
    pub created: u64,
    pub  updated: u64,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    id: usize,
    username: String,
    #[serde(rename = "profileName")]
    profile_name: Option<String>,
    #[serde(rename = "profileName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
    email: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    created: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Authorization {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: u64,
    pub user: MinUser,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PlaybackInfoPostPaywallRes {
    pub track_id: u64,
    pub asset_presentation: AssetPresentation,
    pub audio_quality: AudioQuality,
    pub manifest_mime_type: String,
    pub manifest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MimeType {
    M4A,
    Flac,
}

impl MimeType {
    pub fn get_file_extension(&self) -> &'static str {
        match self {
            MimeType::M4A => "m4a",
            MimeType::Flac => "flac",
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscription {
    pub start_date: String,
    pub valid_until: String,
    pub status: String,
    pub subscription: Subscription,
    pub highest_sound_quality: AudioQuality,
    pub premium_access: bool,
    pub can_get_trial: bool
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    pub offline_grace_period: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubscriptionType {
    Free,
    Premium,
    Hifi
}

impl ToString for SubscriptionType {
    fn to_string(&self) -> String {
        match self {
            SubscriptionType::Free => "FREE".to_string(),
            SubscriptionType::Premium => "PREMIUM".to_string(),
            SubscriptionType::Hifi => "HIFI".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PlaybackManifest {
    #[serde(rename = "mimeType")]
    pub mime_type: MimeType,
    pub  codecs: String,
    #[serde(rename = "encryptionType")]
    pub encryption_type: EncryptionType,
    pub urls: Vec<String>,
}

impl FromStr for PlaybackManifest {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json_bytes = STANDARD.decode(s.as_bytes()).map_err(|e| {
            eprintln!("Error decoding base64: {}", e);
            super::Error::ParseError
        })?;
        let value: Self = serde_json
            ::from_slice(&json_bytes)
            .map_err(|_| super::Error::ParseError)?;

        Ok(value)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub picture: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: usize,
    pub title: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "audioModes")]
    pub audio_modes: Option<Vec<AudioMode>>,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: Option<i64>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<AudioQuality>,
    pub artist: Option<Artist>,
    #[serde(default)]
    pub artists: Vec<Artist>,
}

impl Album {
    pub fn get_artist(&self) -> Artist {
        self.artist.clone().unwrap_or_else(|| self.artists.first().unwrap_or(&Artist { id: 0, name: "Unknown".to_string(), type_field: None, picture: None }).clone())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub enum AudioQuality {
    Low,
    High,
    Lossless,
    Max,
}

impl ToString for AudioQuality {
    fn to_string(&self) -> String {
        match self {
            AudioQuality::Low => "LOW".to_string(),
            AudioQuality::High => "HIGH".to_string(),
            AudioQuality::Lossless => "LOSSLESS".to_string(),
            AudioQuality::Max => "HI_RES".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AudioMode {
    Stereo,
    DolbyAtmos,
    Sony360RA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlaybackMode {
    Stream,
    Offline,
}

impl ToString for PlaybackMode {
    fn to_string(&self) -> String {
        match self {
            PlaybackMode::Stream => "STREAM".to_string(),
            PlaybackMode::Offline => "OFFLINE".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssetPresentation {
    Full,
    Preview,
}

impl ToString for AssetPresentation {
    fn to_string(&self) -> String {
        match self {
            AssetPresentation::Full => "FULL".to_string(),
            AssetPresentation::Preview => "PREVIEW".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EncryptionType {
    None,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Eq, Hash)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub struct Mixes {
    #[serde(rename = "MASTER_TRACK_MIX")]
    pub master_track_mix: Option<String>,
    #[serde(rename = "TRACK_MIX")]
    pub track_mix: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: usize,
    pub title: String,
    pub duration: i64,
    pub mixes: Option<Mixes>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: AudioQuality,
    pub audio_modes: Vec<AudioMode>,
    pub media_metadata: MediaMetadata,
    pub explicit: bool,
    pub track_number: Option<i64>,
    pub album: Option<Album>,
    pub artist: Option<Artist>,
    pub artists: Vec<Artist>,
}

impl Track {
    pub fn get_artist(&self) -> Artist {
        self.artist.clone().unwrap_or_else(|| self.artists.first().unwrap().clone())
    }

    pub fn get_cover_url(&self) -> String {
        if self.album.is_some() {
            format!("{}/{}/80x80.jpg", "https://resources.tidal.com/images", self.album.as_ref().unwrap().cover.replace("-", "/"))
        } else {
            format!("{}/{}/80x80.jpg", "https://resources.tidal.com/images", self.get_artist().picture.as_ref().unwrap().replace("-", "/"))
        }
    }

    pub async fn fetch_cover(&self) -> Result<Vec<u8>, super::Error> {
        let url = self.get_cover_url();
     //   println!("Cover Url : {}", url);
        let bytes = reqwest::get(&url).await.map_err(|e| Error::Reqwest(e))?.bytes().await.map_err(|_| Error::ParseError)?;
      //  fs::write(format!("{}.jpg", self.title), bytes.to_vec());
        Ok(bytes.to_vec())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ItemResponse<T> {
    pub limit: usize,
    pub offset: usize,
    pub total_number_of_items: usize,
    pub items: Vec<T>,
}

impl<T> Default for ItemResponse<T> {
    fn default() -> Self {
        Self {
            limit: 0,
            offset: 0,
            total_number_of_items: 0,
            items: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemResponseItem<T> {
    pub item: T,
    #[serde(alias = "type")]
    pub item_type: String,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SearchType {
    Artist,
    Track,
    Album,
    Playlist
}

impl SearchType {
    pub fn search_types() -> Vec<SearchType> {
        vec![SearchType::Artist, SearchType::Track, SearchType::Album, SearchType::Playlist]
    }
}

impl ToString for SearchType {
    fn to_string(&self) -> String {
        match self {
            SearchType::Artist => "ARTISTS".to_string(),
            SearchType::Track => "TRACKS".to_string(),
            SearchType::Album => "ALBUMS".to_string(),
            SearchType::Playlist => "PLAYLISTS".to_string(),
        }
    }

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SearchResponse {
    pub artists:ItemResponse<Artist>,
    pub tracks:ItemResponse<Track>,
    pub albums:ItemResponse<Album>
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub artists:Vec<Artist>,
    pub tracks:Vec<Track>,
    pub albums:Vec<Album>
}

impl Default for SearchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchResult {
    pub fn new() -> Self {
        Self {
            artists: vec![],
            tracks: vec![],
            albums: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.artists.clear();
        self.tracks.clear();
        self.albums.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.artists.is_empty() && self.tracks.is_empty() && self.albums.is_empty()
    }
}

impl From<SearchResponse> for SearchResult {
    fn from(value: SearchResponse) -> Self {
        Self {
            artists: value.artists.items,
            tracks: value.tracks.items,
            albums: value.albums.items
        }
    }
}