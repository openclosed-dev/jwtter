use std::string::FromUtf8Error;

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

#[derive(Debug)]
pub enum DecodeError {
    MissingPayload,
    InvalidBase64Encoding(base64::DecodeError),
    InvalidUnicodeEncoding(FromUtf8Error),
    InvalidJson(serde_json::error::Error),
    NotJsonObject
}

impl std::fmt::Display for DecodeError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingPayload => {
                write!(f, "Payload part is missing")
            },
            Self::InvalidBase64Encoding(source) => {
                write!(f, "Payload is not encoded properly in Base64: {}", source)
            },
            Self::InvalidUnicodeEncoding(source) => {
                write!(f, "Payload is not encoded properly in UTF-8: {}", source)
            },
            Self::InvalidJson(source) => {
                write!(f, "Payload is not a valid JSON value: {}", source)
            },
            Self::NotJsonObject => {
                write!(f, "Payload is not a JSON object")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

pub type Result<T> = std::result::Result<T, DecodeError>;

pub fn decode_jwt(token: &str) -> Result<()> {
    let parts: Vec<_> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(DecodeError::MissingPayload)
    }

    match STANDARD_NO_PAD.decode(parts[1]) {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(text) => decode_payload_text(&text),
                Err(err) => {
                    Err(DecodeError::InvalidUnicodeEncoding(err)) 
                } 
            }
        },
        Err(err) => {
            Err(DecodeError::InvalidBase64Encoding(err))
        }
    }
}

fn decode_payload_text(text: &str) -> Result<()> {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(value) => {
            print_all_claims(&value)
        },
        Err(err) => {
            Err(DecodeError::InvalidJson(err))  
        }
    }
}

fn print_all_claims(json: &serde_json::Value) -> Result<()> {
    if let Some(claims) = json.as_object() {
        for (key, value) in claims {
            println!("{}: {}", key, value)
        }
        Ok(())
    } else {
        Err(DecodeError::NotJsonObject)
    }
}
