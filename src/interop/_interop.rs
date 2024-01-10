mod core;

#[cfg(feature = "bytemuck-1")] mod bytemuck_1 { use bytemuck_1 as bytemuck; include!("bytemuck-1.rs"); }



#[cfg(feature = "raw-window-handle-0-1")] mod raw_window_handle_0_1 { use raw_window_handle_0_1 as raw_window_handle; include!("raw-window-handle-0.1.rs"); }
#[cfg(feature = "raw-window-handle-0-2")] mod raw_window_handle_0_2 { use raw_window_handle_0_2 as raw_window_handle; include!("raw-window-handle-0.1.rs"); }
// windows::WindowHandle gained an hinstance field
#[cfg(feature = "raw-window-handle-0-3")] mod raw_window_handle_0_3 { use raw_window_handle_0_3 as raw_window_handle; include!("raw-window-handle-0.3.rs"); }
// windows::WindowsHandle → Win32Handle
#[cfg(feature = "raw-window-handle-0-4")] mod raw_window_handle_0_4 { use raw_window_handle_0_4 as raw_window_handle; include!("raw-window-handle-0.4.rs"); }
// Win32Handle → Win32WindowHandle
#[cfg(feature = "raw-window-handle-0-5")] mod raw_window_handle_0_5 { use raw_window_handle_0_5 as raw_window_handle; include!("raw-window-handle-0.5.rs"); }
// nonzero interior
#[cfg(feature = "raw-window-handle-0-6")] mod raw_window_handle_0_6 { use raw_window_handle_0_6 as raw_window_handle; include!("raw-window-handle-0.6.rs"); }



#[cfg(feature = "winapi-0-3")] mod winapi_0_3 { use winapi_0_3 as winapi; include!("winapi-0.3.rs"); }
#[cfg(feature = "winapi-0-2")] mod winapi_0_2 { use winapi_0_2 as winapi; include!("winapi-0.2.rs"); }
#[cfg(feature = "winapi-0-1")] mod winapi_0_2 { use winapi_0_1 as winapi; include!("winapi-0.1.rs"); }



#[cfg(feature = "windows-0-52")] mod windows_0_52 { use windows_0_52 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-51")] mod windows_0_51 { use windows_0_51 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-50")] mod windows_0_50 { use windows_0_50 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-49")] mod windows_0_49 { use windows_0_49 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-48")] mod windows_0_48 { use windows_0_48 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-47")] mod windows_0_47 { use windows_0_47 as windows; include!("windows-0.46.rs"); }
#[cfg(feature = "windows-0-46")] mod windows_0_46 { use windows_0_46 as windows; include!("windows-0.46.rs"); }
// windows = "0.46": windows::core::Interface renamed to windows::core::ComInterface, Into<InParam<T>> replaced with IntoParam<T>
#[cfg(feature = "windows-0-45")] mod windows_0_45 { use windows_0_45 as windows; include!("windows-0.44.rs"); }
#[cfg(feature = "windows-0-44")] mod windows_0_44 { use windows_0_44 as windows; include!("windows-0.44.rs"); }
// windows = "0.44": lifetime removed from Into<InParam<'_, T>>
#[cfg(feature = "windows-0-43")] mod windows_0_43 { use windows_0_43 as windows; include!("windows-0.39.rs"); }
#[cfg(feature = "windows-0-42")] mod windows_0_42 { use windows_0_42 as windows; include!("windows-0.39.rs"); }
#[cfg(feature = "windows-0-41")] mod windows_0_41 { use windows_0_41 as windows; include!("windows-0.39.rs"); }
#[cfg(feature = "windows-0-40")] mod windows_0_40 { use windows_0_40 as windows; include!("windows-0.39.rs"); }
#[cfg(feature = "windows-0-39")] mod windows_0_39 { use windows_0_39 as windows; include!("windows-0.39.rs"); }
// windows = "0.39": IntoParam<T> replaced with Into<InParam<'_, T>>
#[cfg(feature = "windows-0-38")] mod windows_0_38 { use windows_0_38 as windows; include!("windows-0.35.rs"); }
#[cfg(feature = "windows-0-37")] mod windows_0_37 { use windows_0_37 as windows; include!("windows-0.35.rs"); }
#[cfg(feature = "windows-0-36")] mod windows_0_36 { use windows_0_36 as windows; include!("windows-0.35.rs"); }
#[cfg(feature = "windows-0-35")] mod windows_0_35 { use windows_0_35 as windows; include!("windows-0.35.rs"); }
// windows = "0.35": windows::Win32::System::WinRT::ICoreWindowInterop introduced
#[cfg(feature = "windows-0-34")] mod windows_0_34 { use windows_0_34 as windows; include!("windows-0.30.rs"); }
#[cfg(feature = "windows-0-33")] mod windows_0_33 { use windows_0_33 as windows; include!("windows-0.30.rs"); }
#[cfg(feature = "windows-0-32")] mod windows_0_32 { use windows_0_32 as windows; include!("windows-0.30.rs"); }
#[cfg(feature = "windows-0-31")] mod windows_0_31 { use windows_0_31 as windows; include!("windows-0.30.rs"); }
#[cfg(feature = "windows-0-30")] mod windows_0_30 { use windows_0_30 as windows; include!("windows-0.30.rs"); }
// windows = "0.30": windows::Win32::Foundation::HWND made non-primitive
