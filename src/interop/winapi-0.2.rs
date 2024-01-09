use winapi::windef::{HWND, HWND__};
use core::ptr::NonNull;

impl From<HWND              > for crate::HWND           { fn from(hwnd: HWND            ) -> Self { Self::from_ptr(hwnd) } }
impl From<NonNull<HWND__>   > for crate::HWND           { fn from(hwnd: NonNull<HWND__> ) -> Self { Self::from_non_null(hwnd) } }
impl From<NonNull<HWND__>   > for crate::NonNullHWND    { fn from(hwnd: NonNull<HWND__> ) -> Self { Self::from_non_null(hwnd) } }

impl From<crate::HWND       > for HWND              { fn from(hwnd: crate::HWND         ) -> Self { hwnd.to_ptr() } }
impl From<crate::NonNullHWND> for HWND              { fn from(hwnd: crate::NonNullHWND  ) -> Self { hwnd.to_ptr() } }
impl From<crate::NonNullHWND> for NonNull<HWND__>   { fn from(hwnd: crate::NonNullHWND  ) -> Self { hwnd.to_non_null() } }
