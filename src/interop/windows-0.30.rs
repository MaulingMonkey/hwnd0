use windows::{
    core::{IntoParam, Param},
    Win32::Foundation::HWND,
};

impl From<HWND              > for crate::HWND   { fn from(hwnd: HWND            ) -> Self { Self::from_isize(hwnd.0) } }
impl From<crate::HWND       > for HWND      { fn from(hwnd: crate::HWND         ) -> Self { Self(hwnd.to_isize()) } }
impl From<crate::NonNullHWND> for HWND      { fn from(hwnd: crate::NonNullHWND  ) -> Self { Self(hwnd.to_isize()) } }

impl<'t> IntoParam<'t, HWND> for crate::HWND        { fn into_param(self) -> Param<'t, HWND> { Param::Owned(self.into()) } }
impl<'t> IntoParam<'t, HWND> for crate::NonNullHWND { fn into_param(self) -> Param<'t, HWND> { Param::Owned(self.into()) } }
