use hwnd0::*;

fn main() {
    #[link(name = "user32")] extern "C" { fn GetDesktopWindow() -> Option<NonNullHWND>; }
    let desktop = unsafe { GetDesktopWindow() }.unwrap();

    let a = [
        HWND::MESSAGE,
        HWND::NOTOPMOST,
        HWND::TOPMOST,
        HWND::TOP,
        HWND::DESKTOP,
        HWND::NULL,
        HWND::BOTTOM,
        HWND::BROADCAST,
        desktop.into(),
    ];

    let b = [
        NonNullHWND::MESSAGE,
        NonNullHWND::BROADCAST,
        desktop,
    ];

    dbg!((a, b));
}
