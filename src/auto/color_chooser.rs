// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use gdk;
use gdk_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ColorChooser(Object<ffi::GtkColorChooser>);

    match fn {
        get_type => || ffi::gtk_color_chooser_get_type(),
    }
}

pub trait ColorChooserExt {
    fn get_rgba(&self) -> gdk::RGBA;

    fn get_use_alpha(&self) -> bool;

    fn set_rgba(&self, color: &gdk::RGBA);

    fn set_use_alpha(&self, use_alpha: bool);

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> u64;

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ColorChooser> + IsA<glib::object::Object>> ColorChooserExt for O {
    fn get_rgba(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_color_chooser_get_rgba(self.to_glib_none().0, color.to_glib_none_mut().0);
            color
        }
    }

    fn get_use_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_chooser_get_use_alpha(self.to_glib_none().0))
        }
    }

    fn set_rgba(&self, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_color_chooser_set_rgba(self.to_glib_none().0, color.to_glib_none().0);
        }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(self.to_glib_none().0, use_alpha.to_glib());
        }
    }

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::RGBA) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "color-activated",
                transmute(color_activated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rgba",
                transmute(notify_rgba_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-alpha",
                transmute(notify_use_alpha_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn color_activated_trampoline<P>(this: *mut ffi::GtkColorChooser, color: *mut gdk_ffi::GdkRGBA, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    callback_guard!();
    let f: &&(Fn(&P, &gdk::RGBA) + 'static) = transmute(f);
    f(&ColorChooser::from_glib_none(this).downcast_unchecked(), &from_glib_none(color))
}

unsafe extern "C" fn notify_rgba_trampoline<P>(this: *mut ffi::GtkColorChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ColorChooser::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_alpha_trampoline<P>(this: *mut ffi::GtkColorChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ColorChooser::from_glib_none(this).downcast_unchecked())
}
