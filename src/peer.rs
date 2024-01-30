/*! Torrent peers

This module defines the [`Peer`] type and support code.

[`Peer`]: crate::Peer
!*/

use crate::macros::*;
use crate::{Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! p_getter {
    ($(#[$meta:meta])* $method: ident, $result: ty) => {
        prim_getter!($(#[$meta])* "p.", $method, $result);
    }
}

macro_rules! p_bool_getter {
    ($(#[$meta:meta])* $method: ident) => {
        p_getter!($(#[$meta])* $method, bool);
    }
}

macro_rules! p_int_getter {
    ($(#[$meta:meta])* $method: ident) => {
        p_getter!($(#[$meta])* $method, i64);
    }
}

macro_rules! p_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        p_getter!($(#[$meta])* $method, String);
    }
}

macro_rules! p_bool_setter {
    ($(#[$meta:meta])* $rmethod: ident, $apimethod: ident) => {
        prim_setter!($(#[$meta])* "p.", $rmethod, $apimethod, bool);
    }
}

#[derive(Debug)]
pub(crate) struct PeerInner {
    peer_sha1_hex: String,
    download: Download,
}

/// A `Peer` associated with a [`Download`]
///
/// Accessors on `Peer` correspond to the `p.*` rtorrent APIs.
///
/// # Examples
///
/// Introspecting a peer:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Peer;
/// fn print_peer_info(peer: Peer) {
///     if let Ok(address) = peer.address() {
///         println!("Peer IP: {}", address);
///     }
/// }
/// ```
///
/// Enumerating peers associated with a download:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::{Download, Peer, Result};
/// # fn print_peer_info(peer: Peer) {
/// #     if let Ok(address) = peer.address() {
/// #         println!("Peer IP: {}", address);
/// #     }
/// # }
/// fn enum_peers(dl: Download) -> Result<()> {
///     for peer in dl.peers()? {
///         print_peer_info(peer);
///     }
///     Ok(())
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
        Self {
            inner: Arc::new(PeerInner {
                download,
                peer_sha1_hex: peerhash.to_owned(),
            }),
        }
    }

    pub fn execute(&self, request: Request) -> std::result::Result<Value, xmlrpc::Error> {
        self.inner.download.execute(request)
    }

    p_str_getter!(
        /// Get the IP address of the peer.
        address
    );
    p_bool_getter!(
        /// Is the peer banned, e.g., for sending "too much" corrupt data?
        banned
    );
    p_bool_setter!(
        /// Ban the peer.
        set_banned,
        banned
    );
    p_str_getter!(
        /// Get the parsed client version of the peer, if it is a client rtorrent recognizes.
        /// Otherwise, `"Unknown"` is returned.
        client_version
    );
    p_int_getter!(
        /// Return the percent of the download the peer reports it has completed.
        completed_percent
    );
    p_int_getter!(
        /// The download rate from this peer, in bytes/second.
        down_rate
    );
    p_int_getter!(
        /// Total bytes downloaded from this peer.
        down_total
    );
    p_str_getter!(
        /// Get the unparsed client ID sent by the peer.  Supposed to be URL-encoded.  This is what
        /// the [`client_version`] method attempts to parse.  See BEP 20 for details.
        ///
        /// [`client_version`]: crate::Peer::client_version
        id_html
    );
    p_bool_getter!(
        /// Is the connection to this peer "encrypted?"
        is_encrypted
    );
    p_bool_getter!(
        /// Did the peer initiate this connection?
        is_incoming
    );
    p_bool_getter!(
        /// Is this connection obfuscated?
        is_obfuscated
    );
    p_bool_getter!(is_preferred);
    p_bool_getter!(is_unwanted);
    p_int_getter!(
        /// Get the (estimated) peer download rate (from the entire swarm, not just this client).
        peer_rate
    );
    p_int_getter!(
        /// Get the (estimated) peer download total (from the entire swarm, not just this client).
        peer_total
    );
    p_int_getter!(
        /// The remote port of the connection to this peer.
        port
    );
    p_bool_getter!(
        /// Is the peer snubbed?
        snubbed
    );
    p_bool_setter!(
        /// Snub the peer.
        set_snubbed,
        snubbed
    );
    p_int_getter!(
        /// The upload rate to this peer, in bytes/second.
        up_rate
    );
    p_int_getter!(
        /// Total bytes uploaded to this peer.
        up_total
    );
}

unsafe impl Send for Peer {}
unsafe impl Sync for Peer {}

impl From<&Peer> for Value {
    fn from(peer: &Peer) -> Self {
        Value::String(format!(
            "{}:p{}",
            &peer.inner.download.sha1_hex(),
            peer.inner.peer_sha1_hex
        ))
    }
}
