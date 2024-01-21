use crate as hwnd0;
use hwnd0::*;
use core::ffi::c_int;
use core::fmt::{self, Debug, Formatter};
use core::num::{NonZeroIsize, NonZeroUsize};
use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
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
impl From<()> for HWND { fn from(_: ()) -> Self { Self::from_usize(0) } }
impl From<NonNullHWND> for HWND { fn from(hwnd: NonNullHWND) -> Self { Self::from_usize(hwnd.to_usize()) } }
impl From<Option<NonNullHWND>> for HWND { fn from(hwnd: Option<NonNullHWND>) -> Self { Self::from_usize(hwnd.map_or(0, |hwnd| hwnd.to_usize())) } }

/// # Constants
#[allow(dead_code)] impl HWND {
    #[inline(always)] const fn from_constant(hwnd: isize) -> Self { Self(hwnd as _) }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)\]
    /// `HWND_MESSAGE = -3`
    ///
    /// Can be passed to [`CreateWindowW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww) etc. as `hWndParent` to create a message-only window, which:
    /// *   Is not visible
    /// *   Has no Z-order
    /// *   Cannot be enumerated
    /// *   Does not receive broadcast messages
    pub const MESSAGE   : Self = Self::from_constant(-3);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#parameters)\]
    /// `HWND_NOTOPMOST = -2`
    ///
    /// Can be passed to [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) etc. as
    /// `hWndInsertAfter` to (conditionally) place the window above all non-topmost windows (without *becoming* a topmost window.)
    pub const NOTOPMOST : Self = Self::from_constant(-2);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#parameters)\]
    /// `HWND_TOPMOST = -1`
    ///
    /// Can be passed to [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) etc. as
    /// `hWndInsertAfter` to (conditionally) place the window above all non-topmost windows (*becoming* a topmost window.)
    pub const TOPMOST   : Self = Self::from_constant(-1);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#parameters)\]
    /// `HWND_TOP = 0`
    ///
    /// Can be passed to [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) etc. as
    /// `hWndInsertAfter` to (conditionally) place the window above (all?) windows in (it's Z-category.)
    ///
    /// **N.B.:** `0` / `nullptr` / `HWND_TOP` / `HWND_DESKTOP` can have wildly different meanings depending on which API it is passed to:
    /// *   [`CreateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww) will interpret `0` as "make the desktop the parent window"
    /// *   [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints) will interpret `0` as "map points to/from screen coordinates"
    /// *   [`GetMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessage) will interpret `0` as "get messages for the current thread (including all windows)"
    /// *   [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) will interpret `0` as "place the window at the top of the Z order"
    pub const TOP       : Self = Self::from_constant(0);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints#parameters)\]
    /// `HWND_DESKTOP = 0`
    ///
    /// Can be passed to [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints#parameters) as
    /// `hHwndFrom` or `hHwndTo` to convert points to/from screen coordinates.
    ///
    /// **N.B.:** `0` / `nullptr` / `HWND_TOP` / `HWND_DESKTOP` can have wildly different meanings depending on which API it is passed to:
    /// *   [`CreateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww) will interpret `0` as "make the desktop the parent window"
    /// *   [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints) will interpret `0` as "map points to/from screen coordinates"
    /// *   [`GetMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessage) will interpret `0` as "get messages for the current thread (including all windows)"
    /// *   [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) will interpret `0` as "place the window at the top of the Z order"
    pub const DESKTOP   : Self = Self::from_constant(0);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
    /// `0` / `nullptr`
    ///
    /// **N.B.:** `0` / `nullptr` / `HWND_TOP` / `HWND_DESKTOP` can have wildly different meanings depending on which API it is passed to:
    /// *   [`CreateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww) will interpret `0` as "make the desktop the parent window"
    /// *   [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints) will interpret `0` as "map points to/from screen coordinates"
    /// *   [`GetMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessage) will interpret `0` as "get messages for the current thread (including all windows)"
    /// *   [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) will interpret `0` as "place the window at the top of the Z order"
    pub const NULL      : Self = Self::from_constant(0);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos#parameters)\]
    /// `HWND_BOTTOM = 1`
    ///
    /// Can be passed to [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos) etc. as
    /// `hWndInsertAfter` to (conditionally) place the window below all other windows (losing *topmost* status if it had it.)
    pub const BOTTOM    : Self = Self::from_constant(1);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage#parameters)\]
    /// `HWND_BROADCAST = 0xFFFF`
    ///
    /// Can be passed to [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage#parameters) etc. as
    /// `hWnd` to send a message to all top-level windows in the system of lesser or equal integrity level
    /// (including disabled, invisible, overlapped, and pop-up windows &mdash; *not* including *child* windows.)
    pub const BROADCAST : Self = Self::from_constant(0xFFFF);

    /// `self == HWND::NULL`
    #[inline(always)] pub fn is_null(self) -> bool { self == HWND::NULL }

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

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
/// GetWindowsLongPtrW
pub(crate) fn get_window_long_ptr_w(hwnd: impl Into<HWND>, index: impl Into<c_int>) -> isize {
    #[link(name = "user32")] extern "system" {
        #[cfg_attr(not(target_pointer_width = "64"), link_name = "GetWindowLongW")]
        fn GetWindowLongPtrW(hwnd: HWND, index: c_int) -> isize;
    }
    unsafe { GetWindowLongPtrW(hwnd.into(), index.into()) }
}

pub(crate) const GWLP_HINSTANCE : c_int = -6;
