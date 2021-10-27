//! Rtorrent p.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `p.*` operation for multicalls.
    PeerMultiCallOp
}

/// The `MultiBuilder` type is a tool for building queries of one or more fields across many peers
/// in a single XMLRPC call.  The query results are nicely typed.
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
    /// Start building a multicall over peers associated with a `download_sha1` (infohash) on
    /// `server`.
    pub fn new(server: &Server, download_sha1: &str) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "p.multicall", download_sha1, ""),
        }
    }
}

macro_rules! define_builder {
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!(PeerMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
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
    /// Get the (internal to rtorrent) identifier for this peer.
    ID, String, "id");
