pub trait WindowExt {
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

    impl super::WindowExt for gpui::Window {
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

    pub trait MacOSWindowExt {
        fn setup_canvas_window(&self);

        fn setup_main_window(&self);
    }

    const CANVAS_WINDOW_LEVEL: NSWindowLevel = objc2_app_kit::NSPopUpMenuWindowLevel + 1;

    impl MacOSWindowExt for gpui::Window {
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

        fn setup_main_window(&self) {
            get_ns_window(self).setLevel(CANVAS_WINDOW_LEVEL + 1);
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
