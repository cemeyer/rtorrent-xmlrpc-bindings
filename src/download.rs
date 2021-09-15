use crate::macros::*;
use crate::{value_conversion, DownloadTracker, Result, Server};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! d_getter {
    ($method: ident, $result: ty, $conv: ident) => {
        prim_getter!("d.", $method, $result, $conv);
    }
}

macro_rules! d_str_getter {
    ($method: ident) => {
        d_getter!($method, String, string_owned);
    }
}

macro_rules! d_bool_getter {
    ($method: ident) => {
        d_getter!($method, bool, bool);
    }
}

macro_rules! d_int_getter {
    ($method: ident) => {
        d_getter!($method, i64, int);
    }
}

macro_rules! d_str_setter {
    ($rmethod: ident, $apimethod: ident) => {
        prim_setter!("d.", $rmethod, $apimethod, &str);
    }
}

#[derive(Debug)]
pub struct DownloadInner {
    sha1_hex: String,
    server: Server,
}

#[derive(Clone, Debug)]
pub struct Download {
    inner: Arc<DownloadInner>,
}

impl Download {
    pub(crate) fn from_value(server: Server, val: &Value) -> Result<Self> {
        let s = value_conversion::string(val)?;
        Ok(Self { inner: Arc::new(DownloadInner { server, sha1_hex: s.to_owned() }) })
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.server.endpoint()
    }

    pub fn sha1_hex(&self) -> &str {
        &self.inner.sha1_hex
    }

    d_str_getter!(base_filename);
    d_str_getter!(base_path);
    d_str_getter!(directory);
    d_str_getter!(directory_base);
    d_str_getter!(message);
    d_str_getter!(name);
    d_str_getter!(loaded_file);
    d_str_getter!(tied_to_file);

    d_str_setter!(set_directory, directory);
    d_str_setter!(set_directory_base, directory_base);

    d_bool_getter!(state);
    d_bool_getter!(is_open);
    d_bool_getter!(is_closed);

    d_int_getter!(tracker_size);

    pub fn trackers(&self) -> Result<Vec<DownloadTracker>> {
        let num = self.tracker_size()?;
        Ok((0..num).map(|i| DownloadTracker::new(self.clone(), i)).collect())
    }
}

unsafe impl Send for Download {}
unsafe impl Sync for Download {}

impl From<&Download> for Value {
    fn from(dl: &Download) -> Self {
        Value::String(dl.inner.sha1_hex.to_owned())
    }
}
