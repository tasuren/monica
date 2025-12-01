pub trait WindowExt {
    fn set_most_top(&self) {}

    fn set_hidden(&self, hidden: bool);

    fn set_ignore_cursor_events(&self, ignore: bool);
}

#[cfg(target_os = "macos")]
mod macos {
    use objc2::rc::Retained;
    use objc2_app_kit::{NSView, NSWindow};
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

    impl super::WindowExt for gpui::Window {
        fn set_most_top(&self) {
            // Move front over canvas window level
            get_ns_window(self).setLevel(objc2_app_kit::NSFloatingWindowLevel + 1);
        }

        fn set_hidden(&self, hidden: bool) {
            get_ns_window(self).setIsVisible(hidden);
        }

        fn set_ignore_cursor_events(&self, ignore: bool) {
            get_ns_window(self).setIgnoresMouseEvents(ignore);
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    impl super::WindowExt for gpui::Window {
        fn set_ignore_cursor_events(&self, ignore: bool) {
            unimplemented!()
        }
    }
}
