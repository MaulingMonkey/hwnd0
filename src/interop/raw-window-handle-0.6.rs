use crate::*;
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use core::num::NonZeroIsize;

impl TryFrom<RawWindowHandle> for crate::HWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Win32(win32) = rwh  { Ok(Self::from_nz_isize(win32.hwnd)) }
        else                                        { Err(()) }
    }
}

impl TryFrom<RawWindowHandle> for crate::NonNullHWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Win32(win32) = rwh  { Ok(Self::from_nz_isize(win32.hwnd)) }
        else                                        { Err(()) }
    }
}

impl From<Win32WindowHandle> for crate::HWND        { fn from(win32: Win32WindowHandle) -> Self { Self::from_nz_isize(win32.hwnd) } }
impl From<Win32WindowHandle> for crate::NonNullHWND { fn from(win32: Win32WindowHandle) -> Self { Self::from_nz_isize(win32.hwnd) } }

impl From<crate::NonNullHWND> for Win32WindowHandle { fn from(hwnd: crate::NonNullHWND) -> Self { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize()); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); h        } }
impl From<crate::NonNullHWND> for RawWindowHandle   { fn from(hwnd: crate::NonNullHWND) -> Self { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize()); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); h.into() } }
impl TryFrom<crate::HWND>     for Win32WindowHandle { fn try_from(hwnd: crate::HWND) -> Result<Self, ()> { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); Ok(h       ) } type Error = (); }
impl TryFrom<crate::HWND>     for RawWindowHandle   { fn try_from(hwnd: crate::HWND) -> Result<Self, ()> { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); Ok(h.into()) } type Error = (); }
