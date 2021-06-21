// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ContentFormats;
use crate::ContentProvider;
use crate::Device;
use crate::Display;
use crate::DragAction;
use crate::DragCancelReason;
use crate::Surface;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkDrag")]
    pub struct Drag(Object<ffi::GdkDrag>);

    match fn {
        type_ => || ffi::gdk_drag_get_type(),
    }
}

impl Drag {
    #[doc(alias = "gdk_drag_begin")]
    pub fn begin<P: IsA<Surface>, Q: IsA<Device>, R: IsA<ContentProvider>>(
        surface: &P,
        device: &Q,
        content: &R,
        actions: DragAction,
        dx: f64,
        dy: f64,
    ) -> Option<Drag> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin(
                surface.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                content.as_ref().to_glib_none().0,
                actions.into_glib(),
                dx,
                dy,
            ))
        }
    }
}

pub const NONE_DRAG: Option<&Drag> = None;

pub trait DragExt: 'static {
    #[doc(alias = "gdk_drag_drop_done")]
    fn drop_done(&self, success: bool);

    #[doc(alias = "gdk_drag_get_actions")]
    #[doc(alias = "get_actions")]
    fn actions(&self) -> DragAction;

    #[doc(alias = "gdk_drag_get_content")]
    #[doc(alias = "get_content")]
    fn content(&self) -> Option<ContentProvider>;

    #[doc(alias = "gdk_drag_get_device")]
    #[doc(alias = "get_device")]
    fn device(&self) -> Option<Device>;

    #[doc(alias = "gdk_drag_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_drag_get_drag_surface")]
    #[doc(alias = "get_drag_surface")]
    fn drag_surface(&self) -> Option<Surface>;

    #[doc(alias = "gdk_drag_get_formats")]
    #[doc(alias = "get_formats")]
    fn formats(&self) -> Option<ContentFormats>;

    #[doc(alias = "gdk_drag_get_selected_action")]
    #[doc(alias = "get_selected_action")]
    fn selected_action(&self) -> DragAction;

    #[doc(alias = "gdk_drag_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<Surface>;

    #[doc(alias = "gdk_drag_set_hotspot")]
    fn set_hotspot(&self, hot_x: i32, hot_y: i32);

    fn set_actions(&self, actions: DragAction);

    #[doc(alias = "selected-action")]
    fn set_selected_action(&self, selected_action: DragAction);

    #[doc(alias = "cancel")]
    fn connect_cancel<F: Fn(&Self, DragCancelReason) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "dnd-finished")]
    fn connect_dnd_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "drop-performed")]
    fn connect_drop_performed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "actions")]
    fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "display")]
    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selected-action")]
    fn connect_selected_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Drag>> DragExt for O {
    fn drop_done(&self, success: bool) {
        unsafe {
            ffi::gdk_drag_drop_done(self.as_ref().to_glib_none().0, success.into_glib());
        }
    }

    fn actions(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drag_get_actions(self.as_ref().to_glib_none().0)) }
    }

    fn content(&self) -> Option<ContentProvider> {
        unsafe { from_glib_none(ffi::gdk_drag_get_content(self.as_ref().to_glib_none().0)) }
    }

    fn device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_drag_get_device(self.as_ref().to_glib_none().0)) }
    }

    fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_drag_get_display(self.as_ref().to_glib_none().0)) }
    }

    fn drag_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_drag_get_drag_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn formats(&self) -> Option<ContentFormats> {
        unsafe { from_glib_none(ffi::gdk_drag_get_formats(self.as_ref().to_glib_none().0)) }
    }

    fn selected_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_get_selected_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_drag_get_surface(self.as_ref().to_glib_none().0)) }
    }

    fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_set_hotspot(self.as_ref().to_glib_none().0, hot_x, hot_y);
        }
    }

    fn set_actions(&self, actions: DragAction) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"actions\0".as_ptr() as *const _,
                actions.to_value().to_glib_none().0,
            );
        }
    }

    fn set_selected_action(&self, selected_action: DragAction) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"selected-action\0".as_ptr() as *const _,
                selected_action.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_cancel<F: Fn(&Self, DragCancelReason) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<
            P: IsA<Drag>,
            F: Fn(&P, DragCancelReason) + 'static,
        >(
            this: *mut ffi::GdkDrag,
            reason: ffi::GdkDragCancelReason,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Drag::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(reason),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_dnd_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dnd_finished_trampoline<P: IsA<Drag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDrag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"dnd-finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    dnd_finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drop_performed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drop_performed_trampoline<P: IsA<Drag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDrag,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop-performed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_performed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<P: IsA<Drag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDrag,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actions_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<P: IsA<Drag>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkDrag,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selected_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_action_trampoline<
            P: IsA<Drag>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkDrag,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Drag::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Drag")
    }
}
