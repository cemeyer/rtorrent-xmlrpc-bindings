//! Rtorrent f.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// An `f.*` operation for multicalls.
    FileMultiCallOp
}

/// The `MultiBuilder` type is a tool for building queries of one or more fields across many files
/// in a single XMLRPC call.  The query results are nicely typed.
///
/// ## Usage
///
/// Example: Print every file associated with a download.
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::f;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     let name = download.name()?;
///     println!("{}: {:?}",
///         name,
///         f::MultiBuilder::new(&my_handle, download.sha1_hex(), None)
///             .call(f::PATH)
///             .invoke()?);
///     break;
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// The `call()` method can be invoked repeatedly to add more columns to the query.
pub struct MultiBuilder {
    pub(crate) inner: raw::MultiBuilder,
}

impl MultiBuilder {
    /// Start building a multicall over files associated with a `download_sha1` (infohash) on
    /// `server`.
    ///
    /// The optional `glob` parameter can be used to filter the queried files using basic glob-like
    /// syntax.
    pub fn new(server: &Server, download_sha1: &str, glob: Option<&str>) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "f.multicall", download_sha1, glob.unwrap_or("")),
        }
    }
}

macro_rules! define_builder {
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!(FileMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! f_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* FileMultiCallOp, $name, $res, "p.", $api);
    };
}

f_op_const!(
    /// Get the path of this file, relative to the download's base path.
    PATH, String, "path");
