/*! Torrent files

This module defines the [`File`] type and support code.

[`File`]: crate::File
!*/

use crate::macros::*;
use crate::{Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! f_int_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "f.", $method, i64);
    }
}

macro_rules! f_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "f.", $method, String);
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
/// Introspecting a file:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Download;
/// # use rtorrent::File;
/// # use rtorrent::Result;
/// fn print_file_info(file: File) -> Result<()> {
///     println!("{} MB: {}",
///         file.size_bytes()? / 1000_000,
///         file.path()?);
///     Ok(())
/// }
/// ```
///
/// Enumerating files associated with a download:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Download;
/// # use rtorrent::File;
/// # use rtorrent::Result;
/// # fn print_file_info(file: File) -> Result<()> {
/// #     println!("{} MB: {}",
/// #         file.size_bytes()? / 1000_000,
/// #         file.path()?);
/// #     Ok(())
/// # }
/// fn enum_files(dl: Download) -> Result<()> {
///     for file in dl.files()? {
///         print_file_info(file)?;
///     }
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
        Self {
            inner: Arc::new(FileInner { download, index }),
        }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    f_int_getter!(
        /// The number of completed chunks associated with this file (including chunks that only
        /// partially include this file).
        completed_chunks
    );
    f_str_getter!(
        /// Get the absolute path of this file.
        frozen_path
    );
    f_int_getter!(
        /// The offset (in bytes) of the file from the start of the torrent data.
        offset
    );
    f_str_getter!(
        /// Get the path of this file, relative to the download's base path.
        path
    );

    f_int_getter!(
        /// The priority of the file.
        ///
        /// * `0`: Off. Do not download.
        /// * `1`: Normal.
        /// * `2`: High. Prioritize this file's chunks over "Normal" files.
        priority
    );
    f_int_setter!(
        /// Set the priority of the file.  See [`File::priority`].
        set_priority,
        priority
    );

    f_int_getter!(
        /// Get the size of the file, in bytes.
        size_bytes
    );

    f_int_getter!(
        /// The number of chunks associated with this file (including chunks that only partially
        /// include this file).
        size_chunks
    );
}

unsafe impl Send for File {}
unsafe impl Sync for File {}

impl From<&File> for Value {
    fn from(file: &File) -> Self {
        Value::String(format!(
            "{}:f{}",
            &file.inner.download.sha1_hex(),
            file.inner.index
        ))
    }
}
