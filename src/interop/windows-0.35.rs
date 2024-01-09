use windows::{
    core::{Interface, Error, IntoParam, Param},
    UI::Core::{CoreWindow, ICoreWindow},
    Win32::Foundation::HWND,
    Win32::System::WinRT::ICoreWindowInterop,
};

impl From<HWND              > for crate::HWND   { fn from(hwnd: HWND            ) -> Self { Self::from_isize(hwnd.0) } }
impl From<crate::HWND       > for HWND      { fn from(hwnd: crate::HWND         ) -> Self { Self(hwnd.to_isize()) } }
impl From<crate::NonNullHWND> for HWND      { fn from(hwnd: crate::NonNullHWND  ) -> Self { Self(hwnd.to_isize()) } }

impl TryFrom<&CoreWindow    > for crate::HWND { type Error = Error; fn try_from(core_window: &CoreWindow ) -> Result<crate::HWND, Error> { unsafe { Ok(core_window.cast::<ICoreWindowInterop>()?.WindowHandle()?.into()) } } }
impl TryFrom<&ICoreWindow   > for crate::HWND { type Error = Error; fn try_from(core_window: &ICoreWindow) -> Result<crate::HWND, Error> { unsafe { Ok(core_window.cast::<ICoreWindowInterop>()?.WindowHandle()?.into()) } } }

impl<'t> IntoParam<'t, HWND> for crate::HWND        { fn into_param(self) -> Param<'t, HWND> { Param::Owned(self.into()) } }
impl<'t> IntoParam<'t, HWND> for crate::NonNullHWND { fn into_param(self) -> Param<'t, HWND> { Param::Owned(self.into()) } }
