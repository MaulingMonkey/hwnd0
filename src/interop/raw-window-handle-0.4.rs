use crate::*;
use raw_window_handle::{RawWindowHandle, Win32Handle};

impl TryFrom<RawWindowHandle> for crate::HWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Win32(win32) = rwh  { Ok(Self::from_ptr(win32.hwnd)) }
        else                                        { Err(()) }
    }
}

impl TryFrom<RawWindowHandle> for crate::NonNullHWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Win32(win32) = rwh  { Self::from_ptr(win32.hwnd).ok_or(()) }
        else                                        { Err(()) }
    }
}

impl From<Win32Handle>      for crate::HWND         { fn from(win32: Win32Handle) -> Self { Self::from_ptr(win32.hwnd) } }
impl TryFrom<Win32Handle>   for crate::NonNullHWND  { fn try_from(win32: Win32Handle) -> Result<Self, ()> { Self::from_ptr(win32.hwnd).ok_or(()) } type Error = (); }

impl From<crate::NonNullHWND> for Win32Handle       { fn from(hwnd: crate::NonNullHWND  ) -> Self { let mut h = Win32Handle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<crate::HWND>        for Win32Handle       { fn from(hwnd: crate::HWND         ) -> Self { let mut h = Win32Handle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<crate::NonNullHWND> for RawWindowHandle   { fn from(hwnd: crate::NonNullHWND  ) -> Self { let mut h = Win32Handle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Win32(h) } }
impl From<crate::HWND>        for RawWindowHandle   { fn from(hwnd: crate::HWND         ) -> Self { let mut h = Win32Handle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Win32(h) } }
