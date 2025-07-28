#![allow(unexpected_cfgs)]

#[cfg(target_os = "macos")]
pub fn customize_window(window: &winit::window::Window) {
    use cocoa::appkit::{NSWindowStyleMask, NSWindowTitleVisibility};
    use cocoa::base::id;
    use objc::{msg_send, *};
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

    unsafe {
        let ns_window: id = match window.raw_window_handle() {
            RawWindowHandle::AppKit(handle) => handle.ns_window as id,
            _ => return,
        };

        let () =
            msg_send![ns_window, setTitleVisibility: NSWindowTitleVisibility::NSWindowTitleHidden];

        let () = msg_send![ns_window, setTitlebarAppearsTransparent: true];

        let () = msg_send![ns_window, setStyleMask: NSWindowStyleMask::NSTitledWindowMask
                                              | NSWindowStyleMask::NSClosableWindowMask
                                              | NSWindowStyleMask::NSMiniaturizableWindowMask
                                              | NSWindowStyleMask::NSResizableWindowMask
                                              | NSWindowStyleMask::NSFullSizeContentViewWindowMask];
    }
}
