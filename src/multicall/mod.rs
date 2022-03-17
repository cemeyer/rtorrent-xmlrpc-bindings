//! Rtorrent multicalls
//!
//! It can be more efficient to query multiple items at a time.  Rtorrent's XMLRPC API exposes an
//! interface for this called "multicalls."  Multicalls allow a single XMLRPC invocation to call
//! the same accessor across many items:
//!
//! * [`d`]: All *downloads* (torrents) in a "view"
//! * [`f`]: All *files* in a download
//! * [`p`]: All swarm *peers* associated with a download
//! * [`t`]: All *trackers* associated with a download
//!
//! See the corresponding module documentation for multicall documentation specific to that kind of
//! query.

#![allow(dead_code)]

mod ops;
mod raw_impl;

mod raw {
    use super::raw_impl;

    use crate::{value_conversion::{self, TryFromValue}, Error, Result};
    use std::marker::PhantomData;
    use xmlrpc::Value;

    pub(crate) use raw_impl::MultiBuilder;
    use raw_impl::MultiBuilderInternal;

    raw_impl::define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    raw_impl::define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    raw_impl::define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    raw_impl::define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
        raw_impl::define_builder!(MultiBuilder4, MultiBuilder5, phantom_a A, phantom_b B, phantom_c C , phantom_d D | phantom_e E);
    raw_impl::define_builder!(MultiBuilder5, MultiBuilder6, phantom_a A, phantom_b B, phantom_c C , phantom_d D , phantom_e E | phantom_g G);
    raw_impl::define_builder!(MultiBuilder6, MultiBuilder7, phantom_a A, phantom_b B, phantom_c C , phantom_d D , phantom_e E, phantom_g G | phantom_h H);
    raw_impl::define_builder!(MultiBuilder7, MultiBuilder8, phantom_a A, phantom_b B, phantom_c C , phantom_d D , phantom_e E, phantom_g G , phantom_h H | phantom_i I);
    raw_impl::define_builder!(MultiBuilder8, MultiBuilder9, phantom_a A, phantom_b B, phantom_c C , phantom_d D , phantom_e E, phantom_g G , phantom_h H , phantom_i I | phantom_j J);
}

/// The `d` module builds multicalls over `Download`s
///
/// These correspond to the `d.*` rtorrent APIs, via the `d.multicall2` API.
///
/// Use the [`d::MultiBuilder`] type to construct queries, and invoke the multicall with the
/// `invoke` API.  For example:
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::d;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// d::MultiBuilder::new(&my_handle, "default")
///     .call(d::NAME)
///     .call(d::RATIO)
///     .call(d::SIZE_BYTES)
///     .invoke()?
///     .iter()
///     .for_each(|(name, ratio, bytes)| {
///         println!("{}: {} bytes, {} ratio", name, bytes, ratio);
///     });
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// [`d::MultiBuilder`]: crate::multicall::d::MultiBuilder
pub mod d {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::d::*;

