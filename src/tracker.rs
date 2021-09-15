use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! t_str_getter {
    ($method: ident) => {
        prim_getter!("t.", $method, String, string_owned);
    }
}

#[derive(Debug)]
pub struct DownloadTrackerInner {
    download: Download,
    index: i64,
}

#[derive(Clone, Debug)]
pub struct DownloadTracker {
    inner: Arc<DownloadTrackerInner>,
}

impl DownloadTracker {
    pub(crate) fn new(download: Download, index: i64) -> Self {
        Self { inner: Arc::new(DownloadTrackerInner { download, index, }) }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    t_str_getter!(url);
}

unsafe impl Send for DownloadTracker {}
unsafe impl Sync for DownloadTracker {}

impl From<&DownloadTracker> for Value {
    fn from(dlt: &DownloadTracker) -> Self {
        Value::String(format!("{}:t{}", &dlt.inner.download.sha1_hex(), dlt.inner.index))
    }
}
