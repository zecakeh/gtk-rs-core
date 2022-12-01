// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GSocketControlMessage")]
    pub struct SocketControlMessage(Object<ffi::GSocketControlMessage, ffi::GSocketControlMessageClass>);

    match fn {
        type_ => || ffi::g_socket_control_message_get_type(),
    }
}

impl SocketControlMessage {
    pub const NONE: Option<&'static SocketControlMessage> = None;

    #[doc(alias = "g_socket_control_message_deserialize")]
    pub fn deserialize(level: i32, type_: i32, data: &[u8]) -> Option<SocketControlMessage> {
        let size = data.len() as _;
        unsafe {
            from_glib_full(ffi::g_socket_control_message_deserialize(
                level,
                type_,
                size,
                data.to_glib_none().0 as glib::ffi::gpointer,
            ))
        }
    }
}

pub trait SocketControlMessageExt: 'static {
    #[doc(alias = "g_socket_control_message_get_level")]
    #[doc(alias = "get_level")]
    fn level(&self) -> i32;

    #[doc(alias = "g_socket_control_message_get_msg_type")]
    #[doc(alias = "get_msg_type")]
    fn msg_type(&self) -> i32;

    #[doc(alias = "g_socket_control_message_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> usize;
}

impl<O: IsA<SocketControlMessage>> SocketControlMessageExt for O {
    fn level(&self) -> i32 {
        unsafe { ffi::g_socket_control_message_get_level(self.as_ref().to_glib_none().0) }
    }

    fn msg_type(&self) -> i32 {
        unsafe { ffi::g_socket_control_message_get_msg_type(self.as_ref().to_glib_none().0) }
    }

    fn size(&self) -> usize {
        unsafe { ffi::g_socket_control_message_get_size(self.as_ref().to_glib_none().0) }
    }
}

impl fmt::Display for SocketControlMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketControlMessage")
    }
}
