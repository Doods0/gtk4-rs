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
    #[doc(alias = "GtkEventControllerLegacy")]
    pub struct EventControllerLegacy(Object<ffi::GtkEventControllerLegacy, ffi::GtkEventControllerLegacyClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_legacy_get_type(),
    }
}

impl EventControllerLegacy {
    #[doc(alias = "gtk_event_controller_legacy_new")]
    pub fn new() -> EventControllerLegacy {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_legacy_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerLegacy`] objects.
    ///
    /// This method returns an instance of [`EventControllerLegacyBuilder`](crate::builders::EventControllerLegacyBuilder) which can be used to create [`EventControllerLegacy`] objects.
    pub fn builder() -> EventControllerLegacyBuilder {
        EventControllerLegacyBuilder::new()
    }

    #[doc(alias = "event")]
    pub fn connect_event<F: Fn(&Self, &gdk::Event) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            F: Fn(&EventControllerLegacy, &gdk::Event) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkEventControllerLegacy,
            event: *mut gdk::ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(event)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"event".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    event_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerLegacy {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerLegacy`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EventControllerLegacyBuilder {
    builder: glib::object::ObjectBuilder<'static, EventControllerLegacy>,
}

impl EventControllerLegacyBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
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
    /// Build the [`EventControllerLegacy`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EventControllerLegacy {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
