//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use core::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Deref,
};

#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;

#[cfg(not(target_has_atomic = "ptr"))]
use portable_atomic_util::Arc;

/// Much like a [`Cow`](std::borrow::Cow), but owned values are [`Arc`]-ed to make clones cheap.
/// This should be used for values that are cloned for use across threads and change rarely (if
/// ever).
///
/// This also makes an opinionated tradeoff by adding a [`CowArc::Static`] and implementing
/// `From<&'static T>` instead of `From<'a T>`. This preserves the static context and prevents
/// conversion to [`CowArc::Owned`] in cases where a reference is known to be static. This is an
/// optimization that prevents allocations and atomic ref-counting.
///
/// This means that static references should prefer [`CowArc::from`] or [`CowArc::Static`] and
/// non-static references must use [`CowArc::Borrowed`].
pub enum CowArc<'a, T: ?Sized + 'static> {
    /// A borrowed value.
    Borrowed(&'a T),
    /// A static value reference.
    ///
    /// This exists to avoid conversion to [`CowArc::Owned`] in cases where a reference is
    /// known to be static. This is an optimization that prevents allocations and atomic
    /// ref-counting.
    Static(&'static T),
    /// An owned [`Arc`]-ed value.
    Owned(Arc<T>),
}

impl<T: ?Sized + 'static> CowArc<'static, T> {
    /// Creates a new [`CowArc::Owned`] from a value.
    ///
    /// This is simply a convenience method;
    /// the value will be wrapped in an [`Arc`].
    ///
    /// Note that `T` must be [`Sized`]: use the enum constructor directly if `T` is unsized.
    pub fn new_owned(value: T) -> Self
    where
        T: Sized,
    {
        CowArc::Owned(Arc::new(value))
    }

    /// Creates a new [`CowArc::Owned`] from an [`Arc`]-like value.
    ///
    /// The [`Arc`] will be moved into the [`CowArc`].
    pub fn new_owned_from_arc(value: impl Into<Arc<T>>) -> Self {
        CowArc::Owned(value.into())
    }
}

impl<T: ?Sized> CowArc<'static, T> {
    /// Indicates this [`CowArc`] should have a static lifetime.
    ///
    /// This ensures if this was created with a value `Borrowed(&'static T)`, it is replaced with
    /// `Static(&'static T)`. It is only possible to call this method if `'a` is `'static`.
    /// This has no effect if this is `Owned(Arc<T>)`.
    #[inline]
    pub fn as_static(self) -> Self {
        match self {
            Self::Borrowed(value) | Self::Static(value) => Self::Static(value),
            Self::Owned(value) => Self::Owned(value),
        }
    }
}

impl<T: ?Sized> Deref for CowArc<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            CowArc::Borrowed(v) | CowArc::Static(v) => v,
            CowArc::Owned(v) => v,
        }
    }
}

impl<T: ?Sized> Borrow<T> for CowArc<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsRef<T> for CowArc<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized> CowArc<'a, T>
where
    &'a T: Into<Arc<T>>,
{
    /// Converts this into an "owned" value.
    ///
    /// If internally a value is borrowed, it will be cloned into an "owned [`Arc`]".
    /// If it is already a [`CowArc::Owned`] or a [`CowArc::Static`], it will remain unchanged.
    #[inline]
    pub fn into_owned(self) -> CowArc<'static, T> {
        match self {
            CowArc::Borrowed(value) => CowArc::Owned(value.into()),
            CowArc::Static(value) => CowArc::Static(value),
            CowArc::Owned(value) => CowArc::Owned(value),
        }
    }

    /// Clones into an owned [`CowArc<'static>`].
    ///
    /// If internally a value is borrowed, it will be cloned into an "owned [`Arc`]".
    /// If it is already a [`CowArc::Owned`] or [`CowArc::Static`], the value will be cloned.
    /// This is equivalent to `.clone().into_owned()`.
    #[inline]
    pub fn clone_owned(&self) -> CowArc<'static, T> {
        self.clone().into_owned()
    }
}

impl<T: ?Sized> Clone for CowArc<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(value) => Self::Borrowed(value),
            Self::Static(value) => Self::Static(value),
            Self::Owned(value) => Self::Owned(value.clone()),
        }
    }
}

impl<T: PartialEq + ?Sized> PartialEq for CowArc<'_, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T: PartialEq + ?Sized> Eq for CowArc<'_, T> {}

impl<T: Hash + ?Sized> Hash for CowArc<'_, T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<T: Debug + ?Sized> Debug for CowArc<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: Display + ?Sized> Display for CowArc<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.deref(), f)
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd for CowArc<'_, T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl Default for CowArc<'static, str> {
    fn default() -> Self {
        CowArc::Static(Default::default())
    }
}

#[cfg(feature = "std")]
// A shortcut, since `Path` does not implement `Default`.
impl Default for CowArc<'static, Path> {
    /// Returns an empty [`Path`], wrapped in [`CowArc::Static`].
    ///
    /// This is equivalent to `CowArc::Static(Path::new(""))`.
    fn default() -> Self {
        CowArc::Static(Path::new(""))
    }
}

impl<T: Ord + ?Sized> Ord for CowArc<'_, T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.deref().cmp(other.deref())
    }
}

#[cfg(feature = "std")]
impl From<PathBuf> for CowArc<'static, Path> {
    #[inline]
    fn from(value: PathBuf) -> Self {
        CowArc::Owned(value.into())
    }
}

#[cfg(feature = "std")]
impl From<&'static str> for CowArc<'static, Path> {
    #[inline]
    fn from(value: &'static str) -> Self {
        CowArc::Static(Path::new(value))
    }
}

impl From<String> for CowArc<'static, str> {
    #[inline]
    fn from(value: String) -> Self {
        CowArc::Owned(value.into())
    }
}

impl<'a> From<&'a String> for CowArc<'a, str> {
    #[inline]
    fn from(value: &'a String) -> Self {
        CowArc::Borrowed(value)
    }
}

impl<T: ?Sized> From<&'static T> for CowArc<'static, T> {
    #[inline]
    fn from(value: &'static T) -> Self {
        CowArc::Static(value)
    }
}

impl<T> From<Arc<T>> for CowArc<'static, T> {
    #[inline]
    fn from(value: Arc<T>) -> Self {
        CowArc::Owned(value)
    }
}
