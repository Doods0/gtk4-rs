// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ListItemFactory};
#[cfg(feature = "v4_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
use glib::{
    object::ObjectType as _,
    signal::{connect_raw, SignalHandlerId},
};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v4_8")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSignalListItemFactory")]
    pub struct SignalListItemFactory(Object<ffi::GtkSignalListItemFactory, ffi::GtkSignalListItemFactoryClass>) @extends ListItemFactory;

    match fn {
        type_ => || ffi::gtk_signal_list_item_factory_get_type(),
    }
}

impl SignalListItemFactory {
    #[doc(alias = "gtk_signal_list_item_factory_new")]
    pub fn new() -> SignalListItemFactory {
        assert_initialized_main_thread!();
        unsafe {
            ListItemFactory::from_glib_full(ffi::gtk_signal_list_item_factory_new()).unsafe_cast()
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "bind")]
    pub fn connect_bind<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn bind_trampoline<
            F: Fn(&SignalListItemFactory, &glib::Object) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bind\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    bind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "setup")]
    pub fn connect_setup<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn setup_trampoline<
            F: Fn(&SignalListItemFactory, &glib::Object) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "teardown")]
    pub fn connect_teardown<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn teardown_trampoline<
            F: Fn(&SignalListItemFactory, &glib::Object) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"teardown\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    teardown_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "unbind")]
    pub fn connect_unbind<F: Fn(&Self, &glib::Object) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unbind_trampoline<
            F: Fn(&SignalListItemFactory, &glib::Object) + 'static,
        >(
            this: *mut ffi::GtkSignalListItemFactory,
            object: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unbind\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    unbind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SignalListItemFactory {
    fn default() -> Self {
        Self::new()
    }
}
