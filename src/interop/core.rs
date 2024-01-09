#[cfg(feature = "hwnd-from-isize")] impl From<isize> for crate::HWND { fn from(hwnd: isize) -> Self { Self::from_isize(hwnd) } }
#[cfg(feature = "hwnd-from-usize")] impl From<usize> for crate::HWND { fn from(hwnd: usize) -> Self { Self::from_usize(hwnd) } }

#[cfg(feature = "isize-from-hwnd")] impl From<crate::HWND> for isize { fn from(hwnd: crate::HWND) -> Self { hwnd.to_isize() } }
#[cfg(feature = "usize-from-hwnd")] impl From<crate::HWND> for usize { fn from(hwnd: crate::HWND) -> Self { hwnd.to_usize() } }

#[cfg(feature = "hwnd-from-isize")] impl TryFrom<isize> for crate::NonNullHWND { fn try_from(hwnd: isize) -> Result<Self, ()> { Self::from_isize(hwnd).ok_or(()) } type Error = (); }
#[cfg(feature = "hwnd-from-usize")] impl TryFrom<usize> for crate::NonNullHWND { fn try_from(hwnd: usize) -> Result<Self, ()> { Self::from_usize(hwnd).ok_or(()) } type Error = (); }

#[cfg(feature = "isize-from-hwnd")] impl From<crate::NonNullHWND> for isize { fn from(hwnd: crate::NonNullHWND) -> Self { hwnd.to_isize() } }
#[cfg(feature = "usize-from-hwnd")] impl From<crate::NonNullHWND> for usize { fn from(hwnd: crate::NonNullHWND) -> Self { hwnd.to_usize() } }
