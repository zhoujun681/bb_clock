extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./ico/clock.ico");
    res.compile().unwrap();
    slint_build::compile("ui/appwindow.slint").unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
}
