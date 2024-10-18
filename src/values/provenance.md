### Pointer Provenance

`HWND`s aren't real pointers ever since Windows 95 / 32-bit Windows, and effectively have no pointer provenance to worry about.
Perhaps someday [`core::ptr::without_provenance_mut`] will be stabilized, be made `const`-friendly, and used... until then, this crate theoretically allows `HWND`s to gain overly permissive provenances.
This should cause no additional undefined behavior, but might make sanitizer diagnostics less clear if you do something incredibly silly like try to dereference an `HWND` (which is always undefined behavior.)

| OS                | Implementation    | Notes |
| ------------------| ------------------| ------|
| Windows&nbsp;3.1  | Near pointer into the window manager's data segment <sup>\[[tont](https://devblogs.microsoft.com/oldnewthing/20070716-00/?p=26003)\]</sup>                            | Technically would have provenance, but 16-bit Windows isn't supported by `rustc`, mooting the issue.  Even [Dennis Duda](https://seri.tools/blog/compiling-rust-for-legacy-windows/) hasn't tried backporting Rust binaries to Windows 3.1. |
| Windows&nbsp;95+  | ≈ Byte offset into 64 KiB handle table <sup>\[[tont](https://devblogs.microsoft.com/oldnewthing/20070716-00/?p=26003)\]</sup>                         | Effectively a [`u16`] index/offset, not a pointer. |
| Windows&nbsp;NT+  | ≈ <code>([u16], [u16])</code> index/offset and "uniquifier" <sup>\[[tont](https://devblogs.microsoft.com/oldnewthing/20070717-00/?p=25983)\]</sup>    | Fixes most handle reuse bugs in practice. |
