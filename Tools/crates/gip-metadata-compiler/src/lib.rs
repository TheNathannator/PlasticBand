// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "MetadataHeader")]
    pub header: MetadataHeader,

    #[serde(rename = "DeviceMetadata")]
    pub device_metadata: DeviceMetadata,

    #[serde(rename = "Messages")]
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetadataHeader {
    #[serde(rename = "MajorVersion")]
    pub major_version: u16,

    #[serde(rename = "MinorVersion")]
    pub minor_version: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceMetadata {
    #[serde(rename = "SupportedInSystemCommands")]
    pub in_commands: Vec<u8>,

    #[serde(rename = "SupportedOutSystemCommands")]
    pub out_commands: Vec<u8>,

    #[serde(rename = "SupportedAudioFormats")]
    pub audio_formats: Vec<AudioFormatPair>,

    #[serde(rename = "SupportedDeviceFirmwareVersions")]
    pub firmware_versions: Vec<FirmwareVersion>,

    #[serde(rename = "PreferredTypes")]
    pub preferred_types: Vec<String>,

    #[serde(rename = "SupportedInterfaces")]
    pub interfaces: Vec<Uuid>,

    #[serde(rename = "SupportedHidDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hid_descriptor: Option<HidDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AudioFormatPair {
    #[serde(rename = "Inbound")]
    #[serde(default)]
    pub inbound: Option<AudioFormat>,

    #[serde(rename = "Outbound")]
    #[serde(default)]
    pub outbound: Option<AudioFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AudioFormat {
    #[serde(rename = "Channels")]
    pub channels: i32,

    #[serde(rename = "Rate")]
    pub rate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FirmwareVersion {
    #[serde(rename = "Major")]
    pub major: u16,

    #[serde(rename = "Minor")]
    pub minor: u16,
}

#[derive(Debug)]
pub struct HidDescriptor(pub Vec<u8>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[serde(rename = "MessageType")]
    pub message_id: u8,

    #[serde(rename = "MessageLength")]
    pub max_length: u16,

    #[serde(rename = "DataType")]
    pub message_type: String,

    #[serde(rename = "IsBigEndian")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_big_endian: bool,

    #[serde(rename = "IsReliable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_reliable: bool,

    #[serde(rename = "IsSequenced")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_sequenced: bool,

    #[serde(rename = "IsUpstream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_upstream: bool,

    #[serde(rename = "IsDownstream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub is_downstream: bool,

    #[serde(rename = "IsDownstreamRequestResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub downstream_requests_response: bool,

    #[serde(rename = "Period")]
    pub period: i32,

    #[serde(rename = "PersistenceTimeout")]
    pub persistence_timeout: i32,
}

fn is_false(value: &bool) -> bool {
    !*value
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

impl Serialize for HidDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_str(self)
        } else {
            serializer.serialize_bytes(&self.0)
        }
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
