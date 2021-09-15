use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! p_getter {
    ($method: ident, $result: ty, $conv: ident) => {
        prim_getter!("p.", $method, $result, $conv);
    }
}

macro_rules! p_str_getter {
    ($method: ident) => {
        p_getter!($method, String, string_owned);
    }
}

#[derive(Debug)]
pub struct PeerInner {
    peer_sha1_hex: String,
    download: Download,
}

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

    pub fn peer_sha1_hex(&self) -> &str {
        &self.inner.peer_sha1_hex
    }

    p_str_getter!(address);
}

unsafe impl Send for Peer {}
unsafe impl Sync for Peer {}

impl From<&Peer> for Value {
    fn from(peer: &Peer) -> Self {
        Value::String(format!("{}:p{}", &peer.inner.download.sha1_hex(), peer.inner.peer_sha1_hex))
    }
}
