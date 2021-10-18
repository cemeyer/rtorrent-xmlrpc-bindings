/*! Torrent downloads

This module defines the [`Download`] type and support code.

[`Download`]: crate::Download
!*/

use crate::macros::*;
use crate::{value_conversion, Error, File, Peer, Result, Server, Tracker};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! d_getter {
    ($(#[$meta:meta])* $method: ident, $result: ty, $conv: ident) => {
        prim_getter!($(#[$meta])* "d.", $method, $result, $conv);
    }
}

macro_rules! d_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, String, string_owned);
    }
}

macro_rules! d_bool_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, bool, bool);
    }
}

macro_rules! d_f1000_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, f64, fraction1000);
    }
}

macro_rules! d_int_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, i64, int);
    }
}

macro_rules! d_int_getter_named {
    ($(#[$meta:meta])* $method: ident, $apimethod: literal) => {
        prim_getter_named!($(#[$meta])* "d.", $method, i64, int, $apimethod);
    }
}

macro_rules! d_str_setter {
    ($(#[$meta:meta])* $rmethod: ident, $apimethod: ident) => {
        prim_setter!($(#[$meta])* "d.", $rmethod, $apimethod, &str);
    }
}

#[derive(Debug)]
pub(crate) struct DownloadInner {
    sha1_hex: String,
    server: Server,
}

/// `Download` represents a loaded torrent
///
/// Accessors on `Download` correspond to the `d.*` rtorrent APIs.
///
/// # Examples
///
/// Enumerating downloads on an rtorrent instance:
///
/// ```rust
/// use rtorrent_xmlrpc_bindings as rtorrent;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
/// for download in my_handle.download_list()? {
///     println!("Download: {}", download.name()?);
/// }
/// ```
///
/// Introspecting downloads:
///
/// ```rust
/// fn print_download_info(dl: Download) -> Result<(), rtorrent::Error> {
///     println!("{}: {} MB. Ratio: {}",
///         dl.name()?,
///         dl.size_bytes()? / 1000_000,
///         dl.ratio()?);
///     Ok(())
/// }
/// ```
///
/// Enumerating [`File`]s associated with a torrent (torrents contain one or more individual files):
///
/// ```rust
/// let dl: Download = ...;
/// for file in dl.files()? {
///     print_file_info(file)?;
/// }
/// ```
///
/// Enumerating [`Tracker`]s associated with a torrent (torrents use one or more tracker(s) to
/// locate peers in the swarm):
///
/// ```rust
/// let dl: Download = ...;
/// for tracker in dl.trackers()? {
///     print_tracker_info(tracker)?;
/// }
/// ```
///
/// Enumerating [`Peer`]s in the swarm associated with a torrent:
///
/// ```rust
/// let dl: Download = ...;
/// for peer in dl.peers()? {
///     print_peer_info(peer)?;
/// }
/// ```
///
/// [`File`]: crate::File
/// [`Peer`]: crate::Peer
/// [`Tracker`]: crate::Tracker
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

    /// Get the "infohash" of this download (hex string).
    pub fn sha1_hex(&self) -> &str {
        &self.inner.sha1_hex
    }

    /// Get a list of active peers associated with this download.
    pub fn peers(&self) -> Result<Vec<Peer>> {
        let raw_list = Request::new("p.multicall")
            .arg(self.sha1_hex())
            .arg("")
            .arg(Value::Array(vec!["p.id=".into()]))
            .call_url(self.endpoint())?;
        let list = value_conversion::list(&raw_list)?
            .iter()
            .map(|ll| {
                let peerhash = value_conversion::list(ll)?
                    .get(0)
                    .ok_or(Error::UnexpectedStructure(
                            format!("expected non-empty inner list, got {:?}", ll)
                        ))?;
                let peerhash = value_conversion::string(peerhash)?;
                Ok(Peer::new(self.clone(), peerhash))
            })
            .collect();
        list
    }

    /// Get a list of files associated with this download.
    pub fn files(&self) -> Result<Vec<File>> {
        let num = self.size_files()?;
        Ok((0..num).map(|i| File::new(self.clone(), i)).collect())
    }

    /// Get a list of trackers associated with this download.
    pub fn trackers(&self) -> Result<Vec<Tracker>> {
        let num = self.tracker_size()?;
        Ok((0..num).map(|i| Tracker::new(self.clone(), i)).collect())
    }

    d_str_getter!(base_filename);
    d_str_getter!(base_path);
    d_str_getter!(directory);
    d_str_getter!(directory_base);
    d_str_setter!(set_directory, directory);
    d_str_setter!(set_directory_base, directory_base);

    d_int_getter!(
        /// The item's chunk size, in bytes (also known as "piece size").
        chunk_size);

    d_bool_getter!(
        /// Is the download complete (100%)?
        complete);
    d_bool_getter!(
        /// Is the download incomplete (less than 100%)?
        incomplete);
    d_int_getter!(
        /// The number of completed bytes.
        completed_bytes);
    d_int_getter!(
        /// The number of completed chunks (pieces).
        completed_chunks);

    d_int_getter_named!(
        /// Get the download rate.
        down_rate, "down.rate");
    d_int_getter_named!(
        /// Get the download total (bytes).
        down_total, "down.total");

    d_bool_getter!(is_active);
    d_bool_getter!(is_open);
    d_bool_getter!(is_closed);

    d_str_getter!(
        /// The metafile from which this download was created.
        loaded_file);

    d_str_getter!(
        /// Unstructured error messages, either generated by rtorrent, or forwarded from the
        /// tracker.
        message);

    d_str_getter!(
        /// Get the name of the torrent.
        name);

    d_f1000_getter!(
        /// Get the upload/download ratio for this download.
        ratio);

    d_int_getter!(
        /// Get the size, in bytes, of the torrent contents.
        size_bytes);
    d_int_getter!(
        /// Get the number of files associated with this download.
        size_files);
    d_bool_getter!(
        /// Get the state (`false` is stopped).
        state);

    d_str_getter!(
        /// Starts as the file the download was initially created from.
        tied_to_file);

    d_int_getter!(
        /// Get the number of trackers associated with this download.
        tracker_size);

    d_int_getter_named!(
        /// Get the upload rate.
        up_rate, "up.rate");
    d_int_getter_named!(
        /// Get the upload total (bytes).
        up_total, "up.total");
}

unsafe impl Send for Download {}
unsafe impl Sync for Download {}

impl From<&Download> for Value {
    fn from(dl: &Download) -> Self {
        Value::String(dl.inner.sha1_hex.to_owned())
    }
}
