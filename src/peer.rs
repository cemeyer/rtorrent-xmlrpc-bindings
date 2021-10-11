/*! Torrent peers

This module defines the [`Peer`] type and support code.

[`Peer`]: crate::Peer
!*/

use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! p_getter {
    ($(#[$meta:meta])* $method: ident, $result: ty, $conv: ident) => {
        prim_getter!($(#[$meta])* "p.", $method, $result, $conv);
    }
}

macro_rules! p_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        p_getter!($(#[$meta])* $method, String, string_owned);
    }
}

#[derive(Debug)]
pub(crate) struct PeerInner {
    peer_sha1_hex: String,
    download: Download,
}

/// A peer associated with a [`Download`]
///
/// Accessors on `Peer` correspond to the `p.*` rtorrent APIs.
///
/// # Examples
///
/// Enumerating peers associated with a download:
///
/// ```rust
/// let dl: Download = ...;
/// for peer in dl.peers()? {
///     print_peer_info(peer)?;
/// }
/// ```
///
/// Introspecting a peer:
///
/// ```rust
/// fn print_peer_info(peer: Peer) {
///     if let Ok(address) = peer.address() {
///         println!("Peer IP: {}", address);
///     }
/// }
/// ```
///
/// ## Caveats
///
/// Peers may disappear at any time, and RPC calls requesting peer information may spontaneously
/// fail after they do.  Code that accesses peer-related information should be prepared for
/// frequent `Err` results.
///
/// [`Download`]: crate::Download
#[derive(Clone, Debug)]
pub struct Peer {
    inner: Arc<PeerInner>,
}

impl Peer {
    pub(crate) fn new(download: Download, peerhash: &str) -> Self {
        Self { inner: Arc::new(PeerInner { download, peer_sha1_hex: peerhash.to_owned(), }) }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    p_str_getter!(
        /// Get the IP address of the peer.
        address);
}

unsafe impl Send for Peer {}
unsafe impl Sync for Peer {}

impl From<&Peer> for Value {
    fn from(peer: &Peer) -> Self {
        Value::String(format!("{}:p{}", &peer.inner.download.sha1_hex(), peer.inner.peer_sha1_hex))
    }
}
