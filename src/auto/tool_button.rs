// This file was generated by gir (32b0f11) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use Object;
use ToolItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToolButton(Object<ffi::GtkToolButton>): ToolItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_tool_button_get_type(),
    }
}

impl ToolButton {
    pub fn new<'a, 'b, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(icon_widget: Q, label: R) -> ToolButton {
        assert_initialized_main_thread!();
        let icon_widget = icon_widget.into();
        let icon_widget = icon_widget.to_glib_none();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new(icon_widget.0, label.0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> ToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToolButtonExt {
    fn get_icon_name(&self) -> Option<String>;

    fn get_icon_widget(&self) -> Option<Widget>;

    fn get_label(&self) -> Option<String>;

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_stock_id(&self) -> Option<String>;

    fn get_use_underline(&self) -> bool;

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_icon_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, icon_widget: Q);

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q);

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P);

    fn set_use_underline(&self, use_underline: bool);

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ToolButton> + IsA<Object>> ToolButtonExt for O {
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_icon_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_icon_widget(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_stock_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_tool_button_get_stock_id(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_button_get_use_underline(self.to_glib_none().0))
        }
    }

    fn set_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_icon_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, icon_widget: Q) {
        let icon_widget = icon_widget.into();
        let icon_widget = icon_widget.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_icon_widget(self.to_glib_none().0, icon_widget.0);
        }
    }

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_label(self.to_glib_none().0, label.0);
        }
    }

    fn set_label_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, label_widget: Q) {
        let label_widget = label_widget.into();
        let label_widget = label_widget.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_label_widget(self.to_glib_none().0, label_widget.0);
        }
    }

    fn set_stock_id<'a, P: Into<Option<&'a str>>>(&self, stock_id: P) {
        let stock_id = stock_id.into();
        let stock_id = stock_id.to_glib_none();
        unsafe {
            ffi::gtk_tool_button_set_stock_id(self.to_glib_none().0, stock_id.0);
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_tool_button_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    fn connect_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "clicked",
                transmute(clicked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn clicked_trampoline<P>(this: *mut ffi::GtkToolButton, f: glib_ffi::gpointer)
where P: IsA<ToolButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ToolButton::from_glib_none(this).downcast_unchecked())
}
