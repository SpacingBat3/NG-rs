
use reqwest::Client;

use crate::traits::ApiRoot;

/// # API context
///
/// This defines details each API share, like session or origin.
///
/// **Note**: This type still has no stable layout, as decision
/// on whether "origin" customization is to be supported.
///
#[derive(Clone)]
pub struct ApiCtx {
    pub session: Client
}

// Constants
impl ApiRoot for ApiCtx {
    const ORIGIN:&'static str = "https://www.newgrounds.com";
}

impl Default for ApiCtx {
    fn default() -> Self {
        Self {
            session: Client::builder()
                // FIXME: expose modules?
                .user_agent("NG-rs/v0")
                .build()
                .expect("Unrecoverable TLS failure")
        }
    }
}

impl From<Client> for ApiCtx {
    fn from(value:Client) -> Self {
        Self {
            session: value
        }
    }
}

// Getters
impl ApiCtx {
    #[inline] pub const fn get_session(&self) -> &Client { &self.session  }
}
