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

    let bytes = STANDARD_NO_PAD.decode(parts[1])
        .map_err(DecodeError::InvalidBase64Encoding)?;


    let text = String::from_utf8(bytes)
        .map_err(DecodeError::InvalidUnicodeEncoding)?;

    decode_payload_text(&text)
}

fn decode_payload_text(text: &str) -> Result<()> {
    let json = serde_json::from_str::<serde_json::Value>(text)
        .map_err(DecodeError::InvalidJson)?;

    let claims = json.as_object()
        .ok_or(DecodeError::NotJsonObject)?;

    print_all_claims(claims);
    Ok(())
}

fn print_all_claims(claims: &serde_json::Map<String, serde_json::Value>) {
    for (key, value) in claims {
        println!("{}: {}", key, value)
    }
}
