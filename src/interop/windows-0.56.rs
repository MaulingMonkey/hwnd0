use windows::{
    core::{Interface, Error, Param, ParamValue},
    UI::Core::{CoreWindow, ICoreWindow},
    Win32::Foundation::HWND,
    Win32::System::WinRT::ICoreWindowInterop,
};

impl From<HWND              > for crate::HWND   { fn from(hwnd: HWND            ) -> Self { Self::from_isize(hwnd.0 as _) } }
impl From<crate::HWND       > for HWND      { fn from(hwnd: crate::HWND         ) -> Self { Self(hwnd.to_isize() as _) } }
impl From<crate::NonNullHWND> for HWND      { fn from(hwnd: crate::NonNullHWND  ) -> Self { Self(hwnd.to_isize() as _) } }

impl TryFrom<&CoreWindow    > for crate::HWND { type Error = Error; fn try_from(core_window: &CoreWindow ) -> Result<crate::HWND, Error> { unsafe { Ok(core_window.cast::<ICoreWindowInterop>()?.WindowHandle()?.into()) } } }
impl TryFrom<&ICoreWindow   > for crate::HWND { type Error = Error; fn try_from(core_window: &ICoreWindow) -> Result<crate::HWND, Error> { unsafe { Ok(core_window.cast::<ICoreWindowInterop>()?.WindowHandle()?.into()) } } }

impl Param<HWND> for crate::HWND        { unsafe fn param(self) -> ParamValue<HWND> { ParamValue::Owned(self.into()) } }
impl Param<HWND> for crate::NonNullHWND { unsafe fn param(self) -> ParamValue<HWND> { ParamValue::Owned(self.into()) } }
