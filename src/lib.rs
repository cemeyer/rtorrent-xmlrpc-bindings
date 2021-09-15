use std::sync::Arc;
use xmlrpc::{Request, Value};

pub(crate) mod value_conversion;
mod download;
mod peer;
mod tracker;

pub use download::Download;
pub use peer::Peer;
pub use tracker::DownloadTracker;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    XmlRpc(xmlrpc::Error),
    UnexpectedStructure(String),
}

impl From<xmlrpc::Error> for Error {
    fn from(x: xmlrpc::Error) -> Self {
        Error::XmlRpc(x)
    }
}

#[derive(Debug)]
struct ServerInner {
    endpoint: String,
}

#[derive(Clone, Debug)]
pub struct Server {
    inner: Arc<ServerInner>,
}

impl Server {
    pub fn new(endpoint: &str) -> Self {
        Self { inner: Arc::new(ServerInner { endpoint: endpoint.to_owned() }) }
    }

    #[inline]
    fn endpoint(&self) -> &str {
        &self.inner.endpoint
    }

    pub fn download_list(&self) -> Result<Vec<Download>> {
        let raw_list = Request::new("download_list").call_url(self.endpoint())?;
        value_conversion::list(&raw_list)?
            .iter()
            .map(|v| Download::from_value(self.clone(), v))
            .collect()
    }
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

pub(crate) mod macros {
    macro_rules! prim_getter {
        ($ns: literal, $method: ident, $result: ty, $conv: ident) => {
            pub fn $method(&self) -> Result<$result> {
                let val = Request::new(concat!($ns, stringify!($method)))
                    .arg(self)
                    .call_url(self.endpoint())?;
                value_conversion::$conv(&val)
            }
        }
    }
    pub(crate) use prim_getter;

    macro_rules! prim_setter {
        ($ns: literal, $rmethod: ident, $apimethod: ident, $ty: ty) => {
            pub fn $rmethod(&self, new: $ty) -> Result<()> {
                let val = Request::new(concat!($ns, stringify!($apimethod), ".set"))
                    .arg(self)
                    .arg(new)
                    .call_url(self.endpoint())?;
                value_conversion::void(&val)
            }
        }
    }
    pub(crate) use prim_setter;
}
