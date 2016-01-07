// This file was generated by gir (4cd15d1) from gir-files (11e0e6d)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Rectangle;
use SensitivityType;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Range(Object<ffi::GtkRange>): Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_range_get_type(),
    }
}

pub trait RangeExt {
    fn get_adjustment(&self) -> Option<Adjustment>;
    fn get_fill_level(&self) -> f64;
    fn get_flippable(&self) -> bool;
    fn get_inverted(&self) -> bool;
    fn get_lower_stepper_sensitivity(&self) -> SensitivityType;
    fn get_min_slider_size(&self) -> i32;
    fn get_range_rect(&self) -> Rectangle;
    fn get_restrict_to_fill_level(&self) -> bool;
    fn get_round_digits(&self) -> i32;
    fn get_show_fill_level(&self) -> bool;
    fn get_slider_range(&self) -> (i32, i32);
    fn get_slider_size_fixed(&self) -> bool;
    fn get_upper_stepper_sensitivity(&self) -> SensitivityType;
    fn get_value(&self) -> f64;
    fn set_adjustment(&self, adjustment: &Adjustment);
    fn set_fill_level(&self, fill_level: f64);
    fn set_flippable(&self, flippable: bool);
    fn set_increments(&self, step: f64, page: f64);
    fn set_inverted(&self, setting: bool);
    fn set_lower_stepper_sensitivity(&self, sensitivity: SensitivityType);
    fn set_min_slider_size(&self, min_size: i32);
    fn set_range(&self, min: f64, max: f64);
    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool);
    fn set_round_digits(&self, round_digits: i32);
    fn set_show_fill_level(&self, show_fill_level: bool);
    fn set_slider_size_fixed(&self, size_fixed: bool);
    fn set_upper_stepper_sensitivity(&self, sensitivity: SensitivityType);
    fn set_value(&self, value: f64);
}

impl<O: Upcast<Range>> RangeExt for O {
    fn get_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(ffi::gtk_range_get_adjustment(self.to_glib_none().0))
        }
    }

    fn get_fill_level(&self) -> f64 {
        unsafe {
            ffi::gtk_range_get_fill_level(self.to_glib_none().0)
        }
    }

    fn get_flippable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_flippable(self.to_glib_none().0))
        }
    }

    fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_inverted(self.to_glib_none().0))
        }
    }

    fn get_lower_stepper_sensitivity(&self) -> SensitivityType {
        unsafe {
            ffi::gtk_range_get_lower_stepper_sensitivity(self.to_glib_none().0)
        }
    }

    fn get_min_slider_size(&self) -> i32 {
        unsafe {
            ffi::gtk_range_get_min_slider_size(self.to_glib_none().0)
        }
    }

    fn get_range_rect(&self) -> Rectangle {
        unsafe {
            let mut range_rect = Rectangle::uninitialized();
            ffi::gtk_range_get_range_rect(self.to_glib_none().0, range_rect.to_glib_none_mut().0);
            range_rect
        }
    }

    fn get_restrict_to_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_restrict_to_fill_level(self.to_glib_none().0))
        }
    }

    fn get_round_digits(&self) -> i32 {
        unsafe {
            ffi::gtk_range_get_round_digits(self.to_glib_none().0)
        }
    }

    fn get_show_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_show_fill_level(self.to_glib_none().0))
        }
    }

    fn get_slider_range(&self) -> (i32, i32) {
        unsafe {
            let mut slider_start = mem::uninitialized();
            let mut slider_end = mem::uninitialized();
            ffi::gtk_range_get_slider_range(self.to_glib_none().0, &mut slider_start, &mut slider_end);
            (slider_start, slider_end)
        }
    }

    fn get_slider_size_fixed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_slider_size_fixed(self.to_glib_none().0))
        }
    }

    fn get_upper_stepper_sensitivity(&self) -> SensitivityType {
        unsafe {
            ffi::gtk_range_get_upper_stepper_sensitivity(self.to_glib_none().0)
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_range_get_value(self.to_glib_none().0)
        }
    }

    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_range_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_fill_level(&self, fill_level: f64) {
        unsafe {
            ffi::gtk_range_set_fill_level(self.to_glib_none().0, fill_level);
        }
    }

    fn set_flippable(&self, flippable: bool) {
        unsafe {
            ffi::gtk_range_set_flippable(self.to_glib_none().0, flippable.to_glib());
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_range_set_increments(self.to_glib_none().0, step, page);
        }
    }

    fn set_inverted(&self, setting: bool) {
        unsafe {
            ffi::gtk_range_set_inverted(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_lower_stepper_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_range_set_lower_stepper_sensitivity(self.to_glib_none().0, sensitivity);
        }
    }

    fn set_min_slider_size(&self, min_size: i32) {
        unsafe {
            ffi::gtk_range_set_min_slider_size(self.to_glib_none().0, min_size);
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_range_set_range(self.to_glib_none().0, min, max);
        }
    }

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_restrict_to_fill_level(self.to_glib_none().0, restrict_to_fill_level.to_glib());
        }
    }

    fn set_round_digits(&self, round_digits: i32) {
        unsafe {
            ffi::gtk_range_set_round_digits(self.to_glib_none().0, round_digits);
        }
    }

    fn set_show_fill_level(&self, show_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_show_fill_level(self.to_glib_none().0, show_fill_level.to_glib());
        }
    }

    fn set_slider_size_fixed(&self, size_fixed: bool) {
        unsafe {
            ffi::gtk_range_set_slider_size_fixed(self.to_glib_none().0, size_fixed.to_glib());
        }
    }

    fn set_upper_stepper_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_range_set_upper_stepper_sensitivity(self.to_glib_none().0, sensitivity);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_range_set_value(self.to_glib_none().0, value);
        }
    }

}
