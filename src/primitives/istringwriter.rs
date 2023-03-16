use crate::primitives::{IUnknown, IUnknownVtbl, Interface, GUID, HRESULT};
use std::{ffi::c_void, ops::Deref};
use windows::core::BSTR;

#[repr(C)]
pub struct _StringWriter {
    pub vtable: *const _StringWriterVtbl,
}

#[repr(C)]
pub struct _StringWriterVtbl {
    pub parent: IUnknownVtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
}

impl _StringWriter {
    pub fn to_string(&self) -> Result<String, String> {
        let mut buffer = BSTR::new();

        let hr = unsafe { (*self).ToString(&mut buffer as *mut _ as *mut *mut u16) };

        if hr.is_err() {
            return Err(format!("Failed while running `ToString`: {:?}", hr));
        }

        Ok(buffer.to_string())
    }

    #[inline]
    pub unsafe fn ToString(&self, pRetVal: *mut *mut u16) -> HRESULT {
        ((*self.vtable).ToString)(self as *const _ as *mut _, pRetVal)
    }
}

impl Interface for _StringWriter {
    const IID: GUID = GUID::from_values(
        0xcb9f94c0,
        0xd691,
        0x3b62,
        [0xb0, 0xb2, 0x3c, 0xe5, 0x30, 0x9c, 0xfa, 0x62],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for _StringWriter {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const _StringWriter as *const IUnknown) }
    }
}
