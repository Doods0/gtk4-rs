// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkShortcutLabel")]
    pub struct ShortcutLabel(Object<ffi::GtkShortcutLabel, ffi::GtkShortcutLabelClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_shortcut_label_get_type(),
    }
}

impl ShortcutLabel {
    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcut_label_new")]
    pub fn new(accelerator: &str) -> ShortcutLabel {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_shortcut_label_new(accelerator.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ShortcutLabel`] objects.
    ///
    /// This method returns an instance of [`ShortcutLabelBuilder`](crate::builders::ShortcutLabelBuilder) which can be used to create [`ShortcutLabel`] objects.
    pub fn builder() -> ShortcutLabelBuilder {
        ShortcutLabelBuilder::new()
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcut_label_get_accelerator")]
    #[doc(alias = "get_accelerator")]
    pub fn accelerator(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_shortcut_label_get_accelerator(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcut_label_get_disabled_text")]
    #[doc(alias = "get_disabled_text")]
    #[doc(alias = "disabled-text")]
    pub fn disabled_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_shortcut_label_get_disabled_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcut_label_set_accelerator")]
    #[doc(alias = "accelerator")]
    pub fn set_accelerator(&self, accelerator: &str) {
        unsafe {
            ffi::gtk_shortcut_label_set_accelerator(
                self.to_glib_none().0,
                accelerator.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcut_label_set_disabled_text")]
    #[doc(alias = "disabled-text")]
    pub fn set_disabled_text(&self, disabled_text: &str) {
        unsafe {
            ffi::gtk_shortcut_label_set_disabled_text(
                self.to_glib_none().0,
                disabled_text.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "accelerator")]
    pub fn connect_accelerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accelerator_trampoline<F: Fn(&ShortcutLabel) + 'static>(
            this: *mut ffi::GtkShortcutLabel,
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
                c"notify::accelerator".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_accelerator_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "disabled-text")]
    pub fn connect_disabled_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_disabled_text_trampoline<F: Fn(&ShortcutLabel) + 'static>(
            this: *mut ffi::GtkShortcutLabel,
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
                c"notify::disabled-text".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_disabled_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ShortcutLabel {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ShortcutLabel`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShortcutLabelBuilder {
    builder: glib::object::ObjectBuilder<'static, ShortcutLabel>,
}

impl ShortcutLabelBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn accelerator(self, accelerator: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("accelerator", accelerator.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn disabled_text(self, disabled_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("disabled-text", disabled_text.into()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ShortcutLabel`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ShortcutLabel {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
