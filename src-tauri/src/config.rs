use std::{fmt::Display, path::Path};

use base64::Engine;
use lettre::{transport::smtp::authentication::Credentials, Tokio1Executor};
use serde::{Deserialize, Serialize};

use crate::error::ServiceBooksError;

#[derive(Debug, Default, Clone)]
pub(crate) struct SessionConfig {
    pub smtp_url: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl TryInto<lettre::AsyncSmtpTransport<Tokio1Executor>> for SessionConfig {
    type Error = lettre::transport::smtp::Error;
    fn try_into(self) -> Result<lettre::AsyncSmtpTransport<Tokio1Executor>, Self::Error> {
        let Self {
            smtp_url,
            smtp_username,
            smtp_password,
            ..
        } = self;
        let creds = Credentials::new(smtp_username, smtp_password);
        Ok(
            lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_url)?
                .credentials(creds)
                .build(),
        )
    }
}

impl SessionConfig {
    pub(crate) async fn save(&self, path: impl AsRef<Path>) -> tokio::io::Result<()> {
        let bytes = self.to_string();
        tokio::fs::write(path, bytes).await
    }

    pub(crate) async fn load(path: impl AsRef<Path>) -> Result<Self, ServiceBooksError> {
        let bytes = tokio::fs::read(path).await?;
        let config = serde_json::from_slice(&bytes)?;
        Ok(config)
    }
}

impl Display for SessionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Could not serialize sessions config")
        )
    }
}

impl Serialize for SessionConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = [
            self.smtp_url.clone(),
            self.smtp_port.clone(),
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        ]
        .join("|");
        let data = base64::engine::general_purpose::STANDARD.encode(&data);
        serializer.serialize_str(&data)
    }
}

impl<'de> Deserialize<'de> for SessionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let encoded_data = String::deserialize(deserializer)?;

        let decoded_data = base64::engine::general_purpose::STANDARD
            .decode(encoded_data.as_bytes())
            .map_err(serde::de::Error::custom)?;

        let data = String::from_utf8(decoded_data).map_err(serde::de::Error::custom)?;

        let split: Vec<&str> = data.split("|").collect();

        if split.len() != 4 {
            return Err(serde::de::Error::custom("invalid number of fields"));
        }

        let smtp_url = split[0].to_string();
        let smtp_port = split[1].to_string();
        let smtp_username = split[2].to_string();
        let smtp_password = split[3].to_string();

        Ok(SessionConfig {
            smtp_url,
            smtp_port,
            smtp_username,
            smtp_password,
        })
    }
}
