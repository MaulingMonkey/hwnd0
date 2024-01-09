use crate::*;
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use core::num::NonZeroIsize;
fn to_win32(rwh: RawWindowHandle) -> Option<Win32WindowHandle> { match rwh { RawWindowHandle::Win32(win32) => Some(win32), _ => None } }

impl TryFrom<RawWindowHandle>   for HWND        { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).map(|win32| Self::from_nz_isize(win32.hwnd)).ok_or(()) } type Error = (); }
impl TryFrom<RawWindowHandle>   for NonNullHWND { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).map(|win32| Self::from_nz_isize(win32.hwnd)).ok_or(()) } type Error = (); }
impl    From<Win32WindowHandle> for HWND        { fn     from(win32: Win32WindowHandle) -> Self { Self::from_nz_isize(win32.hwnd) } }
impl    From<Win32WindowHandle> for NonNullHWND { fn     from(win32: Win32WindowHandle) -> Self { Self::from_nz_isize(win32.hwnd) } }

impl    From<NonNullHWND>   for Win32WindowHandle   { fn     from(hwnd: NonNullHWND) -> Self      { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize());            h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); h        } }
impl    From<NonNullHWND>   for RawWindowHandle     { fn     from(hwnd: NonNullHWND) -> Self      { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize());            h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); h.into() } }
impl TryFrom<HWND>          for Win32WindowHandle   { fn try_from(hwnd: HWND) -> Result<Self, ()> { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); Ok(h       ) } type Error = (); }
impl TryFrom<HWND>          for RawWindowHandle     { fn try_from(hwnd: HWND) -> Result<Self, ()> { let mut h = Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?); h.hinstance = NonZeroIsize::new(get_window_long_ptr_w(hwnd, GWLP_HINSTANCE)); Ok(h.into()) } type Error = (); }
