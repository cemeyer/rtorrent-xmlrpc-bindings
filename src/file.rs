use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! f_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "f.", $method, String, string_owned);
    }
}

#[derive(Debug)]
pub(crate) struct FileInner {
    download: Download,
    index: i64,
}

/// Represents a single file associated with a download.
#[derive(Clone, Debug)]
pub struct File {
    inner: Arc<FileInner>,
}

impl File {
    pub(crate) fn new(download: Download, index: i64) -> Self {
        Self { inner: Arc::new(FileInner { download, index, }) }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    f_str_getter!(
        /// Get the path of this file, relative to the download's base path.
        path);
    f_str_getter!(
        /// Get the absolute path of this file.
        frozen_path);
}

unsafe impl Send for File {}
unsafe impl Sync for File {}

impl From<&File> for Value {
    fn from(file: &File) -> Self {
        Value::String(format!("{}:f{}", &file.inner.download.sha1_hex(), file.inner.index))
    }
}
