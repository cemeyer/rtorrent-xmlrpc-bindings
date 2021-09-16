use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! t_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "t.", $method, String, string_owned);
    }
}

#[derive(Debug)]
pub(crate) struct TrackerInner {
    download: Download,
    index: i64,
}

/// Represents a tracker associated with a download.
#[derive(Clone, Debug)]
pub struct Tracker {
    inner: Arc<TrackerInner>,
}

impl Tracker {
    pub(crate) fn new(download: Download, index: i64) -> Self {
        Self { inner: Arc::new(TrackerInner { download, index, }) }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    t_str_getter!(
        /// Get the URL of the tracker.
        url);
}

unsafe impl Send for Tracker {}
unsafe impl Sync for Tracker {}

impl From<&Tracker> for Value {
    fn from(tracker: &Tracker) -> Self {
        Value::String(format!("{}:t{}", &tracker.inner.download.sha1_hex(), tracker.inner.index))
    }
}
