// This file was generated by gir (32b0f11) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_10")]
use BaselinePosition;
use Container;
use Orientable;
use Orientation;
use PackType;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct Box(Object<ffi::GtkBox>): Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_box_get_type(),
    }
}

impl Box {
    pub fn new(orientation: Orientation, spacing: i32) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_box_new(orientation.to_glib(), spacing)).downcast_unchecked()
        }
    }
}

pub trait BoxExt {
    #[cfg(feature = "v3_10")]
    fn get_baseline_position(&self) -> BaselinePosition;

    #[cfg(feature = "v3_12")]
    fn get_center_widget(&self) -> Option<Widget>;

    fn get_homogeneous(&self) -> bool;

    fn get_spacing(&self) -> i32;

    fn pack_end<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32);

    fn pack_start<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32);

    fn query_child_packing<P: IsA<Widget>>(&self, child: &P) -> (bool, bool, u32, PackType);

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32);

    #[cfg(feature = "v3_10")]
    fn set_baseline_position(&self, position: BaselinePosition);

    #[cfg(feature = "v3_12")]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    fn set_child_packing<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32, pack_type: PackType);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_spacing(&self, spacing: i32);

    #[doc(hidden)]
    fn get_child_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_expand<T: IsA<Widget>>(&self, item: &T, expand: bool);

    #[doc(hidden)]
    fn get_child_fill<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_fill<T: IsA<Widget>>(&self, item: &T, fill: bool);

    #[doc(hidden)]
    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType;

    #[doc(hidden)]
    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType);

    #[doc(hidden)]
    fn get_child_padding<T: IsA<Widget>>(&self, item: &T) -> u32;

    #[doc(hidden)]
    fn set_child_padding<T: IsA<Widget>>(&self, item: &T, padding: u32);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);
}

impl<O: IsA<Box> + IsA<Container>> BoxExt for O {
    #[cfg(feature = "v3_10")]
    fn get_baseline_position(&self) -> BaselinePosition {
        unsafe {
            from_glib(ffi::gtk_box_get_baseline_position(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_box_get_center_widget(self.to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_box_get_homogeneous(self.to_glib_none().0))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_box_get_spacing(self.to_glib_none().0)
        }
    }

    fn pack_end<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_end(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn pack_start<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32) {
        unsafe {
            ffi::gtk_box_pack_start(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding);
        }
    }

    fn query_child_packing<P: IsA<Widget>>(&self, child: &P) -> (bool, bool, u32, PackType) {
        unsafe {
            let mut expand = mem::uninitialized();
            let mut fill = mem::uninitialized();
            let mut padding = mem::uninitialized();
            let mut pack_type = mem::uninitialized();
            ffi::gtk_box_query_child_packing(self.to_glib_none().0, child.to_glib_none().0, &mut expand, &mut fill, &mut padding, &mut pack_type);
            (from_glib(expand), from_glib(fill), padding, from_glib(pack_type))
        }
    }

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            ffi::gtk_box_reorder_child(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_baseline_position(&self, position: BaselinePosition) {
        unsafe {
            ffi::gtk_box_set_baseline_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[cfg(feature = "v3_12")]
    fn set_center_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_box_set_center_widget(self.to_glib_none().0, widget.0);
        }
    }

    fn set_child_packing<P: IsA<Widget>>(&self, child: &P, expand: bool, fill: bool, padding: u32, pack_type: PackType) {
        unsafe {
            ffi::gtk_box_set_child_packing(self.to_glib_none().0, child.to_glib_none().0, expand.to_glib(), fill.to_glib(), padding, pack_type.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(hidden)]
    fn get_child_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "expand".to_glib_none().0, Value::from(&expand).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_fill<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_fill<T: IsA<Widget>>(&self, item: &T, fill: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "fill".to_glib_none().0, Value::from(&fill).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_pack_type<T: IsA<Widget>>(&self, item: &T) -> PackType {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    #[doc(hidden)]
    fn set_child_pack_type<T: IsA<Widget>>(&self, item: &T, pack_type: PackType) {
        let pack_type = pack_type.to_glib() as i32;
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "pack-type".to_glib_none().0, Value::from(&pack_type).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_padding<T: IsA<Widget>>(&self, item: &T) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "padding".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_padding<T: IsA<Widget>>(&self, item: &T, padding: u32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "padding".to_glib_none().0, Value::from(&padding).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }
}
