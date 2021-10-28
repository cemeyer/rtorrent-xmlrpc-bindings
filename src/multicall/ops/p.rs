//! Rtorrent p.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `p.*` operation for multicalls
    PeerMultiCallOp
}

/// `MultiBuilder` is a tool for building queries across many peers
///
/// The constructed query is executed in a single XMLRPC call.  The query results are in convenient
/// Rust types.
///
/// ## Usage
///
/// Example: Print every peer and peer IP address associated with a download.
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::p;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     println!("{}:", download.name()?);
///     p::MultiBuilder::new(&my_handle, download.sha1_hex())
///         .call(p::ID)
///         .call(p::ADDRESS)
///         .invoke()?
///         .iter()
///         .for_each(|(id, addr)| {
///             println!("  {}: {}", id, addr);
///         });
///     break;
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// The `call()` method can be invoked repeatedly to add more columns to the query -- in the above
/// example, selecting the `ID` and `ADDRESS` columns.
pub struct MultiBuilder {
    pub(crate) inner: raw::MultiBuilder,
}

impl MultiBuilder {
    /// Start building a multicall over peers associated with a download on `server`.
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
            inner: raw::MultiBuilder::new(server, "p.multicall", download_sha1, ""),
        }
    }
}

macro_rules! define_builder {
    ( $(#[$meta:meta])* $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!($(#[$meta])* PeerMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! p_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* PeerMultiCallOp, $name, $res, "p.", $api);
    };
}

p_op_const!(
    /// Get the IP address of the peer.
    ADDRESS, String, "address");
p_op_const!(
    /// Is the peer banned, e.g., for sending "too much" corrupt data?
    BANNED, bool, "banned");
p_op_const!(
    /// Get the parsed client version of the peer, if it is a client rtorrent recognizes.
    /// Otherwise, `"Unknown"` is returned.
    CLIENT_VERSION, String, "client_version");
p_op_const!(
    /// Return the percent of the download the peer reports it has completed.
    COMPLETED_PERCENT, i64, "completed_percent");
p_op_const!(
    /// The download rate from this peer, in bytes/second.
    DOWN_RATE, i64, "down_rate");
p_op_const!(
    /// Total bytes downloaded from this peer.
    DOWN_TOTAL, i64, "down_total");
p_op_const!(
    /// Get the (internal to rtorrent) identifier for this peer.
    ID, String, "id");
p_op_const!(
    /// Get the unparsed client ID sent by the peer.  Supposed to be URL-encoded.  This is what
    /// the [`client_version`] (`CLIENT_VERSION`) method attempts to parse.  See BEP 20 for details.
    ///
    /// [`client_version`]: crate::Peer::client_version
    ID_HTML, String, "id_html");
p_op_const!(
    /// Is the connection to this peer "encrypted?"
    IS_ENCRYPTED, bool, "is_encrypted");
p_op_const!(
    /// Did the peer initiate this connection?
    IS_INCOMING, bool, "is_incoming");
p_op_const!(
    /// Is this connection obfuscated?
    IS_OBFUSCATED, bool, "is_obfuscated");
p_op_const!(
    IS_PREFERRED, bool, "is_preferred");
p_op_const!(
    IS_UNWANTED, bool, "is_unwanted");
p_op_const!(
    /// Get the (estimated) peer download rate (from the entire swarm, not just this client).
    PEER_RATE, i64, "peer_rate");
p_op_const!(
    /// Get the (estimated) peer download total (from the entire swarm, not just this client).
    PEER_TOTAL, i64, "peer_total");
p_op_const!(
    /// The remote port of the connection to this peer.
    PORT, i64, "port");
p_op_const!(
    /// Is the peer snubbed?
    SNUBBED, bool, "snubbed");
p_op_const!(
    /// The upload rate to this peer, in bytes/second.
    UP_RATE, i64, "up_rate");
p_op_const!(
    /// Total bytes uploaded to this peer.
    UP_TOTAL, i64, "up_total");
