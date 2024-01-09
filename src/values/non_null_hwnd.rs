use crate as hwnd0;
use hwnd0::*;
use core::fmt::{self, Debug, Formatter};
use core::num::{NonZeroIsize, NonZeroUsize};
use core::ptr::NonNull;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
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
#[doc = include_str!("non_null_hwnd.conversion.md")]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct NonNullHWND(NonZeroUsize);
impl Debug for NonNullHWND { fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "0x{:08x}", self.0.get()) } }
impl TryFrom<HWND> for NonNullHWND          { fn try_from(hwnd: HWND) -> Result<Self, ()> { Self::from_usize(hwnd.to_usize()).ok_or(()) } type Error = (); }
impl From<HWND> for Option<NonNullHWND>     { fn from(hwnd: HWND) -> Self { NonNullHWND::from_usize(hwnd.to_usize()) } }

#[allow(dead_code)] impl NonNullHWND {
    #[inline(always)] const fn from_constant(hwnd: isize) -> Self { match NonZeroUsize::new(hwnd as _) { Some(v) => Self(v), None => panic!("invalid constant") } }
    pub const MESSAGE   : Self = Self::from_constant(-3);
    pub const NOTOPMOST : Self = Self::from_constant(-2);
    pub const TOPMOST   : Self = Self::from_constant(-1);
    //pub const TOP       : Self = Self::from_constant(0);
    //pub const DESKTOP   : Self = Self::from_constant(0);
    //pub const NULL      : Self = Self::from_constant(0);
    pub const BOTTOM    : Self = Self::from_constant(1);
    pub const BROADCAST : Self = Self::from_constant(0xFFFF);

    #[inline(always)] pub(crate) fn from_isize         (hwnd: isize         ) -> Option<Self> { Some(Self(NonZeroUsize::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_usize         (hwnd: usize         ) -> Option<Self> { Some(Self(NonZeroUsize::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_ptr<T>        (hwnd: *mut    T     ) -> Option<Self> { Some(Self(NonZeroUsize::new(hwnd as _)?)) }
    #[inline(always)] pub(crate) fn from_non_null<T>   (hwnd: NonNull<T>    ) ->        Self  { Self(unsafe { NonZeroUsize::new_unchecked(hwnd.as_ptr() as _) }) }
    #[inline(always)] pub(crate) fn from_nz_isize      (hwnd: NonZeroIsize  ) ->        Self  { Self(unsafe { NonZeroUsize::new_unchecked(hwnd.get() as _) }) }
    #[inline(always)] pub(crate) fn from_nz_usize      (hwnd: NonZeroUsize  ) ->        Self  { Self(unsafe { NonZeroUsize::new_unchecked(hwnd.get() as _) }) }

    #[inline(always)] pub(crate) fn to_isize           (self) -> isize         { self.0.get() as _ }
    #[inline(always)] pub(crate) fn to_usize           (self) -> usize         { self.0.get() as _ }
    #[inline(always)] pub(crate) fn to_ptr<T>          (self) -> *mut    T     { self.0.get() as _ }
    #[inline(always)] pub(crate) fn to_non_null<T>     (self) -> NonNull<T>    { unsafe {      NonNull::new_unchecked(self.0.get() as _) } }
    #[inline(always)] pub(crate) fn to_nz_isize        (self) -> NonZeroIsize  { unsafe { NonZeroIsize::new_unchecked(self.0.get() as _) } }
    #[inline(always)] pub(crate) fn to_nz_usize        (self) -> NonZeroUsize  { unsafe { NonZeroUsize::new_unchecked(self.0.get() as _) } }
}