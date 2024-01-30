/*! # rtorrent-xmlrpc-bindings

`rtorrent-xmlrpc-bindings` provides strongly-typed Rust bindings for the [rtorrent] [XMLRPC API].

The XMLRPC API allows a high degree of introspection and control over an rtorrent instance.

## Usage

The top-level structure representing an rtorrent instance is [`Server`].  All errors produced by
the crate are encapsulated by the [`Error`] type.

```no_run
use rtorrent_xmlrpc_bindings as rtorrent;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
println!("Hostname: {}", my_handle.hostname()?);

for download in my_handle.download_list()? {
    println!("Download: {}", download.name()?);
}
# Ok::<(), rtorrent::Error>(())
```

It can be more efficient to query multiple items at a time.  Rtorrent's XMLRPC API exposes an
interface for this called "multicalls."  In this crate, they are available through the
[`multicall`] submodule.

The following example queries the name and ratio of every torrent in rtorrent's "default" view and
prints the results.

```no_run
use rtorrent_xmlrpc_bindings as rtorrent;
use rtorrent::multicall::d;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");

d::MultiBuilder::new(&my_handle, "default")
    .call(d::NAME)
    .call(d::RATIO)
    .invoke()?
    .iter()
    .for_each(|(name, ratio)| {
        println!("{}: ratio: {}", name, ratio);
    });
# Ok::<(), rtorrent::Error>(())
```

## Current Limitations

* Some XMLRPC APIs are not yet wrapped by this crate.

[rtorrent]: https://rakshasa.github.io/rtorrent/
[XMLRPC API]: https://rtorrent-docs.readthedocs.io/en/latest/cmd-ref.html

[`Error`]: crate::Error
[`multicall`]: crate::multicall
[`Server`]: crate::Server
!*/

use std::sync::Arc;
use xmlrpc::{Request, Value};

mod download;
mod file;
pub mod multicall;
mod peer;
mod tracker;
pub(crate) mod value_conversion;

pub use download::Download;
pub use file::File;
pub use peer::Peer;
pub use tracker::Tracker;
pub use value_conversion::TryFromValue;

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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::XmlRpc(xe) => {
                write!(f, "XML-RPC: {}", xe)
            }
            Error::UnexpectedStructure(us) => {
                write!(f, "Unexpected XML structure: {}", us)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::XmlRpc(xe) => Some(xe),
            _ => None,
        }
    }
}

macro_rules! server_getter {
    ($(#[$meta:meta])* $method: ident, $api: literal, $ty: ty) => {
        $(#[$meta])*
        pub fn $method(&self) -> Result<$ty> {
            let val = Request::new($api)
                .call_url(self.endpoint())?;
            <$ty as TryFromValue>::try_from_value(&val)
        }
    }
}

#[derive(Debug)]
struct ServerInner {
    endpoint: String,
}

/// `Server` represents a logical rtorrent instance
#[derive(Clone, Debug)]
pub struct Server {
    inner: Arc<ServerInner>,
}

impl Server {
    /// Instantiate the API at some URI.
    ///
    /// ```no_run
    /// # use rtorrent_xmlrpc_bindings as rtorrent;
    /// # use rtorrent::Server;
    /// let server = Server::new("http://myhostname/RPC2");
    /// for dl in server.download_list()? {
    ///   // ...
    /// }
    /// # Ok::<(), rtorrent::Error>(())
    /// ```
    pub fn new(endpoint: &str) -> Self {
        Self {
            inner: Arc::new(ServerInner {
                endpoint: endpoint.to_owned(),
            }),
        }
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
            .map(|v| Download::from_value(&self, v))
            .collect()
    }

    /// Add torrent from url/magnetlink.
    ///
    /// If start is true, also start the added download.
    pub fn load_torrent_url(&self, link: &str, start: bool) -> Result<i64> {
        let load = if start {
            "load.start_verbose"
        } else {
            "load.verbose"
        };
        let raw_response = Request::new(load)
            .arg("")
            .arg(link.to_string())
            .call_url(self.endpoint())?;
        <i64 as TryFromValue>::try_from_value(&raw_response)
    }

    /// Add torrent from torrent file contents.
    ///
    /// If start is true, also start the added download.
    pub fn load_torrent_bytes(&self, contents: &[u8], start: bool) -> Result<i64> {
        let load = if start {
            "load.raw_start_verbose"
        } else {
            "load.raw_verbose"
        };
        let raw_response = Request::new(load)
            .arg("")
            .arg(contents.to_vec())
            .call_url(self.endpoint())?;
        <i64 as TryFromValue>::try_from_value(&raw_response)
    }

    server_getter!(
        /// Get the IP address associated with this rtorrent instance.
        ip,
        "network.bind_address",
        String
    );
    server_getter!(
        /// Get the port(s) associated with this rtorrent instance.
        port,
        "network.port_range",
        String
    );
    server_getter!(
        /// Get the hostname associated with this rtorrent instance.
        hostname,
        "system.hostname",
        String
    );
    server_getter!(
        /// Get the time in seconds since Unix Epoch when this rtorrent instance was started.
        startup_time,
        "system.startup_time",
        i64
    );
    server_getter!(
        /// Exit rtorrent, informing trackers that we are going away and waiting some time for them
        /// to acknowledge.
        exit_rtorrent,
        "system.shutdown.normal",
        i64
    );
    server_getter!(
        /// Get the XMLRPC API version associated with this instance.
        api_version,
        "system.api_version",
        String
    );
    server_getter!(
        /// Get the rtorrent version associated with this instance.
        client_version,
        "system.client_version",
        String
    );
    server_getter!(
        /// Get the libtorrent version associated with this instance.
        library_version,
        "system.library_version",
        String
    );

    server_getter!(
        /// Get the total downloaded metric for this instance (bytes).
        down_total,
        "throttle.global_down.total",
        i64
    );
    server_getter!(
        /// Get the current download rate for this instance (bytes/s).
        down_rate,
        "throttle.global_down.rate",
        i64
    );
    server_getter!(
        /// Get the total uploaded metric for this instance (bytes).
        up_total,
        "throttle.global_up.total",
        i64
    );
    server_getter!(
        /// Get the current upload rate for this instance (bytes/s).
        up_rate,
        "throttle.global_up.rate",
        i64
    );
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

pub(crate) mod macros {
    pub(crate) use super::TryFromValue;

    macro_rules! prim_getter {
        (
            $(#[$meta:meta])*
            $ns: literal, $method: ident, $result: ty
        ) => {
            $(#[$meta])*
            pub fn $method(&self) -> Result<$result> {
                let val = Request::new(concat!($ns, stringify!($method)))
                    .arg(self)
                    .call_url(self.endpoint())?;
                <$result as TryFromValue>::try_from_value(&val)
            }
        }
    }
    pub(crate) use prim_getter;
    macro_rules! prim_getter_named {
        (
            $(#[$meta:meta])*
            $ns: literal, $method: ident, $result: ty, $apimethod: literal
        ) => {
            $(#[$meta])*
            pub fn $method(&self) -> Result<$result> {
                let val = Request::new(concat!($ns, $apimethod))
                    .arg(self)
                    .call_url(self.endpoint())?;
                <$result as TryFromValue>::try_from_value(&val)
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
                <() as TryFromValue>::try_from_value(&val)
            }
        }
    }
    pub(crate) use prim_setter;
}
