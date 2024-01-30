/*! Torrent trackers

This module defines the [`Tracker`] type and support code.

[`Tracker`]: crate::Tracker
!*/

use crate::macros::*;
use crate::{Download, Result};
use std::sync::Arc;
use xmlrpc::{Request, Value};

macro_rules! t_str_getter {
    ($(#[$meta:meta])* $method: ident) => {
        prim_getter!($(#[$meta])* "t.", $method, String);
    }
}

#[derive(Debug)]
pub(crate) struct TrackerInner {
    download: Download,
    index: i64,
}

/// A `Tracker` associated with a [`Download`]
///
/// Accessors on `Tracker` correspond to the `t.*` rtorrent APIs.
///
/// # Examples
///
/// Introspecting a tracker:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Tracker;
/// # use rtorrent::Result;
/// fn print_tracker_info(tracker: Tracker) -> Result<()> {
///     println!("Tracker URL: {}", tracker.url()?);
///     Ok(())
/// }
/// ```
///
/// Enumerating the trackers associated with a [`Download`]:
///
/// ```no_run
/// # use rtorrent_xmlrpc_bindings as rtorrent;
/// # use rtorrent::Download;
/// # use rtorrent::Tracker;
/// # use rtorrent::Result;
/// # fn print_tracker_info(tracker: Tracker) -> Result<()> {
/// #     println!("Tracker URL: {}", tracker.url()?);
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
/// [`Download`]: crate::Download
#[derive(Clone, Debug)]
pub struct Tracker {
    inner: Arc<TrackerInner>,
}

impl Tracker {
    pub(crate) fn new(download: Download, index: i64) -> Self {
        Self {
            inner: Arc::new(TrackerInner { download, index }),
        }
    }

    #[inline]
    pub(crate) fn endpoint(&self) -> &str {
        self.inner.download.endpoint()
    }

    t_str_getter!(
        /// Get the URL of the tracker.
        url
    );
}

unsafe impl Send for Tracker {}
unsafe impl Sync for Tracker {}

impl From<&Tracker> for Value {
    fn from(tracker: &Tracker) -> Self {
        Value::String(format!(
            "{}:t{}",
            &tracker.inner.download.sha1_hex(),
            tracker.inner.index
        ))
    }
}
