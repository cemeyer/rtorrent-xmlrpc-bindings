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

use std::io::{Cursor, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use xmlrpc::{Request, Transport, Value};

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
            let val = self.execute(Request::new($api))?;
            <$ty as TryFromValue>::try_from_value(&val)
        }
    }
}

#[derive(Clone, Debug)]
struct UnixSocketTransport {
    socket: String,
}

impl UnixSocketTransport {
    pub fn new(socket: &str) -> Self {
        UnixSocketTransport {
            socket: socket.to_owned(),
        }
    }

    fn scgi_headers(size: usize) -> String {
        let headers = vec![
            ("CONTENT_LENGTH", format!("{size}")),
            ("SCGI", "1".to_string()),
            ("REQUEST_METHOD", "POST".to_string()),
            ("SERVER_PROTOCOL", "HTTP/1.1".to_string()),
        ]
        .into_iter()
        .map(|(k, v)| format!("{k}\0{v}\0"))
        .collect::<Vec<String>>()
        .join("");
        format!("{}:{headers},", headers.len())
    }

    fn process_body(
        &self,
        body: &[u8],
    ) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut stream = UnixStream::connect(&self.socket)?;
        let headers = UnixSocketTransport::scgi_headers(body.len());
        stream.write_all(headers.as_ref())?;
        stream.write_all(body)?;

        let mut response = Vec::new();
        stream.read_to_end(&mut response)?;

        // Find the end of headers by searching for two \r\n in a row.
        let header_cut = response
            .windows(4)
            .enumerate()
            .find_map(|(idx, window)| {
                if window == b"\r\n\r\n" {
                    Some(idx + 4)
                } else {
                    None
                }
            })
            .expect("foo");
        response.drain(..header_cut);
        Ok(response)
    }
}

impl Transport for UnixSocketTransport {
    type Stream = Cursor<Vec<u8>>;
    fn transmit(
        self,
        request: &Request<'_>,
    ) -> std::result::Result<Self::Stream, Box<dyn std::error::Error + Send + Sync>> {
        let mut body = Vec::new();
        // This unwrap never panics as we are using `Vec<u8>` as a `Write` implementor,
        // and not doing anything else that could return an `Err` in `write_as_xml()`.
        request.write_as_xml(&mut body).unwrap();
        let response = self.process_body(&body)?;
        Ok(Cursor::new(response))
    }
}

#[derive(Debug)]
enum Endpoint {
    Http(String),
    Scgi(UnixSocketTransport),
}

#[derive(Debug)]
struct ServerInner {
    endpoint: Endpoint,
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
                endpoint: if endpoint.starts_with("http://") {
                    Endpoint::Http(endpoint.to_owned())
                } else {
                    Endpoint::Scgi(UnixSocketTransport::new(endpoint))
                },
            }),
        }
    }

    pub fn execute(&self, request: Request) -> std::result::Result<Value, xmlrpc::Error> {
        match &self.inner.endpoint {
            Endpoint::Http(url) => request.call_url(url),
            Endpoint::Scgi(transport) => request.call(transport.clone()),
        }
    }

    /// Get a list of all downloads loaded in this instance of rtorrent.
    pub fn download_list(&self) -> Result<Vec<Download>> {
        let raw_list = self.execute(Request::new("download_list"))?;
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
        let raw_response = self.execute(Request::new(load).arg("").arg(link.to_string()))?;
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
        let raw_response = self.execute(Request::new(load).arg("").arg(contents.to_vec()))?;
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
                let val = self.execute(Request::new(concat!($ns, stringify!($method)))
                    .arg(self))?;
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
                let val = self.execute(Request::new(concat!($ns, $apimethod))
                    .arg(self))?;
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
            pub fn $rmethod(&self, _new: $ty) -> Result<()> {
                let val = self.execute(Request::new(concat!($ns, stringify!($apimethod), ".set"))
                    .arg(self))?;
                <() as TryFromValue>::try_from_value(&val)
            }
        }
    }
    pub(crate) use prim_setter;
}
