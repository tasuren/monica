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
        UI::WindowsAndMessaging::*,
    };

    fn get_hwnd(window: &gpui::Window) -> HWND {
        let handle = HasWindowHandle::window_handle(window).unwrap().as_raw();

        match handle {
            RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as _),
            _ => unreachable!(),
        }
    }

    fn manage_ex_style(hwnd: HWND, add_mode: bool, target_ex_style: WINDOW_EX_STYLE) {
        let raw_ex_style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
        let mut ex_style = WINDOW_EX_STYLE(raw_ex_style as _);

        if add_mode {
            ex_style |= target_ex_style;
        } else {
            ex_style &= !target_ex_style;
        }

        unsafe {
            _ = SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style.0 as _);
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
            let hwnd = get_hwnd(self);
            set_always_on_top(hwnd);

            // Set the window as no activate.
            manage_ex_style(hwnd, true, WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE);
        }

        fn setup_canvas_window(&self) {
            let hwnd = get_hwnd(self);

            // Remove border, shadow and other window features.
            // If you set `WS_BORDER` to the window, then the window show the border and shadow.
            unsafe {
                _ = SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE).0 as _);
            };

            manage_ex_style(hwnd, true, WS_EX_NOACTIVATE);
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
            manage_ex_style(hwnd, ignore, WS_EX_TRANSPARENT | WS_EX_LAYERED);
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

                _ = GetWindowRect(hwnd, &raw mut window_rect);
                _ = GetClientRect(hwnd, &raw mut client_rect);

                let diff_x =
                    (window_rect.right - window_rect.left) - (client_rect.right - client_rect.left);
                let diff_y =
                    (window_rect.bottom - window_rect.top) - (client_rect.bottom - client_rect.top);
                let offset_x = diff_x / 2;
                let offset_y = diff_y / 2;

                // Set the window position and size without the invisible resize border.
                _ = SetWindowPos(
                    hwnd,
                    None,
                    x - offset_x,
                    y - offset_y,
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
