//! Rtorrent f.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// An `f.*` operation for multicalls
    FileMultiCallOp
}

/// `MultiBuilder` is a tool for building queries across many files
///
/// The constructed query is executed in a single XMLRPC call.  The query results are in convenient
/// Rust types.
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
    /// Start building a multicall over files associated with a download on `server`.
    ///
    /// The download is identified by the SHA1 of its "infohash", `download_sha1`, which can be
    /// obtained via [`Download::sha1_hex`] from some `Download` object, or the result of a
    /// [`d::HASH`] call using [`multicall::d::MultiBuilder`].
    ///
    /// Optionally, the `glob` parameter may be used to filter the queried files using basic
    /// glob-like syntax (e.g., `Some("*.iso")`).
    ///
    /// [`Download::sha1_hex`]: crate::Download::sha1_hex
    /// [`d::HASH`]: crate::multicall::d::HASH
    /// [`multicall::d::MultiBuilder`]: crate::multicall::d::MultiBuilder
    pub fn new(server: &Server, download_sha1: &str, glob: Option<&str>) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "f.multicall", download_sha1, glob.unwrap_or("")),
        }
    }
}

macro_rules! define_builder {
    ( $(#[$meta:meta])* $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!($(#[$meta])* FileMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! f_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* FileMultiCallOp, $name, $res, "p.", $api);
    };
}

f_op_const!(
    /// The number of completed chunks associated with this file (including chunks that only
    /// partially include this file).
    COMPLETED_CHUNKS, i64, "completed_chunks");
f_op_const!(
    /// Get the absolute path of this file.
    FROZEN_PATH, String, "frozen_path");
f_op_const!(
    /// The offset (in bytes) of the file from the start of the torrent data.
    OFFSET, i64, "offset");
f_op_const!(
    /// Get the path of this file, relative to the download's base path.
    PATH, String, "path");
f_op_const!(
    /// The priority of the file.
    ///
    /// * `0`: Off. Do not download.
    /// * `1`: Normal.
    /// * `2`: High. Prioritize this file's chunks over "Normal" files.
    PRIORITY, i64, "priority");
f_op_const!(
    /// Get the size of the file, in bytes.
    SIZE_BYTES, i64, "size_bytes");
f_op_const!(
    /// The number of chunks associated with this file (including chunks that only partially
    /// include this file).
    SIZE_CHUNKS, i64, "size_chunks");
