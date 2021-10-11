/*! Torrent files

This module defines the [`File`] type and support code.

[`File`]: crate::File
!*/

use crate::macros::*;
use crate::{value_conversion, Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! f_int_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "f.", $method, i64, int);
    }
}

macro_rules! f_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "f.", $method, String, string_owned);
    }
}

macro_rules! f_int_setter {
    ($(#[$meta:meta])* $set_method: ident, $apimethod: ident) => {
        prim_setter!($(#[$meta])* "f.", $set_method, $apimethod, i64);
    }
}

#[derive(Debug)]
pub(crate) struct FileInner {
    download: Download,
    index: i64,
}

/// A single `File` associated with a [`Download`]
///
/// Accessors on `File` correspond to the `f.*` rtorrent APIs.
///
/// # Examples
///
/// Enumerating files associated with a download:
///
/// ```rust
/// let dl: Download = ...;
/// for file in dl.files()? {
///     print_file_info(file)?;
/// }
/// ```
///
/// Introspecting a file:
///
/// ```rust
/// fn print_file_info(file: File) -> Result<(), rtorrent::Error> {
///     println!("{} MB: {}",
///         file.size_bytes()? / 1000_000,
///         file.path()?);
///     Ok(())
/// }
/// ```
///
/// [`Download`]: crate::Download
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
    f_int_getter!(
        /// Get the size of the file, in bytes.
        size_bytes);

    f_int_getter!(
        /// The number of chunks associated with this file (including chunks that only partially
        /// include this file).
        size_chunks);
    f_int_getter!(
        /// The number of completed chunks associated with this file (including chunks that only
        /// partially include this file).
        completed_chunks);

    f_int_getter!(
        /// The priority of the file.
        ///
        /// * `0`: Off. Do not download.
        /// * `1`: Normal.
        /// * `2`: High. Prioritize this file's chunks over "Normal" files.
        priority);
    f_int_setter!(
        /// Set the priority of the file.  See [`File::priority`].
        set_priority, priority);

    f_int_getter!(
        /// The offset (in bytes) of the file from the start of the torrent data.
        offset);
}

unsafe impl Send for File {}
unsafe impl Sync for File {}

impl From<&File> for Value {
    fn from(file: &File) -> Self {
        Value::String(format!("{}:f{}", &file.inner.download.sha1_hex(), file.inner.index))
    }
}
