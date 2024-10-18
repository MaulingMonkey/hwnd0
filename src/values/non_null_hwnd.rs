use crate as hwnd0;
use hwnd0::*;
use core::ffi::c_void;
use core::fmt::{self, Debug, Formatter};
use core::num::{NonZeroIsize, NonZeroUsize};
use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
/// `__attribute__((nonnull)) HWND`
/// &mdash;
/// A weak handle to a "window" on this computer.
///
/// May or may not belong to the current thread.<br>
/// May or may not belong to the current *process*.<br>
/// May or may not have ever existed, or be valid.<br>
/// May or may not be currently or previously visible or on-screen.<br>
/// May or may not have access blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation).<br>
/// May be some kind of magic number or value, such as [`NonNullHWND::BROADCAST`] or [`NonNullHWND::MESSAGE`].<br>
/// May **not** be null (see [`HWND`] for a nullable alternative.)<br>
/// May be a top level application/main window.<br>
/// May be a tiny button.<br>
/// May be a [*message-only window*](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows), used only for callbacks.<br>
/// Recommended reading: [What Is a Window?](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)
///
///
///
/// ### Implemented Traits
/// * Construction: [`Clone`], [`Copy`], ~~[`Default`]~~, <code>[From]&lt;...&gt;</code>, <code>[TryFrom]&lt;...&gt;</code>
/// * Comparison: [`Eq`], [`PartialEq`], [`Ord`], [`PartialOrd`], [`Hash`]
/// * Debug: [`Debug`]
/// * Thread Safety: [`Send`], [`Sync`]
///
#[doc = include_str!("data-race-safety.md")]
///
#[doc = include_str!("non_null_hwnd.conversion.md")]
///
#[doc = include_str!("provenance.md")]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct NonNullHWND(NonNull<c_void>);
// N.B.: ntdef.h defines HWND via `DECLARE_HANDLE(HWND);`.  This either resolves to `HANDLE` ≈ `void*` or `struct HWND__*` depending on `STRICT`.
// https://clang.llvm.org/docs/ControlFlowIntegrity.html#fsanitize-cfi-icall-generalize-pointers might be necessary to make things play nice...

impl Debug for NonNullHWND { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "0x{:08x}", self.0.as_ptr() as usize) } }
impl TryFrom<HWND> for NonNullHWND          { fn try_from(hwnd: HWND) -> Result<Self, ()> { Self::from_usize(hwnd.to_usize()).ok_or(()) } type Error = (); }
impl From<HWND> for Option<NonNullHWND>     { fn from(hwnd: HWND) -> Self { NonNullHWND::from_usize(hwnd.to_usize()) } }

unsafe impl Send for NonNullHWND {} // see data-race-safety.md for rambling details
unsafe impl Sync for NonNullHWND {} // see data-race-safety.md for rambling details

/// # Constants
#[allow(dead_code)] impl NonNullHWND {
    #[inline(always)] const fn from_constant(hwnd: isize) -> Self { match unsafe { core::mem::transmute::<isize, Option<NonNull<c_void>>>(hwnd as _) } { Some(v) => Self(v), None => panic!("invalid constant") } }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows)\]
    /// `HWND_MESSAGE = -3`
    ///
    /// Can be passed to [`CreateWindowW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindoww) etc. as `hWndParent` to create a message-only window, which:
    /// *   Is not visible
    /// *   Has no Z-order
    /// *   Cannot be enumerated
    /// *   Does not receive broadcast messages
    pub const MESSAGE   : Self = Self::from_constant(-3);

    //pub const NOTOPMOST : Self = Self::from_constant(-2); // SetWindowPos param - not all values can be NonNullHWND, so expose none of them as NonNullHWND.
    //pub const TOPMOST   : Self = Self::from_constant(-1); // SetWindowPos param - not all values can be NonNullHWND, so expose none of them as NonNullHWND.
    //pub const TOP       : Self = Self::from_constant(0); // nullptr - cannot be a NonNullHWND. SetWindowPos param - not all values can be NonNullHWND, so expose none of them as NonNullHWND.
    //pub const DESKTOP   : Self = Self::from_constant(0); // nullptr - cannot be a NonNullHWND.
    //pub const NULL      : Self = Self::from_constant(0); // nullptr - cannot be a NonNullHWND.
    //pub const BOTTOM    : Self = Self::from_constant(1); // SetWindowPos param - not all values can be NonNullHWND, so expose none of them as NonNullHWND.

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage#parameters)\]
    /// `HWND_BROADCAST = 0xFFFF`
    ///
    /// Can be passed to [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage#parameters) etc. as
    /// `hWnd` to send a message to all top-level windows in the system of lesser or equal integrity level
    /// (including disabled, invisible, overlapped, and pop-up windows &mdash; *not* including *child* windows.)
    pub const BROADCAST : Self = Self::from_constant(0xFFFF);

    #[inline(always)] pub(crate) fn from_isize         (hwnd: isize         ) -> Option<Self> { Some(Self(NonNull::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_usize         (hwnd: usize         ) -> Option<Self> { Some(Self(NonNull::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_ptr<T>        (hwnd: *mut    T     ) -> Option<Self> { Some(Self(NonNull::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_non_null<T>   (hwnd: NonNull<T>    ) ->        Self  { Self(unsafe { NonNull::new_unchecked(hwnd.as_ptr() as _) }) }
    #[inline(always)] pub(crate) fn from_nz_isize      (hwnd: NonZeroIsize  ) ->        Self  { Self(unsafe { NonNull::new_unchecked(hwnd.get() as _) }) }
    #[inline(always)] pub(crate) fn from_nz_usize      (hwnd: NonZeroUsize  ) ->        Self  { Self(unsafe { NonNull::new_unchecked(hwnd.get() as _) }) }

    #[inline(always)] pub(crate) fn to_isize           (self) -> isize         { self.0.as_ptr() as _ }
    #[inline(always)] pub(crate) fn to_usize           (self) -> usize         { self.0.as_ptr() as _ }
    #[inline(always)] pub(crate) fn to_ptr<T>          (self) -> *mut    T     { self.0.as_ptr() as _ }
    #[inline(always)] pub(crate) fn to_non_null<T>     (self) -> NonNull<T>    { unsafe {      NonNull::new_unchecked(self.0.as_ptr() as _) } }
    #[inline(always)] pub(crate) fn to_nz_isize        (self) -> NonZeroIsize  { unsafe { NonZeroIsize::new_unchecked(self.0.as_ptr() as _) } }
    #[inline(always)] pub(crate) fn to_nz_usize        (self) -> NonZeroUsize  { unsafe { NonZeroUsize::new_unchecked(self.0.as_ptr() as _) } }
}
