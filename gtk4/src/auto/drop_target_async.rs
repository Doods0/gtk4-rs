// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, PropagationLimit, PropagationPhase};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkDropTargetAsync")]
    pub struct DropTargetAsync(Object<ffi::GtkDropTargetAsync, ffi::GtkDropTargetAsyncClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_drop_target_async_get_type(),
    }
}

impl DropTargetAsync {
    #[doc(alias = "gtk_drop_target_async_new")]
    pub fn new(formats: Option<gdk::ContentFormats>, actions: gdk::DragAction) -> DropTargetAsync {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_drop_target_async_new(
                formats.into_glib_ptr(),
                actions.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DropTargetAsync`] objects.
    ///
    /// This method returns an instance of [`DropTargetAsyncBuilder`](crate::builders::DropTargetAsyncBuilder) which can be used to create [`DropTargetAsync`] objects.
    pub fn builder() -> DropTargetAsyncBuilder {
        DropTargetAsyncBuilder::new()
    }

    #[doc(alias = "gtk_drop_target_async_get_actions")]
    #[doc(alias = "get_actions")]
    pub fn actions(&self) -> gdk::DragAction {
        unsafe {
            from_glib(ffi::gtk_drop_target_async_get_actions(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drop_target_async_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> Option<gdk::ContentFormats> {
        unsafe {
            from_glib_full(ffi::gtk_drop_target_async_get_formats(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_drop_target_async_reject_drop")]
    pub fn reject_drop(&self, drop: &gdk::Drop) {
        unsafe {
            ffi::gtk_drop_target_async_reject_drop(self.to_glib_none().0, drop.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_drop_target_async_set_actions")]
    #[doc(alias = "actions")]
    pub fn set_actions(&self, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_drop_target_async_set_actions(self.to_glib_none().0, actions.into_glib());
        }
    }

    #[doc(alias = "gtk_drop_target_async_set_formats")]
    #[doc(alias = "formats")]
    pub fn set_formats(&self, formats: Option<&gdk::ContentFormats>) {
        unsafe {
            ffi::gtk_drop_target_async_set_formats(self.to_glib_none().0, formats.to_glib_none().0);
        }
    }

    #[doc(alias = "accept")]
    pub fn connect_accept<F: Fn(&Self, &gdk::Drop) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    accept_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-enter")]
    pub fn connect_drag_enter<F: Fn(&Self, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_enter_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: std::ffi::c_double,
            y: std::ffi::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-enter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drag_enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-leave")]
    pub fn connect_drag_leave<F: Fn(&Self, &gdk::Drop) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_leave_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop) + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-leave\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drag_leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drag-motion")]
    pub fn connect_drag_motion<F: Fn(&Self, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_motion_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> gdk::DragAction + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: std::ffi::c_double,
            y: std::ffi::c_double,
            f: glib::ffi::gpointer,
        ) -> gdk::ffi::GdkDragAction {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-motion\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drag_motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drop")]
    pub fn connect_drop<F: Fn(&Self, &gdk::Drop, f64, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<
            F: Fn(&DropTargetAsync, &gdk::Drop, f64, f64) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTargetAsync,
            drop: *mut gdk::ffi::GdkDrop,
            x: std::ffi::c_double,
            y: std::ffi::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(drop), x, y).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "actions")]
    pub fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<F: Fn(&DropTargetAsync) + 'static>(
            this: *mut ffi::GtkDropTargetAsync,
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
                b"notify::actions\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_actions_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "formats")]
    pub fn connect_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&DropTargetAsync) + 'static>(
            this: *mut ffi::GtkDropTargetAsync,
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
                b"notify::formats\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for DropTargetAsync {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DropTargetAsync`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DropTargetAsyncBuilder {
    builder: glib::object::ObjectBuilder<'static, DropTargetAsync>,
}

impl DropTargetAsyncBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn actions(self, actions: gdk::DragAction) -> Self {
        Self {
            builder: self.builder.property("actions", actions),
        }
    }

    pub fn formats(self, formats: &gdk::ContentFormats) -> Self {
        Self {
            builder: self.builder.property("formats", formats.clone()),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DropTargetAsync`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DropTargetAsync {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
