use crate::r#type::{UDbFlg, UDbFlgBit};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DataFormat(pub UDbFlgBit);

impl DataFormat {
    pub const DATA_FORMAT_STATIC:  UDbFlgBit = 1 << 0;
    pub const DATA_FORMAT_HTML:    UDbFlgBit = 1 << 1;
    pub const DATA_FORMAT_TEXT:    UDbFlgBit = 1 << 2;
    pub const DATA_FORMAT_JSON:    UDbFlgBit = 1 << 3;

    pub const STATIC:   Self = Self(Self::DATA_FORMAT_STATIC);
    pub const HTML:     Self = Self(Self::DATA_FORMAT_HTML);
    pub const TEXT:     Self = Self(Self::DATA_FORMAT_TEXT);
    pub const JSON:     Self = Self(Self::DATA_FORMAT_JSON);

    pub const FLAGS: [UDbFlgBit; 4] = [
        Self::DATA_FORMAT_STATIC,
        Self::DATA_FORMAT_HTML,
        Self::DATA_FORMAT_TEXT,
        Self::DATA_FORMAT_JSON,
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
