use crate::*;
use raw_window_handle::{RawWindowHandle, windows};

impl TryFrom<RawWindowHandle> for crate::HWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Windows(win32) = rwh    { Ok(Self::from_ptr(win32.hwnd)) }
        else                                            { Err(()) }
    }
}

impl TryFrom<RawWindowHandle> for crate::NonNullHWND {
    type Error = ();
    fn try_from(rwh: RawWindowHandle) -> Result<Self, Self::Error> {
        if let RawWindowHandle::Windows(win32) = rwh    { Self::from_ptr(win32.hwnd).ok_or(()) }
        else                                            { Err(()) }
    }
}

impl From<windows::WindowsHandle>       for crate::HWND         { fn from(win32: windows::WindowsHandle) -> Self { Self::from_ptr(win32.hwnd) } }
impl TryFrom<windows::WindowsHandle>    for crate::NonNullHWND  { fn try_from(win32: windows::WindowsHandle) -> Result<Self, ()> { Self::from_ptr(win32.hwnd).ok_or(()) } type Error = (); }

impl From<crate::NonNullHWND> for windows::WindowsHandle    { fn from(hwnd: crate::NonNullHWND  ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<crate::HWND>        for windows::WindowsHandle    { fn from(hwnd: crate::HWND         ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<crate::NonNullHWND> for RawWindowHandle           { fn from(hwnd: crate::NonNullHWND  ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Windows(h) } }
impl From<crate::HWND>        for RawWindowHandle           { fn from(hwnd: crate::HWND         ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Windows(h) } }
