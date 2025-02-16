// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Accessible, AccessibleRole};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkATContext")]
    pub struct ATContext(Object<ffi::GtkATContext, ffi::GtkATContextClass>);

    match fn {
        type_ => || ffi::gtk_at_context_get_type(),
    }
}

impl ATContext {
    #[doc(alias = "gtk_at_context_create")]
    pub fn create(
        accessible_role: AccessibleRole,
        accessible: &impl IsA<Accessible>,
        display: &impl IsA<gdk::Display>,
    ) -> Option<ATContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_at_context_create(
                accessible_role.into_glib(),
                accessible.as_ref().to_glib_none().0,
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_at_context_get_accessible")]
    #[doc(alias = "get_accessible")]
    pub fn accessible(&self) -> Accessible {
        unsafe { from_glib_none(ffi::gtk_at_context_get_accessible(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_at_context_get_accessible_role")]
    #[doc(alias = "get_accessible_role")]
    #[doc(alias = "accessible-role")]
    pub fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            from_glib(ffi::gtk_at_context_get_accessible_role(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "accessible-role")]
    pub fn set_accessible_role(&self, accessible_role: AccessibleRole) {
        ObjectExt::set_property(self, "accessible-role", accessible_role)
    }

    pub fn display(&self) -> Option<gdk::Display> {
        ObjectExt::property(self, "display")
    }

    pub fn set_display<P: IsA<gdk::Display>>(&self, display: Option<&P>) {
        ObjectExt::set_property(self, "display", display)
    }

    #[doc(alias = "state-change")]
    pub fn connect_state_change<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_change_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-change\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    state_change_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accessible-role")]
    pub fn connect_accessible_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "display")]
    pub fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
