### Thread Safety

The thread that creates a window will also be the thread that:
*   Executes that window's [`WNDPROC`]s (when the message loop is pumped.)
*   Eventually destroys said window (by e.g. calling [`DestroyWindow`] in response to [`WM_CLOSE`] via [`DefWindowProcW`]?)

This translates to Rust's thread safety traits nicely:

| Item              | [`Send`]  | [`Sync`]  |
| ------------------| ----------| ----------|
| `HWND`            | ✔️        | ✔️       |
| Underlying window | ❌        | ✔️       |

**Note well:** windows are a miserable pile of interior mutability:
*   Some "atomic" or mutexed (modifiable from multiple threads soundly, although the order of operations may be unreliable.)
*   Some forbidden by thread (e.g. only the window's thread can access/modify some state thanks to runtime checks.)
*   Some incredibly `unsafe` (e.g. only the window's thread **should** access/modify some state, caller beware.)<br>
    Safe wrappers can be made by checking the thread ID, which is uniquely throughout the system:<br>
    <code>assert_eq!\([GetWindowThreadProcessId]\(hwnd, nullptr\), [GetCurrentThreadId]\(\)\);</code>

Recommended reading: [Thread affinity of user interface objects, part 1: Window handles](https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843) (The Old New Thing.)



### Data Race Safety

Threads and processes are not the only source of data races.
[`WNDPROC`] are incredibly recursive, and worse, will execute any time you pump your message loop.
This includes when blocked on most modal dialog windows, even when you were already in the middle of executing another message loop &mdash; assertions, debug checks, file prompts, and possibly even heap corruption notices in the middle of allocating?

I recommend:
*   Being incredibly cautious about making unvalidated assumptions of window state.
*   Hold those assumptions for as short a duration as possible.
*   Assume any arbitrary callbacks, trait code, or third party crates will invalidate said assumptions.

Fortunately, most Win32 APIs appear sound to call with invalid handles (returning consistent dedicated error codes after failing to validate invalid handles.)
[Uniquifier](https://devblogs.microsoft.com/oldnewthing/20070717-00/?p=25983)s will also *typically* avoid use-after-free bugs in the short term.
For [`WNDPROC`]s you control, you could consider delaying window destruction until your main loop,
hiding windows instead of performing the default action in response to [`WM_CLOSE`].

If you *must* ensure a window remains valid for a block of code, and *cannot* modify the [`WNDPROC`] to extend the window's lifetime, consider (ab)using
<code>[SetWindowsHookExW]\(WH_CALLWNDPROC, ...\)</code>
to give yourself a chance to <code>[std::process::abort]\(\)</code> on [`WM_DESTROY`] instead of invoking undefined behavior.
This "should" only be necessary for some graphics APIs / non-Rust third party code.



<!-- references -->

[std::process::abort]:          https://doc.rust-lang.org/std/process/fn.abort.html
[`DestroyWindow`]:              https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow
[`DefWindowProcW`]:             https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw
[GetCurrentThreadId]:           https://learn.microsoft.com/en-gb/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthreadid
[GetWindowThreadProcessId]:     https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid
[SetWindowsHookExW]:            https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw
[`WM_CLOSE`]:                   https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-close
[`WM_DESTROY`]:                 https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-destroy
[`WNDPROC`]:                    https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc
