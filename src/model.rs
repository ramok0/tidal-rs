//This files contains the structs of Tidal's API.

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAuth {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: i64,
    pub interval: i64,
}
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    user_id:u64,
    email:String,
    username:String,
    created:u64,
    updated:u64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Authorization {
    pub access_token:String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in:u64,
    pub user:User
}

struct PlaybackManifest  {

}

struct Artist {

}

struct Album {

}

struct Track {

}