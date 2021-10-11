/*! # rtorrent-xmlrpc-bindings

`rtorrent-xmlrpc-bindings` provides strongly-typed Rust bindings for the [rtorrent] [XMLRPC API].

The XMLRPC API allows a high degree of introspection and control over an rtorrent instance.

## Usage

The top-level structure representing an rtorrent instance is [`Server`].  All errors produced by
the crate are encapsulated by the [`Error`] type.

```rust
use rtorrent_xmlrpc_bindings as rtorrent;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
println!("Hostname: {}", my_handle.hostname()?);

for download in my_handle.download_list()? {
    println!("Download: {}", download.name()?);
}
```

## Current Limitations

* Some XMLRPC APIs are not yet wrapped by this crate.
* Multicalls, which could make some queries much more efficient, are not currently supported by
  this crate.

[rtorrent]: https://rakshasa.github.io/rtorrent/
[XMLRPC API]: https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html

[`Error`]: crate::Error
[`Server`]: crate::Server
!*/

use std::sync::Arc;
use xmlrpc::{Request, Value};

pub(crate) mod value_conversion;
mod download;
mod file;
mod peer;
mod tracker;

pub use download::Download;
pub use file::File;
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

macro_rules! server_str_getter {
    ($(#[$meta:meta])* $method: ident, $api: literal) => {
        $(#[$meta])*
        pub fn $method(&self) -> Result<String> {
            let val = Request::new($api)
                .call_url(self.endpoint())?;
            value_conversion::string_owned(&val)
        }
    }
}

macro_rules! server_int_getter {
    ($(#[$meta:meta])* $method: ident, $api: literal) => {
        $(#[$meta])*
        pub fn $method(&self) -> Result<i64> {
            let val = Request::new($api)
                .call_url(self.endpoint())?;
            value_conversion::int(&val)
        }
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

    server_str_getter!(
        /// Get the IP address associated with this rtorrent instance.
        ip, "network.bind_address");
    server_str_getter!(
        /// Get the hostname associated with this rtorrent instance.
        hostname, "system.hostname");

    server_str_getter!(
        /// Get the XMLRPC API version associated with this instance.
        api_version, "system.api_version");
    server_str_getter!(
        /// Get the rtorrent version associated with this instance.
        client_version, "system.client_version");
    server_str_getter!(
        /// Get the libtorrent version associated with this instance.
        library_version, "system.library_version");

    server_int_getter!(
        /// Get the total downloaded metric for this instance (bytes).
        down_total, "throttle.global_down.total");
    server_int_getter!(
        /// Get the current download rate for this instance (bytes/s).
        down_rate, "throttle.global_down.rate");
    server_int_getter!(
        /// Get the total uploaded metric for this instance (bytes).
        up_total, "throttle.global_up.total");
    server_int_getter!(
        /// Get the current upload rate for this instance (bytes/s).
        up_rate, "throttle.global_up.rate");
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
    macro_rules! prim_getter_named {
        (
            $(#[$meta:meta])*
            $ns: literal, $method: ident, $result: ty, $conv: ident, $apimethod: literal
        ) => {
            $(#[$meta])*
            pub fn $method(&self) -> Result<$result> {
                let val = Request::new(concat!($ns, $apimethod))
                    .arg(self)
                    .call_url(self.endpoint())?;
                value_conversion::$conv(&val)
            }
        }
    }
    pub(crate) use prim_getter_named;

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
