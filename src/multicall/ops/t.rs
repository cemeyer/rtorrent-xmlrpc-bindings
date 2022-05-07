//! Rtorrent t.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `t.*` operation for multicalls
    TrackerMultiCallOp
}

/// `MultiBuilder` is a tool for building queries across many trackers
///
/// The constructed query is executed in a single XMLRPC call.  The query results are in convenient
/// Rust types.
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
    /// Start building a multicall over trackers associated with a download on `server`.
    ///
    /// The download is identified by the SHA1 of its "infohash", `download_sha1`, which can be
    /// obtained via [`Download::sha1_hex`] from some `Download` object, or the result of a
    /// [`d::HASH`] call using [`multicall::d::MultiBuilder`].
    ///
    /// [`Download::sha1_hex`]: crate::Download::sha1_hex
    /// [`d::HASH`]: crate::multicall::d::HASH
    /// [`multicall::d::MultiBuilder`]: crate::multicall::d::MultiBuilder
    pub fn new(server: &Server, download_sha1: &str) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "t.multicall", download_sha1, ""),
        }
    }
}

macro_rules! define_builder {
    ( $(#[$meta:meta])* $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!($(#[$meta])* TrackerMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! t_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* TrackerMultiCallOp, $name, $res, "t.", $api);
    };
}

t_op_const!(
    /// Get the URL of the tracker.
    URL, String, "url");

t_op_const!(
    // Get the last time rtorrent attempted to contact tracker.
    ACTIVTY_TIME_LAST, i64, "activity_time_last");

t_op_const!(
    // Get the next time rtorrent will attempt to contact tracker.
    ACTIVTY_TIME_NEXT, i64, "activity_time_next");
    
t_op_const!(
    // Get the next time rtorrent will attempt to contact tracker.
    GROUP, i64, "group");
t_op_const!(
    // Get the tracker ID.
    ID, String, "id");

t_op_const!(
    // Get the number of peers the tracker returned last scrape.
    LATEST_SUM_PEERS, i64, "latest_sum_peers");
