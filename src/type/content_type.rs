use std::convert::TryFrom;
use crate::r#type::{UDbFlg, UDbFlgBit};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ContentType(pub UDbFlgBit);

impl ContentType {
    pub const TYPE_APPLICATION_JSON:                UDbFlgBit = 1 << 0;
    pub const TYPE_APPLICATION_WWW_FORM_URLENCODED: UDbFlgBit = 1 << 1;
    pub const TYPE_MULTIPART_FORM_DATA:             UDbFlgBit = 1 << 2;

    pub const JSON: Self = Self(Self::TYPE_APPLICATION_JSON);
    pub const URL:  Self = Self(Self::TYPE_APPLICATION_WWW_FORM_URLENCODED);
    pub const FORM: Self = Self(Self::TYPE_MULTIPART_FORM_DATA);

    pub const FLAGS: [UDbFlgBit; 3] = [
        Self::TYPE_APPLICATION_JSON,
        Self::TYPE_APPLICATION_WWW_FORM_URLENCODED,
        Self::TYPE_MULTIPART_FORM_DATA,
    ];

    pub fn from_flag(flag: UDbFlg) -> Vec<Self> {
        let mut contents = vec![];
        for c in Self::FLAGS {
            if (flag & c) > 0 {
                contents.push(Self(c));
            }
        }
        contents
    }
}

impl TryFrom<ContentType> for String {
    type Error = ();

    fn try_from(item: ContentType) -> Result<Self, Self::Error> {
        match item {
            ContentType::JSON   => Ok(mime::APPLICATION_JSON.to_string()),
            ContentType::URL    => Ok(mime::APPLICATION_WWW_FORM_URLENCODED.to_string()),
            ContentType::FORM   => Ok(mime::MULTIPART_FORM_DATA.to_string()),
            _                   => Err(())
        }
    }
}
