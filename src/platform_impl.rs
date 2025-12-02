pub trait WindowExt {
    fn setup_main_window(&self) {}

    fn setup_canvas_window(&self) {}

    fn set_hidden(&self, hidden: bool);

    fn set_ignore_cursor_events(&self, ignore: bool);
}

#[cfg(target_os = "macos")]
pub mod macos {
    use objc2::rc::Retained;
    use objc2_app_kit::{NSColor, NSView, NSWindow, NSWindowCollectionBehavior, NSWindowLevel};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    fn get_ns_window(window: &gpui::Window) -> Retained<NSWindow> {
        let handle = HasWindowHandle::window_handle(window).unwrap().as_raw();
        let ns_view: Retained<NSView> = match handle {
            RawWindowHandle::AppKit(handle) => unsafe {
                Retained::retain(handle.ns_view.as_ptr().cast()).expect("Failed to get `NSView`")
            },
            _ => unreachable!(),
        };

        ns_view
            .window()
            .expect("There no `NSWindow` with `NSView`.")
    }

    const CANVAS_WINDOW_LEVEL: NSWindowLevel = objc2_app_kit::NSPopUpMenuWindowLevel + 1;

    impl super::WindowExt for gpui::Window {
        fn setup_main_window(&self) {
            get_ns_window(self).setLevel(CANVAS_WINDOW_LEVEL + 1);
        }

        fn setup_canvas_window(&self) {
            let ns_window = get_ns_window(self);

            ns_window.setIgnoresMouseEvents(true);

            ns_window.setLevel(CANVAS_WINDOW_LEVEL);
            ns_window.setCollectionBehavior(
                NSWindowCollectionBehavior::CanJoinAllApplications
                    | NSWindowCollectionBehavior::CanJoinAllSpaces
                    | NSWindowCollectionBehavior::FullScreenAuxiliary,
            );
            ns_window.setMovable(false);

            // Remove border and shadow.
            ns_window.setOpaque(false);
            ns_window.setBackgroundColor(Some(&NSColor::clearColor()));
        }

        fn set_hidden(&self, hidden: bool) {
            let ns_window = get_ns_window(self);

            if hidden {
                ns_window.orderOut(Some(&ns_window));
            } else {
                ns_window.makeKeyAndOrderFront(Some(&ns_window));
            }
        }

        fn set_ignore_cursor_events(&self, ignore: bool) {
            get_ns_window(self).setIgnoresMouseEvents(ignore);
        }
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows::Win32::{
        Foundation::{HWND, RECT},
        Graphics::Dwm::{DWMWA_EXTENDED_FRAME_BOUNDS, DwmGetWindowAttribute},
        UI::WindowsAndMessaging::*,
    };

    fn get_hwnd(window: &gpui::Window) -> HWND {
        let handle = HasWindowHandle::window_handle(window).unwrap().as_raw();

        match handle {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as _),
            _ => unreachable!(),
        }
    }

    fn manage_style(hwnd: HWND, add_mode: bool, target_style: WINDOW_STYLE) {
        let raw_style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
        let mut style = WINDOW_STYLE(raw_style as _);

        if add_mode {
            style |= target_style;
        } else {
            style &= !target_style;
        }

        unsafe {
            _ = SetWindowLongPtrW(hwnd, GWL_STYLE, style.0 as _);
        };
    }

    fn manage_style_ex(hwnd: HWND, add_mode: bool, target_style_ex: WINDOW_EX_STYLE) {
        let raw_style_ex = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
        let mut style_ex = WINDOW_EX_STYLE(raw_style_ex as _);

        if add_mode {
            style_ex |= target_style_ex;
        } else {
            style_ex &= !target_style_ex;
        }

        unsafe {
            _ = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style_ex.0 as _);
        };
    }

    fn set_always_on_top(hwnd: HWND) {
        unsafe {
            _ = SetWindowPos(
                hwnd,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        }
    }

    impl super::WindowExt for gpui::Window {
        fn setup_main_window(&self) {
            // I don't know why, but gpui floating and popup window
            // doesn't make itself topmost. Then we do this ourselves.
            set_always_on_top(get_hwnd(self));
        }

        fn setup_canvas_window(&self) {
            let hwnd = get_hwnd(self);

            // Remove border and shadow
            manage_style(hwnd, false, WS_BORDER);

            manage_style(hwnd, true, WS_POPUP);
            manage_style_ex(
                hwnd,
                true,
                WS_EX_NOACTIVATE | WS_EX_TRANSPARENT | WS_EX_LAYERED,
            );

            set_always_on_top(hwnd);
        }

        fn set_hidden(&self, hidden: bool) {
            let hwnd = get_hwnd(self);

            unsafe {
                _ = ShowWindow(hwnd, if hidden { SW_HIDE } else { SW_SHOWNOACTIVATE });
            };
        }

        fn set_ignore_cursor_events(&self, ignore: bool) {
            let hwnd = get_hwnd(self);
            manage_style_ex(hwnd, ignore, WS_EX_TRANSPARENT | WS_EX_LAYERED);
        }
    }

    pub trait WindowsWindowExt {
        fn set_window_rect(&self, x: i32, y: i32, width: i32, height: i32);

        fn set_window_pos_top(&self);
    }

    impl WindowsWindowExt for gpui::Window {
        fn set_window_rect(&self, x: i32, y: i32, width: i32, height: i32) {
            let hwnd = get_hwnd(self);

            unsafe {
                // Calculate the invisible resize border.
                let mut window_rect = RECT::default();
                let mut client_rect = RECT::default();

                _ = DwmGetWindowAttribute(
                    hwnd,
                    DWMWA_EXTENDED_FRAME_BOUNDS,
                    &raw mut window_rect as _,
                    std::mem::size_of::<RECT>() as _,
                );
                _ = GetClientRect(hwnd, &raw mut client_rect);

                let diff_x = (window_rect.right - window_rect.left) - client_rect.right;
                let diff_y = (window_rect.bottom - window_rect.top) - client_rect.bottom;

                // Set the window position and size without the invisible resize border.
                _ = SetWindowPos(
                    hwnd,
                    None,
                    x - diff_x,
                    y - diff_y,
                    width + diff_x,
                    height + diff_y,
                    SWP_NOACTIVATE,
                );
            }
        }

        fn set_window_pos_top(&self) {
            unsafe {
                _ = SetWindowPos(
                    get_hwnd(self),
                    Some(HWND_TOP),
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
                );
            }
        }
    }
}
