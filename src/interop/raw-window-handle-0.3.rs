use crate::*;
use raw_window_handle::{RawWindowHandle, windows};

fn to_win32(rwh: RawWindowHandle) -> Option<windows::WindowsHandle> { match rwh { RawWindowHandle::Windows(win32) => Some(win32), _ => None } }

impl TryFrom<RawWindowHandle>           for HWND        { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).map     (|win32| Self::from_ptr(win32.hwnd)).ok_or(()) } type Error = (); }
impl TryFrom<RawWindowHandle>           for NonNullHWND { fn try_from(rwh: RawWindowHandle) -> Result<Self, ()> { to_win32(rwh).and_then(|win32| Self::from_ptr(win32.hwnd)).ok_or(()) } type Error = (); }
impl    From<windows::WindowsHandle>    for HWND        { fn     from(win32: windows::WindowsHandle) ->        Self      { Self::from_ptr(win32.hwnd)           } }
impl TryFrom<windows::WindowsHandle>    for NonNullHWND { fn try_from(win32: windows::WindowsHandle) -> Result<Self, ()> { Self::from_ptr(win32.hwnd).ok_or(()) } type Error = (); }

impl From<NonNullHWND>  for windows::WindowsHandle  { fn from(hwnd: NonNullHWND ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<HWND>         for windows::WindowsHandle  { fn from(hwnd: HWND        ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; h } }
impl From<NonNullHWND>  for RawWindowHandle         { fn from(hwnd: NonNullHWND ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Windows(h) } }
impl From<HWND>         for RawWindowHandle         { fn from(hwnd: HWND        ) -> Self { let mut h = windows::WindowsHandle::empty(); h.hwnd = hwnd.to_ptr(); h.hinstance = get_window_long_ptr_w(hwnd, GWLP_HINSTANCE) as _; RawWindowHandle::Windows(h) } }
