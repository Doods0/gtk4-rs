// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkMacosMonitor")]
    pub struct MacosMonitor(Object<ffi::GdkMacosMonitor, ffi::GdkMacosMonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_macos_monitor_get_type(),
    }
}

impl MacosMonitor {
    #[doc(alias = "gdk_macos_monitor_get_workarea")]
    #[doc(alias = "get_workarea")]
    pub fn workarea(monitor: &impl IsA<gdk::Monitor>) -> gdk::Rectangle {
        assert_initialized_main_thread!();
        unsafe {
            let mut geometry = gdk::Rectangle::uninitialized();
            ffi::gdk_macos_monitor_get_workarea(
                monitor.as_ref().to_glib_none().0,
                geometry.to_glib_none_mut().0,
            );
            geometry
        }
    }
}
