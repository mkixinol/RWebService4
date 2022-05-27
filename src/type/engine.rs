use crate::r#type::{IDbNo};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Engine(IDbNo);

impl Engine {
    pub const ENGINE_TYPE_MODULE:       IDbNo = 0;
    pub const ENGINE_TYPE_V8:           IDbNo = -1;
    pub const ENGINE_TYPE_HTTP:         IDbNo = -2;
    pub const ENGINE_TYPE_HANDLEBARS:   IDbNo = -3;

    pub const ENGINE_MODULE_CONTENT:    IDbNo = 1;
    pub const ENGINE_MODULE_MEMBER:     IDbNo = 2;
    pub const ENGINE_MODULE_AUTH:       IDbNo = 3;
    pub const ENGINE_MODULE_PAGE:       IDbNo = 4;
    pub const ENGINE_MODULE_ACTION:     IDbNo = 5;
    pub const ENGINE_MODULE_TEMPLATE:   IDbNo = 6;

    pub const MODULE:       Engine = Engine(Self::ENGINE_TYPE_MODULE);
    pub const V8:           Engine = Engine(Self::ENGINE_TYPE_V8);
    pub const HTTP:         Engine = Engine(Self::ENGINE_TYPE_HTTP);
    pub const HANDLEBARS:   Engine = Engine(Self::ENGINE_TYPE_HANDLEBARS);
    pub const CONTENT:      Engine = Engine(Self::ENGINE_MODULE_CONTENT);
    pub const MEMBER:       Engine = Engine(Self::ENGINE_MODULE_MEMBER);
    pub const AUTH:         Engine = Engine(Self::ENGINE_MODULE_AUTH);
    pub const PAGE:         Engine = Engine(Self::ENGINE_MODULE_PAGE);
    pub const ACTION:       Engine = Engine(Self::ENGINE_MODULE_ACTION);
    pub const TEMPLATE:     Engine = Engine(Self::ENGINE_MODULE_TEMPLATE);
}
