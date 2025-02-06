// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::ParseIntError;
use std::str::FromStr;

use serde::Deserialize;
use uuid::Uuid;

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub MetadataHeader: MetadataHeader,
    pub DeviceMetadata: DeviceMetadata,
    pub Messages: Vec<Message>,
}

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct MetadataHeader {
    pub MajorVersion: i32,
    pub MinorVersion: i32,
}

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct DeviceMetadata {
    pub SupportedInSystemCommands: Vec<u8>,
    pub SupportedOutSystemCommands: Vec<u8>,
    pub SupportedAudioFormats: Vec<AudioFormatPair>,
    pub SupportedDeviceFirmwareVersions: Vec<FirmwareVersion>,
    pub PreferredTypes: Vec<String>,
    pub SupportedInterfaces: Vec<Uuid>,
    #[serde(default)]
    pub SupportedHidDescriptor: Option<HidDescriptor>,
}

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct AudioFormatPair {
    #[serde(default)]
    pub Inbound: Option<AudioFormat>,
    #[serde(default)]
    pub Outbound: Option<AudioFormat>,
}

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct AudioFormat {
    pub Channels: i32,
    pub Rate: i32,
}

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct FirmwareVersion {
    pub Major: u16,
    pub Minor: u16,
}

#[derive(Debug)]
pub struct HidDescriptor(pub Vec<u8>);

#[allow(non_snake_case, reason = "JSON format uses PascalCase names")]
#[derive(Debug, Deserialize)]
pub struct Message {
    pub MessageType: u8,
    pub MessageLength: u16,
    pub DataType: String,

    #[serde(default)]
    pub IsBigEndian: bool,
    #[serde(default)]
    pub IsReliable: bool,
    #[serde(default)]
    pub IsSequenced: bool,
    #[serde(default)]
    pub IsUpstream: bool,
    #[serde(default)]
    pub IsDownstream: bool,
    #[serde(default)]
    pub IsDownstreamRequestResponse: bool,

    pub Period: i32,
    pub PersistenceTimeout: i32,
}

impl std::fmt::Display for HidDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(f, "{:2X}", self.0[0])?;
        for byte in &self.0[1..] {
            write!(f, " {byte:2X}")?;
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HidDescriptorParseError {
    #[error("a segment is not the right length (!= 2)")]
    InvalidSegmentLength,

    #[error(transparent)]
    ByteParse(#[from] ParseIntError),
}

impl FromStr for HidDescriptor {
    type Err = HidDescriptorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = Vec::new();
        for split in s.split(' ') {
            if split.len() != 2 {
                return Err(HidDescriptorParseError::InvalidSegmentLength);
            }

            bytes.push(u8::from_str_radix(split, 16)?);
        }

        Ok(Self(bytes))
    }
}

impl<'de> Deserialize<'de> for HidDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(HidDescriptorVisitor)
        } else {
            deserializer.deserialize_byte_buf(HidDescriptorVisitor)
        }
    }
}

struct HidDescriptorVisitor;

impl serde::de::Visitor<'_> for HidDescriptorVisitor {
    type Value = HidDescriptor;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a space-separated set of hexadecimal byte values, without a 0x prefix")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<HidDescriptor>().map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(HidDescriptor(v.to_owned()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(HidDescriptor(v))
    }
}
