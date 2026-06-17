use encoding_rs::{Encoding, GBK, SHIFT_JIS, UTF_8};

use crate::error::RzipError;

#[derive(Clone, Copy, Debug)]
pub enum EncodingKind {
    Utf8,
    Gbk,
    ShiftJis,
}

impl EncodingKind {
    pub fn from_label(value: &str) -> Result<Self, RzipError> {
        let normalized = value.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "utf8" | "utf-8" | "unicode" => Ok(Self::Utf8),
            "gbk" | "cp936" | "936" => Ok(Self::Gbk),
            "shift_jis" | "shift-jis" | "sjis" | "cp932" => Ok(Self::ShiftJis),
            _ => Err(RzipError::UnsupportedEncoding(value.to_string())),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Gbk => "gbk",
            Self::ShiftJis => "shift_jis",
        }
    }

    fn encoder(self) -> &'static Encoding {
        match self {
            Self::Utf8 => UTF_8,
            Self::Gbk => GBK,
            Self::ShiftJis => SHIFT_JIS,
        }
    }

    pub fn decode(self, input: &[u8]) -> Result<String, RzipError> {
        let (decoded, _, had_errors) = self.encoder().decode(input);
        if had_errors {
            return Err(RzipError::DecodeEntryName {
                encoding: self.label(),
            });
        }
        Ok(decoded.into_owned())
    }

    pub fn encode(self, input: &str) -> Result<Vec<u8>, RzipError> {
        let (encoded, _, had_errors) = self.encoder().encode(input);
        if had_errors {
            return Err(RzipError::EncodePath {
                path: input.to_string(),
                encoding: self.label(),
            });
        }
        Ok(encoded.into_owned())
    }
}
