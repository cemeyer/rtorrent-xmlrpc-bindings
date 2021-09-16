use std::sync::Arc;
use xmlrpc::{Request, Value};

pub(crate) mod value_conversion;
mod download;
mod peer;
mod tracker;

pub use download::Download;
pub use peer::Peer;
pub use tracker::Tracker;

/// The canonical [`Result`] for this crate (we return the same error type everywhere).
pub type Result<T> = std::result::Result<T, Error>;

/// The unified error type for this crate.
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

/// Represents a logical rtorrent instance.
#[derive(Clone, Debug)]
pub struct Server {
    inner: Arc<ServerInner>,
}

impl Server {
    /// Instantiate the API at some URI.
    ///
    /// ```
    /// let server = Server::new("http://myhostname/RPC2");
    /// for dl in server.download_list()? {
    ///   // ...
    /// }
    /// ```
    pub fn new(endpoint: &str) -> Self {
        Self { inner: Arc::new(ServerInner { endpoint: endpoint.to_owned() }) }
    }

    #[inline]
    fn endpoint(&self) -> &str {
        &self.inner.endpoint
    }

    /// Get a list of all downloads loaded in this instance of rtorrent.
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
        (
            $(#[$meta:meta])*
            $ns: literal, $method: ident, $result: ty, $conv: ident
        ) => {
            $(#[$meta])*
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
        (
            $(#[$meta:meta])*
            $ns: literal, $rmethod: ident, $apimethod: ident, $ty: ty
        ) => {
            $(#[$meta])*
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
