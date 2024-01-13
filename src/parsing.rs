use serde::{Serialize, Deserializer, Deserialize, de::Visitor};

use crate::model::{AudioQuality, AudioMode, PlaybackMode, AssetPresentation, EncryptionType, MimeType};

struct AudioQualityVisitor;
struct AudioModeVisitor;
struct PlaybackModeVisitor;
struct AssetPresentationVisitor;
struct EncryptionTypeVisitor;
struct MimeTypeVisitor;

impl<'de> Visitor<'de> for AudioQualityVisitor {
    type Value = AudioQuality;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing an audio quality")?;

        Ok(())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                println!("VISIT : {}", v);
        match v.as_str() {
            "LOW" => Ok(AudioQuality::Low),
            "HIGH" => Ok(AudioQuality::High),
            "LOSSLESS" => Ok(AudioQuality::Lossless),
            "HI_RES" => Ok(AudioQuality::Max),
            _ => Err(E::unknown_field(v.as_str(), &["LOW", "HIGH", "LOSSLESS", "HI_RES"])),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "LOW" => Ok(AudioQuality::Low),
            "HIGH" => Ok(AudioQuality::High),
            "LOSSLESS" => Ok(AudioQuality::Lossless),
            "HI_RES" => Ok(AudioQuality::Max),
            _ => Err(E::unknown_field(v, &["LOW", "HIGH", "LOSSLESS", "HI_RES"])),
        }
    }
}

impl<'de> Deserialize<'de> for AudioQuality {
    fn deserialize<D>(deserializer: D) -> Result<AudioQuality, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(AudioQualityVisitor)
    }
}

impl Serialize for AudioQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
        serializer.serialize_str(self.to_string().as_str())
    }
}


impl<'de> Visitor<'de> for AudioModeVisitor {
    type Value = AudioMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing an audio mode")?;

        Ok(())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "STEREO" => Ok(AudioMode::Stereo),
            "DOLBY_ATMOS" => Ok(AudioMode::DolbyAtmos),
            "SONY_360RA" => Ok(AudioMode::Sony360RA),
            _ => Err(E::unknown_field(v.as_str(), &["STEREO", "DOLBY_ATMOS", "SONY_360RA"])),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "STEREO" => Ok(AudioMode::Stereo),
            "DOLBY_ATMOS" => Ok(AudioMode::DolbyAtmos),
            "SONY_360RA" => Ok(AudioMode::Sony360RA),
            _ => Err(E::unknown_field(v, &["STEREO", "DOLBY_ATMOS", "SONY_360RA"])),
        }
    }
}

impl<'de> Deserialize<'de> for AudioMode {
    fn deserialize<D>(deserializer: D) -> Result<AudioMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(AudioModeVisitor)
    }
}

impl Serialize for AudioMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
        match self {
            AudioMode::Stereo => serializer.serialize_str("STEREO"),
            AudioMode::DolbyAtmos => serializer.serialize_str("DOLBY_ATMOS"),
            AudioMode::Sony360RA => serializer.serialize_str("SONY_360RA"),
        }
    }
}

impl<'de> Visitor<'de> for PlaybackModeVisitor {
    type Value = PlaybackMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing a playback mode")?;

        Ok(())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "STREAM" => Ok(PlaybackMode::Stream),
            "OFFLINE" => Ok(PlaybackMode::Offline),
            _ => Err(E::unknown_field(v.as_str(), &["STREAM", "OFFLINE"])),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "STREAM" => Ok(PlaybackMode::Stream),
            "OFFLINE" => Ok(PlaybackMode::Offline),
            _ => Err(E::unknown_field(v, &["STREAM", "OFFLINE"])),
        }
    }
}

impl<'de> Deserialize<'de> for PlaybackMode {
    fn deserialize<D>(deserializer: D) -> Result<PlaybackMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PlaybackModeVisitor)
    }
}

impl Serialize for PlaybackMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
            serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Visitor<'de> for AssetPresentationVisitor {
    type Value = AssetPresentation;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing an asset presentation")?;

        Ok(())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "FULL" => Ok(AssetPresentation::Full),
            "PREVIEW" => Ok(AssetPresentation::Preview),
            _ => Err(E::unknown_field(v, &["FULL", "PREVIEW"])),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "FULL" => Ok(AssetPresentation::Full),
            "PREVIEW" => Ok(AssetPresentation::Preview),
            _ => Err(E::unknown_field(v.as_str(), &["FULL", "PREVIEW"])),
        }
    }
}

impl<'de> Deserialize<'de> for AssetPresentation {
    fn deserialize<D>(deserializer: D) -> Result<AssetPresentation, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(AssetPresentationVisitor)
    }
}

impl Serialize for AssetPresentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
            serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Visitor<'de> for EncryptionTypeVisitor {
    type Value = EncryptionType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing an encryption type")?;

        Ok(())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "NONE" => Ok(EncryptionType::None),
            _ => Err(E::unknown_field(v.as_str(), &["NONE"])),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "NONE" => Ok(EncryptionType::None),
            _ => Err(E::unknown_field(v, &["NONE"])),
        }
    }
}

impl<'de> Deserialize<'de> for EncryptionType {
    fn deserialize<D>(deserializer: D) -> Result<EncryptionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(EncryptionTypeVisitor)
    }
}

impl Serialize for EncryptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
        match self {
            EncryptionType::None => serializer.serialize_str("NONE"),
        }
    }
}

impl<'de> Visitor<'de> for MimeTypeVisitor {
    type Value = MimeType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing a mime type")?;

        Ok(())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v.as_str() {
            "audio/mp4" => Ok(MimeType::M4A),
            "audio/flac" => Ok(MimeType::Flac),
            _ => Err(E::unknown_field(v.as_str(), &["audio/mp4", "audio/flac"])),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "audio/mp4" => Ok(MimeType::M4A),
            "audio/flac" => Ok(MimeType::Flac),
            _ => Err(E::unknown_field(v, &["audio/mp4", "audio/flac"])),
        }
    }
}

impl<'de> Deserialize<'de> for MimeType {
    fn deserialize<D>(deserializer: D) -> Result<MimeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(MimeTypeVisitor)
    }
}

impl Serialize for MimeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
        match self {
            MimeType::M4A => serializer.serialize_str("audio/mp4"),
            MimeType::Flac => serializer.serialize_str("audio/flac"),
        }
    }
}