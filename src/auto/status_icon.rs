// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use ImageType;
use Menu;
use Orientation;
use Tooltip;
use ffi;
use gdk;
use gdk_ffi;
use gdk_pixbuf;
use gio;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct StatusIcon(Object<ffi::GtkStatusIcon>);

    match fn {
        get_type => || ffi::gtk_status_icon_get_type(),
    }
}

impl StatusIcon {
    pub fn new() -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new())
        }
    }

    pub fn new_from_file<P: AsRef<std::path::Path>>(filename: P) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_file(filename.as_ref().to_glib_none().0))
        }
    }

    pub fn new_from_gicon<P: IsA<gio::Icon>>(icon: &P) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_gicon(icon.to_glib_none().0))
        }
    }

    pub fn new_from_icon_name(icon_name: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_icon_name(icon_name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_pixbuf(pixbuf.to_glib_none().0))
        }
    }

    pub fn new_from_stock(stock_id: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_stock(stock_id.to_glib_none().0))
        }
    }

    pub fn position_menu<P: IsA<Menu>>(menu: &P, x: &mut i32, y: &mut i32, user_data: &StatusIcon) -> bool {
        skip_assert_initialized!();
        unsafe {
            let mut push_in = mem::uninitialized();
            ffi::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, user_data.to_glib_none().0);
            from_glib(push_in)
        }
    }
}

impl Default for StatusIcon {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StatusIconExt {
    fn get_geometry(&self) -> Option<(gdk::Screen, gdk::Rectangle, Orientation)>;

    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_has_tooltip(&self) -> bool;

    fn get_icon_name(&self) -> Option<String>;

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_screen(&self) -> Option<gdk::Screen>;

    fn get_size(&self) -> i32;

    fn get_stock(&self) -> Option<String>;

    fn get_storage_type(&self) -> ImageType;

    fn get_title(&self) -> Option<String>;

    fn get_tooltip_markup(&self) -> Option<String>;

    fn get_tooltip_text(&self) -> Option<String>;

    fn get_visible(&self) -> bool;

    fn get_x11_window_id(&self) -> u32;

