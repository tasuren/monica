#[cfg(windows)]
fn main() {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("icons/Monica.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
