// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use gio;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;
use Display;
use Screen;

glib_wrapper! {
    pub struct AppLaunchContext(Object<gdk_sys::GdkAppLaunchContext, AppLaunchContextClass>) @extends gio::AppLaunchContext;

    match fn {
        get_type => || gdk_sys::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn set_desktop(&self, desktop: i32) {
        unsafe {
            gdk_sys::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            gdk_sys::gdk_app_launch_context_set_icon(
                self.to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gdk_sys::gdk_app_launch_context_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    pub fn set_screen(&self, screen: &Screen) {
        unsafe {
            gdk_sys::gdk_app_launch_context_set_screen(
                self.to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            gdk_sys::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }

    pub fn get_property_display(&self) -> Option<Display> {
        unsafe {
            let mut value = Value::from_type(<Display as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"display\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `display` getter")
        }
    }
}

impl fmt::Display for AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppLaunchContext")
    }
}