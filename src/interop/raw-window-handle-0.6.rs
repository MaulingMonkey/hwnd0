use raw_window_handle::{RawWindowHandle, Win32WindowHandle};

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

#[cfg(xxx)] mod xxx { // do I want to populate `h.hinstance`? in 0.6, `Win32WindowHandle::new` makes it pretty clear that `hinstance = None` is OK, but...
    impl From<crate::NonNullHWND> for Win32WindowHandle { fn from(hwnd: crate::NonNullHWND) -> Self { Win32WindowHandle::new(hwnd.to_nz_isize())        } }
    impl From<crate::NonNullHWND> for RawWindowHandle   { fn from(hwnd: crate::NonNullHWND) -> Self { Win32WindowHandle::new(hwnd.to_nz_isize()).into() } }
    impl TryFrom<crate::HWND>     for Win32WindowHandle { fn try_from(hwnd: crate::HWND) -> Result<Self, ()> { Ok(Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?)       ) } type Error = (); }
    impl TryFrom<crate::HWND>     for RawWindowHandle   { fn try_from(hwnd: crate::HWND) -> Result<Self, ()> { Ok(Win32WindowHandle::new(hwnd.to_nz_isize().ok_or(())?).into()) } type Error = (); }
}
