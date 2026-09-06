
use reqwest::{Client, ClientBuilder};
use crate::traits::Router;

/// # API context
///
/// This defines details each API share, like session or origin.
///
/// **Note**: This type still has no stable layout, as decision
/// on whether "origin" customization is to be supported.
///
#[derive(Clone)]
pub struct Context {
    /// Session associated with this context
    pub session: Client
}

impl Router for Context {
    #[inline] fn router_path(&self)->impl AsRef<str> { "https://www.newgrounds.com" }
    #[inline] fn router_parent(&self)->Option<&impl Router> { Option::<&Self>::None }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            session: Client::builder()
                // FIXME: expose modules?
                .user_agent("NG-rs/v0")
                .https_only(true)
                .no_gzip()
                .build()
                .expect("Unrecoverable TLS failure")
        }
    }
}

impl From<Client> for Context {
    fn from(value:Client) -> Self {
        Self {
            session: value
        }
    }
}

impl TryFrom<ClientBuilder> for Context {
    type Error = reqwest::Error;
    fn try_from(builder: ClientBuilder) -> Result<Self, Self::Error> {
        Ok(Self { session: builder.build()? })
    }
}
