use crate as hwnd0;
use hwnd0::*;
use core::ffi::c_int;
use core::fmt::{self, Debug, Formatter};
use core::num::{NonZeroIsize, NonZeroUsize};
use core::ptr::NonNull;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
/// `HWND`
/// &mdash;
/// A weak handle to a "window" on this computer.
///
/// May or may not belong to the current thread.<br>
/// May or may not belong to the current *process*.<br>
/// May or may not have ever existed, or be valid.<br>
/// May or may not be currently or previously visible or on-screen.<br>
/// May or may not have access blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation).<br>
/// May be some kind of magic number or value, such as [`HWND::BROADCAST`] or [`HWND::MESSAGE`].<br>
/// May be null (see [`NonNullHWND`] for a non-null alternative.)<br>
/// May be a top level application/main window.<br>
/// May be a tiny button.<br>
/// May be a [*message-only window*](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows), used only for callbacks.<br>
/// Recommended reading: [What Is a Window?](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)
///
///
///
/// ### Implemented Traits
/// * Construction: [`Clone`], [`Copy`], [`Default`], <code>[From]&lt;...&gt;</code>, <code>[TryFrom]&lt;...&gt;</code>
/// * Comparison: [`Eq`], [`PartialEq`], [`Ord`], [`PartialOrd`], [`Hash`]
/// * Debug: [`Debug`]
/// * Thread Safety: [`Send`], [`Sync`]
///
#[doc = include_str!("data-race-safety.md")]
///
#[doc = include_str!("hwnd.conversion.md")]
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct HWND(usize);
impl Debug for HWND { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "0x{:08x}", self.0) } }
impl From<NonNullHWND> for HWND { fn from(hwnd: NonNullHWND) -> Self { Self::from_usize(hwnd.to_usize()) } }
impl From<Option<NonNullHWND>> for HWND { fn from(hwnd: Option<NonNullHWND>) -> Self { Self::from_usize(hwnd.map_or(0, |hwnd| hwnd.to_usize())) } }

#[allow(dead_code)] impl HWND {
    #[inline(always)] const fn from_constant(hwnd: isize) -> Self { Self(hwnd as _) }
    pub const MESSAGE   : Self = Self::from_constant(-3);
    pub const NOTOPMOST : Self = Self::from_constant(-2);
    pub const TOPMOST   : Self = Self::from_constant(-1);
    pub const TOP       : Self = Self::from_constant(0);
    pub const DESKTOP   : Self = Self::from_constant(0);
    pub const NULL      : Self = Self::from_constant(0);
    pub const BOTTOM    : Self = Self::from_constant(1);
    pub const BROADCAST : Self = Self::from_constant(0xFFFF);

    #[inline(always)] pub(crate) fn from_isize         (hwnd: isize         ) -> Self { Self(hwnd as _) }
    #[inline(always)] pub(crate) fn from_usize         (hwnd: usize         ) -> Self { Self(hwnd as _) }
    #[inline(always)] pub(crate) fn from_ptr<T>        (hwnd: *mut    T     ) -> Self { Self(hwnd as _) }
    #[inline(always)] pub(crate) fn from_non_null<T>   (hwnd: NonNull<T>    ) -> Self { Self(hwnd.as_ptr() as _) }
    #[inline(always)] pub(crate) fn from_nz_isize      (hwnd: NonZeroIsize  ) -> Self { Self(hwnd.get() as _) }
    #[inline(always)] pub(crate) fn from_nz_usize      (hwnd: NonZeroUsize  ) -> Self { Self(hwnd.get() as _) }

    #[inline(always)] pub(crate) fn to_isize           (self) -> isize                { self.0 as _ }
    #[inline(always)] pub(crate) fn to_usize           (self) -> usize                { self.0 as _ }
    #[inline(always)] pub(crate) fn to_ptr<T>          (self) ->        *mut    T     { self.0 as _ }
    #[inline(always)] pub(crate) fn to_non_null<T>     (self) -> Option<NonNull<T>>   {      NonNull::new(self.0 as _) }
    #[inline(always)] pub(crate) fn to_nz_isize        (self) -> Option<NonZeroIsize> { NonZeroIsize::new(self.0 as _) }
    #[inline(always)] pub(crate) fn to_nz_usize        (self) -> Option<NonZeroUsize> { NonZeroUsize::new(self.0 as _) }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
/// GetWindowsLongPtrW
pub(crate) fn get_window_long_ptr_w(hwnd: impl Into<HWND>, index: impl Into<c_int>) -> isize {
    #[link(name = "user32")] extern "system" {
        #[cfg_attr(not(target_pointer_width = "64"), link(name = "GetWindowsLongW"))]
        fn GetWindowsLongW(hwnd: HWND, index: c_int) -> isize;
    }
    unsafe { GetWindowsLongW(hwnd.into(), index.into()) }
}

pub(crate) const GWLP_HINSTANCE : c_int = -6;
