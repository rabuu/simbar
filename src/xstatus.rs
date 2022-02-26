use std::ffi::CString;
use std::os::raw::c_ulong;
use std::ptr;
use x11::xlib;

pub struct XStatus {
    display: *mut xlib::Display,
    window: c_ulong,
}

impl XStatus {
    pub fn new() -> Self {
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        let window = unsafe { xlib::XDefaultRootWindow(display) };

        XStatus { display, window }
    }

    pub fn set_status(&self, data: &str) {
        let c_str = CString::new(data).unwrap();
        let str_ptr = c_str.as_ptr() as *const i8;

        unsafe {
            xlib::XStoreName(self.display, self.window, str_ptr);
            xlib::XSync(self.display, 0);
        }
    }
}
