//! Rtorrent t.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `t.*` operation for multicalls.
    TrackerMultiCallOp
}

/// The `MultiBuilder` type is a tool for building queries of one or more fields across many
/// trackers in a single XMLRPC call.  The query results are nicely typed.
///
/// ## Usage
///
/// Example: Print every tracker URL associated with a download.
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::t;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     let name = download.name()?;
///     let tracker_urls = t::MultiBuilder::new(&my_handle, download.sha1_hex())
///         .call(t::URL)
///         .invoke()?
///         .into_iter()
///         .map(|(addr,)| addr)
///         .collect::<Vec<_>>();
///     println!("{}: URLs: {:?}", name, tracker_urls);
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
    /// Start building a multicall over trackers associated with a `download_sha1` (infohash) on
    /// `server`.
    pub fn new(server: &Server, download_sha1: &str) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "p.multicall", download_sha1, ""),
        }
    }
}

macro_rules! define_builder {
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!(TrackerMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! t_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* TrackerMultiCallOp, $name, $res, "p.", $api);
    };
}

t_op_const!(
    /// Get the URL of the tracker.
    URL, String, "url");