    fn is_embedded(&self) -> bool;

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P);

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P);

    fn set_from_icon_name(&self, icon_name: &str);

    fn set_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P);

    fn set_from_stock(&self, stock_id: &str);

    fn set_has_tooltip(&self, has_tooltip: bool);

    fn set_name(&self, name: &str);

    fn set_screen(&self, screen: &gdk::Screen);

    fn set_title(&self, title: &str);

    fn set_tooltip_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P);

    fn set_tooltip_text(&self, text: &str);

    fn set_visible(&self, visible: bool);

    fn get_property_embedded(&self) -> bool;

    fn set_property_file(&self, file: Option<&str>);

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>);

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_orientation(&self) -> Orientation;

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn set_property_stock(&self, stock: Option<&str>);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64;

    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_property_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<StatusIcon> + IsA<glib::object::Object>> StatusIconExt for O {
    fn get_geometry(&self) -> Option<(gdk::Screen, gdk::Rectangle, Orientation)> {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut area = gdk::Rectangle::uninitialized();
            let mut orientation = mem::uninitialized();
            let ret = from_glib(ffi::gtk_status_icon_get_geometry(self.to_glib_none().0, &mut screen, area.to_glib_none_mut().0, &mut orientation));
            if ret { Some((from_glib_none(screen), area, from_glib(orientation))) } else { None }
        }
    }

    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_gicon(self.to_glib_none().0))
        }
    }

    fn get_has_tooltip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_has_tooltip(self.to_glib_none().0))
        }
    }

    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_screen(self.to_glib_none().0))
        }
    }

    fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_status_icon_get_size(self.to_glib_none().0)
        }
    }

    fn get_stock(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_stock(self.to_glib_none().0))
        }
    }

    fn get_storage_type(&self) -> ImageType {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_storage_type(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_title(self.to_glib_none().0))
        }
    }

    fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_markup(self.to_glib_none().0))
        }
    }

    fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_text(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_visible(self.to_glib_none().0))
        }
    }

    fn get_x11_window_id(&self) -> u32 {
        unsafe {
            ffi::gtk_status_icon_get_x11_window_id(self.to_glib_none().0)
        }
    }

    fn is_embedded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_is_embedded(self.to_glib_none().0))
        }
    }

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P) {
        unsafe {
            ffi::gtk_status_icon_set_from_gicon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_from_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    fn set_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_status_icon_set_from_pixbuf(self.to_glib_none().0, pixbuf.0);
        }
    }

    fn set_from_stock(&self, stock_id: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_status_icon_set_has_tooltip(self.to_glib_none().0, has_tooltip.to_glib());
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_status_icon_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_status_icon_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_tooltip_markup<'a, P: Into<Option<&'a str>>>(&self, markup: P) {
        let markup = markup.into();
        let markup = markup.to_glib_none();
        unsafe {
            ffi::gtk_status_icon_set_tooltip_markup(self.to_glib_none().0, markup.0);
        }
    }

    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_status_icon_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    fn get_property_embedded(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "embedded".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_file(&self, file: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "file".to_glib_none().0, Value::from(file).to_glib_none().0);
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_property_orientation(&self) -> Orientation {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "orientation".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, Value::from(pixbuf).to_glib_none().0);
        }
    }

    fn set_property_stock(&self, stock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock".to_glib_none().0, Value::from(stock).to_glib_none().0);
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_button_press_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::EventButton) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "button-press-event",
                transmute(button_press_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_button_release_event<F: Fn(&Self, &gdk::EventButton) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::EventButton) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "button-release-event",
                transmute(button_release_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_popup_menu<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "popup-menu",
                transmute(popup_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_query_tooltip<F: Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, bool, &Tooltip) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "query-tooltip",
                transmute(query_tooltip_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_scroll_event<F: Fn(&Self, &gdk::EventScroll) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::EventScroll) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "scroll-event",
                transmute(scroll_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_size_changed<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "size-changed",
                transmute(size_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_embedded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::embedded",
                transmute(notify_embedded_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-tooltip",
                transmute(notify_has_tooltip_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::orientation",
                transmute(notify_orientation_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf",
                transmute(notify_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::screen",
                transmute(notify_screen_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::size",
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock",
                transmute(notify_stock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::storage-type",
                transmute(notify_storage_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-markup",
                transmute(notify_tooltip_markup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tooltip-text",
                transmute(notify_tooltip_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::visible",
                transmute(notify_visible_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkStatusIcon, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn button_press_event_trampoline<P>(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, &gdk::EventButton) -> bool + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn button_release_event_trampoline<P>(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, &gdk::EventButton) -> bool + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn popup_menu_trampoline<P>(this: *mut ffi::GtkStatusIcon, button: libc::c_uint, activate_time: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, u32, u32) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), button, activate_time)
}

unsafe extern "C" fn query_tooltip_trampoline<P>(this: *mut ffi::GtkStatusIcon, x: libc::c_int, y: libc::c_int, keyboard_mode: glib_ffi::gboolean, tooltip: *mut ffi::GtkTooltip, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, i32, i32, bool, &Tooltip) -> bool + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), x, y, from_glib(keyboard_mode), &from_glib_none(tooltip)).to_glib()
}

unsafe extern "C" fn scroll_event_trampoline<P>(this: *mut ffi::GtkStatusIcon, event: *mut gdk_ffi::GdkEventScroll, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, &gdk::EventScroll) -> bool + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), &from_glib_none(event)).to_glib()
}

unsafe extern "C" fn size_changed_trampoline<P>(this: *mut ffi::GtkStatusIcon, size: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P, i32) -> bool + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked(), size).to_glib()
}

unsafe extern "C" fn notify_embedded_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_tooltip_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_orientation_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_screen_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_storage_type_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_markup_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tooltip_text_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_visible_trampoline<P>(this: *mut ffi::GtkStatusIcon, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<StatusIcon> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&StatusIcon::from_glib_none(this).downcast_unchecked())
}