    define_builder!(
        /// `MultiBuilder1` represents a single-column query over all `Download`s in a view
        MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(
        /// `MultiBuilder2` represents a two-column query over all `Download`s in a view
        MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(
        /// `MultiBuilder3` represents a three-column query over all `Download`s in a view
        MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(
        /// `MultiBuilder4` represents a four-column query over all `Download`s in a view
        MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
        define_builder!(
        /// `MultiBuilder5` represents a four-column query over all `Download`s in a view
        MultiBuilder4, MultiBuilder5, phantom_a A, phantom_b B, phantom_c C , phantom_d D | phantom_e E);
    define_builder!(
        /// `MultiBuilder6` represents a four-column query over all `Download`s in a view
        MultiBuilder5, MultiBuilder6, phantom_a A, phantom_b B, phantom_c C , phantom_d D, phantom_e E | phantom_g G);
    define_builder!(
        /// `MultiBuilder7` represents a four-column query over all `Download`s in a view
        MultiBuilder6, MultiBuilder7, phantom_a A, phantom_b B, phantom_c C , phantom_d D, phantom_e E, phantom_g G | phantom_h H);
    define_builder!(
        /// `MultiBuilder8` represents a four-column query over all `Download`s in a view
        MultiBuilder7, MultiBuilder8, phantom_a A, phantom_b B, phantom_c C , phantom_d D, phantom_e E, phantom_g G , phantom_h H | phantom_i I);
    define_builder!(
        /// `MultiBuilder9` represents a four-column query over all `Download`s in a view
        MultiBuilder8, MultiBuilder9, phantom_a A, phantom_b B, phantom_c C , phantom_d D, phantom_e E, phantom_g G , phantom_h H , phantom_i I | phantom_j J);
}

/// The `f` module builds multicalls over `File`s in a `Download`
///
/// These correspond to the `f.*` rtorrent APIs, via the `f.multicall` API.
///
/// Use the [`f::MultiBuilder`] type to construct queries, and invoke the multicall with the
/// `invoke` API.  For example:
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::f;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     let name = download.name()?;
///     println!("{}: {:?}",
///         name,
///         f::MultiBuilder::new(&my_handle, download.sha1_hex(), None)
///             .call(f::PATH)
///             .invoke()?);
///     break;
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// [`f::MultiBuilder`]: crate::multicall::f::MultiBuilder
pub mod f {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::f::*;

    define_builder!(
        /// `MultiBuilder1` represents a single-column query over all `File`s in a `Download`
        MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(
        /// `MultiBuilder2` represents a two-column query over all `File`s in a `Download`
        MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(
        /// `MultiBuilder3` represents a three-column query over all `File`s in a `Download`
        MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(
        /// `MultiBuilder4` represents a four-column query over all `File`s in a `Download`
        MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
    define_builder!(
        /// `MultiBuilder5` represents a four-column query over all `File`s in a `Download`
        MultiBuilder4, MultiBuilder5, phantom_a A, phantom_b B, phantom_c C , phantom_d D | phantom_e E);
}

/// The `p` module builds multicalls over `Peer`s on a `Download`
///
/// These correspond to the `p.*` rtorrent APIs, via the `p.multicall` API.
///
/// Use the [`p::MultiBuilder`] type to construct queries, and invoke the multicall with the
/// `invoke` API.  For example:
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::p;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     println!("{}:", download.name()?);
///     p::MultiBuilder::new(&my_handle, download.sha1_hex())
///         .call(p::ID)
///         .call(p::ADDRESS)
///         .invoke()?
///         .iter()
///         .for_each(|(id, addr)| {
///             println!("  {}: {}", id, addr);
///         });
///     break;
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// [`p::MultiBuilder`]: crate::multicall::p::MultiBuilder
pub mod p {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::p::*;

    define_builder!(
        /// `MultiBuilder1` represents a single-column query over all swarm `Peers` associated with
        /// a `Download`
        MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(
        /// `MultiBuilder2` represents a two-column query over all swarm `Peers` associated with a
        /// `Download`
        MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(
        /// `MultiBuilder3` represents a three-column query over all swarm `Peers` associated with
        /// a `Download`
        MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(
        /// `MultiBuilder4` represents a four-column query over all swarm `Peers` associated with a
        /// `Download`
        MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
    define_builder!(
        /// `MultiBuilder5` represents a four-column query over all swarm `Peers` associated with a
        /// `Download`
        MultiBuilder4, MultiBuilder5, phantom_a A, phantom_b B, phantom_c C , phantom_d D | phantom_e E);
    define_builder!(
        /// `MultiBuilder6` represents a four-column query over all swarm `Peers` associated with a
        /// `Download`
        MultiBuilder5, MultiBuilder6, phantom_a A, phantom_b B, phantom_c C , phantom_d D, phantom_e E | phantom_f F);

}

/// The `t` module builds multicalls over `Trackers`s associated with a `Download`
///
/// These correspond to the `t.*` rtorrent APIs, via the `t.multicall` API.
///
/// Use the [`t::MultiBuilder`] type to construct queries, and invoke the multicall with the
/// `invoke` API.  For example:
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::t;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// for download in my_handle.download_list()? {
///     let name = download.name()?;
///     let tracker_urls = t::MultiBuilder::new(&my_handle, download.sha1_hex())
///         .call(t::URL)
///         .invoke()?
///         .into_iter()
///         .map(|(addr,)| addr)
///         .collect::<Vec<_>>();
///     println!("{}: URLs: {:?}", name, tracker_urls);
///     break;
/// }
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// [`t::MultiBuilder`]: crate::multicall::t::MultiBuilder
pub mod t {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::t::*;

    define_builder!(
        /// `MultiBuilder1` represents a single-column query over all `Tracker`s for a `Download`
        MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(
        /// `MultiBuilder2` represents a two-column query over all `Tracker`s for a `Download`
        MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(
        /// `MultiBuilder3` represents a three-column query over all `Tracker`s for a `Download`
        MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(
        /// `MultiBuilder4` represents a four-column query over all `Tracker`s for a `Download`
        MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}
