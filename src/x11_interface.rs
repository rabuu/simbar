use std::ffi::{CStr, CString};
use std::os::raw::c_ulong;
use std::ptr;
use x11::xlib;

/// Interface to communicate with X11 Server
pub struct X11Interface {
    display: *mut xlib::Display,
    window: c_ulong,
}

impl X11Interface {
    /// Constructor for `X11Interface`
    pub fn new() -> Self {
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        let window = unsafe { xlib::XDefaultRootWindow(display) };

        X11Interface { display, window }
    }

    /// Set the X11 status
    pub fn set_status(&self, data: &str) {
        let cstr = CString::new(data).unwrap();
        let ptr = cstr.as_ptr() as *const i8;

        unsafe {
            xlib::XStoreName(self.display, self.window, ptr);
            xlib::XSync(self.display, 0);
        }
    }

    /// Get a Xresources value
    pub fn get_xresource(&self, name: impl AsRef<str>) -> Option<String> {
        let xrm = self.get_xresource_manager_string()?;
        for resource in xrm.lines() {
            if resource.starts_with(&format!("simbar.{}:", name.as_ref())) {
                let value = resource.splitn(2, ':').last()?.trim().to_string();
                return Some(value);
            }
        }
        None
    }

    fn get_xresource_manager_string(&self) -> Option<String> {
        let xresmanstr = unsafe {
            let ptr = xlib::XResourceManagerString(self.display);
            CStr::from_ptr(ptr)
        };

        Some(xresmanstr.to_str().ok()?.to_string())
    }
}

impl Drop for X11Interface {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
        drop(self.display);
        drop(self.window);
    }
}
