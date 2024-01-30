/*! Torrent downloads

This module defines the [`Download`] type and support code.

[`Download`]: crate::Download
!*/

use crate::macros::*;
use crate::multicall::p;
use crate::{value_conversion, File, Peer, Result, Server, Tracker};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! d_getter {
    ($(#[$meta:meta])* $method: ident, $result: ty) => {
        prim_getter!($(#[$meta])* "d.", $method, $result);
    }
}

macro_rules! d_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, String);
    }
}

macro_rules! d_str_getter_named {
    ($(#[$meta:meta])* $method: ident, $apimethod: literal) => {
        prim_getter_named!($(#[$meta])* "d.", $method, String, $apimethod);
    }
}

macro_rules! d_bool_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, bool);
    }
}

macro_rules! d_f1000_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, f64);
    }
}

macro_rules! d_int_getter {
    ($(#[$meta:meta])* $method: ident) => {
        d_getter!($(#[$meta])* $method, i64);
    }
}

macro_rules! d_int_getter_named {
    ($(#[$meta:meta])* $method: ident, $apimethod: literal) => {
        prim_getter_named!($(#[$meta])* "d.", $method, i64, $apimethod);
    }
}

macro_rules! d_str_setter {
    ($(#[$meta:meta])* $rmethod: ident, $apimethod: ident) => {
        prim_setter!($(#[$meta])* "d.", $rmethod, $apimethod, &str);
    }
}

macro_rules! d_int_setter {
    ($(#[$meta:meta])* $rmethod: ident, $apimethod: ident) => {
        prim_setter!($(#[$meta])* "d.", $rmethod, $apimethod, i64);
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
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
/// for download in my_handle.download_list()? {
///     println!("Download: {}", download.name()?);
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// Introspecting downloads:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::{Download, Result};
///
/// fn print_download_info(dl: Download) -> Result<()> {
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
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Download;
/// # use rtorrent::Result;
/// # fn print_file_info(f: rtorrent::File) -> Result<()> {
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
/// Enumerating [`Tracker`]s associated with a torrent (torrents use one or more tracker(s) to
/// locate peers in the swarm):
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::{Download, Result};
/// # fn print_tracker_info(t: rtorrent::Tracker) -> Result<()> {
/// #     Ok(())
/// # }
/// fn enum_trackers(dl: Download) -> Result<()> {
///     for tracker in dl.trackers()? {
///         print_tracker_info(tracker)?;
///     }
///     Ok(())
/// }
/// ```
///
/// Enumerating [`Peer`]s in the swarm associated with a torrent:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::{Download, Result};
/// # fn print_peer_info(t: rtorrent::Peer) -> Result<()> {
/// #     Ok(())
/// # }
/// fn enum_peers(dl: Download) -> Result<()> {
///     for peer in dl.peers()? {
///         print_peer_info(peer)?;
///     }
///     Ok(())
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
    pub(crate) fn from_value(server: &Server, val: &Value) -> Result<Self> {
        let s = value_conversion::string(val)?;
        Ok(Self::from_hash(server, s))
    }

    /// Construct a Download representing the given infohash on the specified server.  This
    /// constructor does not validate that the infohash is valid or actually exists on the server.
    pub fn from_hash(server: &Server, hash: &str) -> Self {
        let server = server.clone();
        Self {
            inner: Arc::new(DownloadInner {
                server,
                sha1_hex: hash.to_owned(),
            }),
        }
    }

    pub fn execute(&self, request: Request) -> std::result::Result<Value, xmlrpc::Error> {
        self.inner.server.execute(request)
    }

    /// Get the "infohash" of this download (hex string).
    pub fn sha1_hex(&self) -> &str {
        &self.inner.sha1_hex
    }

    /// Get a list of active peers associated with this download.
    pub fn peers(&self) -> Result<Vec<Peer>> {
        p::MultiBuilder::new(&self.inner.server, self.sha1_hex())
            .call(p::ID)
            .invoke()?
            .into_iter()
            .map(|(id,)| Ok(Peer::new(self.clone(), &id)))
            .collect()
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
        chunk_size
    );

    d_bool_getter!(
        /// Is the download complete (100%)?
        complete
    );
    d_bool_getter!(
        /// Is the download incomplete (less than 100%)?
        incomplete
    );
    d_int_getter!(
        /// The number of completed bytes.
        completed_bytes
    );
    d_int_getter!(
        /// The number of completed chunks (pieces).
        completed_chunks
    );

    d_int_getter_named!(
        /// Get the download rate.
        down_rate,
        "down.rate"
    );
    d_int_getter_named!(
        /// Get the download total (bytes).
        down_total,
        "down.total"
    );

    d_bool_getter!(is_active);
    d_bool_getter!(is_open);
    d_bool_getter!(is_closed);

    d_bool_getter!(
        /// Starts the download.
        start
    );
    d_bool_getter!(
        /// Stops the download.
        stop
    );
    d_bool_getter!(
        /// Removes download from rtorrent's index of torrents, including associated session files.
        /// The data stored for the item is not touched.
        erase
    );

    d_bool_getter!(
        /// Cause the download to be hash checked.  The download is paused during hashing.
        check_hash
    );
    d_bool_getter!(
        /// Trigger a tracker announce (notify tracker of local status and potentially get
        /// information about new peers).
        tracker_announce
    );

    d_str_getter!(
        /// The metafile from which this download was created.
        loaded_file
    );

    d_str_getter!(
        /// Unstructured error messages, either generated by rtorrent, or forwarded from the
        /// tracker.
        message
    );

    d_str_getter!(
        /// Get the name of the torrent.
        name
    );
    d_str_getter!(
        /// Get the bitfield of the torrent.  The bitfield is a string of hex digits representing
        /// completeness.
        bitfield
    );
    d_f1000_getter!(
        /// Get the upload/download ratio for this download.
        ratio
    );
    d_int_setter!(
        /// Control the priority of the download.  Possible values are: 0, off; 1, low; 2, normal;
        /// and 3, high.
        priority_set,
        priority
    );
    d_int_getter!(
        /// Get the size, in bytes, of the torrent contents.
        size_bytes
    );
    d_int_getter!(
        /// Get the bytes left to verify and/or download.
        left_bytes
    );
    d_int_getter!(
        /// Get the number of files associated with this download.
        size_files
    );
    d_bool_getter!(
        /// Get the state (`false` is stopped).
        state
    );

    d_str_getter!(
        /// Starts as the file the download was initially created from.
        tied_to_file
    );

    d_int_getter!(
        /// Get the number of trackers associated with this download.
        tracker_size
    );

    d_int_getter_named!(
        /// Get the upload rate.
        up_rate,
        "up.rate"
    );
    d_int_getter_named!(
        /// Get the upload total (bytes).
        up_total,
        "up.total"
    );
    d_str_getter_named!(
        /// Get the group of a download.
        group_name,
        "group.name"
    );
    d_int_getter!(
        /// Get the 'creation date' field of a download (as a timestamp).
        creation_date
    );
    d_int_getter!(
        /// Get timestamp the torrent was loaded (or rtorrent was restarted, whichever is more
        /// recent).
        load_date
    );
}

unsafe impl Send for Download {}
unsafe impl Sync for Download {}

impl From<&Download> for Value {
    fn from(dl: &Download) -> Self {
        Value::String(dl.inner.sha1_hex.to_owned())
    }
}
