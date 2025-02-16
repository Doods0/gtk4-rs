// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, TreeModel};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkTreeSortable")]
    pub struct TreeSortable(Interface<ffi::GtkTreeSortable, ffi::GtkTreeSortableIface>) @requires TreeModel;

    match fn {
        type_ => || ffi::gtk_tree_sortable_get_type(),
    }
}

impl TreeSortable {
    pub const NONE: Option<&'static TreeSortable> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TreeSortable>> Sealed for T {}
}

pub trait TreeSortableExt: IsA<TreeSortable> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_sortable_has_default_sort_func")]
    fn has_default_sort_func(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_sortable_has_default_sort_func(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_sortable_sort_column_changed")]
    fn sort_column_changed(&self) {
        unsafe {
            ffi::gtk_tree_sortable_sort_column_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "sort-column-changed")]
    fn connect_sort_column_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sort_column_changed_trampoline<
            P: IsA<TreeSortable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkTreeSortable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TreeSortable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"sort-column-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    sort_column_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TreeSortable>> TreeSortableExt for O {}
