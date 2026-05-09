#![allow(unused_imports)]
use super::*;

/// [`RistPeer`] is not a wrapper of [`librist_sys::rist_peer`] but a id type
/// without [`Drop`] implementation
/// because it cannot be safely dropped without access to the corresponding
/// [`RistCtx`].
///
/// [`RistPeer`] does not have any instance of or pointer to [`RistCtx`]
/// for the following reasons.
/// 1. According to the C library comment, [`librist_sys::rist_ctx`] may have
/// multiple [`librist_sys::rist_peer`] instances, so moving the ownership
/// of [`RistCtx`] restricts the capability of original library.
/// 2. [`std::future::AsyncDrop`] is still nightly only feature and sharing
/// mutable ownership over [`std::sync::Mutex`] is not performant.
///
/// [`RistCtx`] is responsible for calling [`librist_sys::rist_peer_destroy`].
pub struct RistPeer {
    pub(crate) librist_rs_ctx_id: u128,
    pub(crate) librist_rs_peer_key: usize,
    pub(crate) librist_rs_peer_id: u128,
}
