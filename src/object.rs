use std::{
    fmt::{write, Display},
    ptr::NonNull,
};

use crate::byte_string::{self, Byte, ByteString};

// TODO: Need to GC these
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjPtr(NonNull<Obj>);

impl From<&ByteString> for ObjPtr {
    fn from(byte_string: &ByteString) -> Self {
        Obj::String(byte_string.into()).to_obj_ptr()
    }
}

impl From<Vec<Byte>> for ObjPtr {
    fn from(byte_string: Vec<Byte>) -> Self {
        Obj::String(byte_string).to_obj_ptr()
    }
}

impl Obj {
    pub fn to_obj_ptr(self) -> ObjPtr {
        let boxed_obj = Box::new(self);
        let raw_ptr = Box::into_raw(boxed_obj);
        let non_null_ptr = unsafe { NonNull::new_unchecked(raw_ptr) };
        ObjPtr(non_null_ptr)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Obj {
    String(Vec<Byte>),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::String(bytestring) => write!(f, "{}", std::str::from_utf8(bytestring).unwrap()),
        }
    }
}

impl Display for ObjPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let derefed_obj = unsafe { &(*self.0.as_ptr()) };
        match derefed_obj {
            Obj::String(bytestring) => write!(f, "{}", std::str::from_utf8(bytestring).unwrap()),
        }
    }
}

impl ObjPtr {
    pub fn is_string(&self) -> bool {
        let derefed_obj = unsafe { &(*self.0.as_ptr()) };
        match derefed_obj {
            Obj::String(_) => true,
        }
    }

    pub fn into_string(&self) -> &ByteString {
        let derefed_obj = unsafe { &(*self.0.as_ptr()) };
        match derefed_obj {
            Obj::String(byte_string) => byte_string,
        }
    }
}
