use crate::*;
use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

fn to_win32(rwh: RawWindowHandle) -> Option<Win32WindowHandle> { match rwh { RawWindowHandle::Win32(win32) => Some(win32), _ => None } }

impl TryFrom<RawWindowHandle>   for HWND        { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).map     (|win32| Self::from_ptr(win32.hwnd)).ok_or(()) } type Error = (); }
impl TryFrom<RawWindowHandle>   for NonNullHWND { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).and_then(|win32| Self::from_ptr(win32.hwnd)).ok_or(()) } type Error = (); }
impl    From<Win32WindowHandle> for HWND        { fn     from(win32: Win32WindowHandle) ->        Self      { Self::from_ptr(win32.hwnd)           } }
impl TryFrom<Win32WindowHandle> for NonNullHWND { fn try_from(win32: Win32WindowHandle) -> Result<Self, ()> { Self::from_ptr(win32.hwnd).ok_or(()) } type Error = (); }

impl From<NonNullHWND>  for Win32WindowHandle   { fn from(hwnd: NonNullHWND ) -> Self { let mut h = Win32WindowHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<HWND>         for Win32WindowHandle   { fn from(hwnd: HWND        ) -> Self { let mut h = Win32WindowHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<NonNullHWND>  for RawWindowHandle     { fn from(hwnd: NonNullHWND ) -> Self { let mut h = Win32WindowHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h.into() } }
impl From<HWND>         for RawWindowHandle     { fn from(hwnd: HWND        ) -> Self { let mut h = Win32WindowHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h.into() } }
