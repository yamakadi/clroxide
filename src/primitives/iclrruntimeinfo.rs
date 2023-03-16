use crate::primitives::{
    Class, ICorRuntimeHost, IUnknown, IUnknownVtbl, Interface, BOOL, GUID, HANDLE, HRESULT,
};
use std::{ffi::c_void, ops::Deref, ptr};

#[repr(C)]
pub struct ICLRRuntimeInfo {
    pub vtable: *const ICLRRuntimeInfoVtbl,
}

#[repr(C)]
pub struct ICLRRuntimeInfoVtbl {
    pub parent: IUnknownVtbl,
    pub GetVersionString: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
    ) -> HRESULT,
    pub GetRuntimeDirectory: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
    ) -> HRESULT,
    pub IsLoaded: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        hndProcess: HANDLE,
        pbLoaded: *mut BOOL,
    ) -> HRESULT,
    pub LoadErrorString: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        iResourceID: u32,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
        iLocaleID: u32,
    ) -> HRESULT,
    pub LoadLibrary: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pwzDllName: *const u16,
        ppProc: *mut *mut c_void,
    ) -> HRESULT,
    pub GetProcAddress: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pszProcName: *const i8,
        ppProc: *mut *mut c_void,
    ) -> HRESULT,
    pub GetInterface: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        rclsid: *const GUID,
        riid: *const GUID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT,
    pub IsLoadable:
        unsafe extern "system" fn(This: *mut ICLRRuntimeInfo, pbLoadable: *mut BOOL) -> HRESULT,
    pub SetDefaultStartupFlags: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        dwStartupFlags: u32,
        pwzHostConfigFile: *const u16,
    ) -> HRESULT,
    pub GetDefaultStartupFlags: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pdwStartupFlags: *mut u32,
        pwzHostConfigFile: *mut u16,
        pcchHostConfigFile: *mut u32,
    ) -> HRESULT,
    pub BindAsLegacyV2Runtime: unsafe extern "system" fn(This: *mut ICLRRuntimeInfo) -> HRESULT,
    pub IsStarted: unsafe extern "system" fn(
        This: *mut ICLRRuntimeInfo,
        pbStarted: *mut BOOL,
        pdwStartupFlags: *mut u32,
    ) -> HRESULT,
}

impl ICLRRuntimeInfo {
    pub fn get_runtime_host(&self) -> Result<*mut ICorRuntimeHost, String> {
        let mut ppv: *mut ICorRuntimeHost = ptr::null_mut();

        let hr = unsafe {
            (*self).GetInterface(
                &ICorRuntimeHost::CLSID,
                &ICorRuntimeHost::IID,
                &mut ppv as *mut *mut _ as *mut *mut c_void,
            )
        };

        if hr.is_err() {
            return Err(format!("Could not retrieve ICorRuntimeHost: {:?}", hr));
        }

        if ppv.is_null() {
            return Err("Could not retrieve ICorRuntimeHost".into());
        }

        return Ok(ppv);
    }

    pub fn has_started() -> Result<bool, String> {
        todo!()
    }

    #[inline]
    pub unsafe fn GetVersionString(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT {
        ((*self.vtable).GetVersionString)(self as *const _ as *mut _, pwzBuffer, pcchBuffer)
    }

    #[inline]
    pub unsafe fn GetRuntimeDirectory(&self, pwzBuffer: *mut u16, pcchBuffer: *mut u32) -> HRESULT {
        ((*self.vtable).GetRuntimeDirectory)(self as *const _ as *mut _, pwzBuffer, pcchBuffer)
    }

    #[inline]
    pub unsafe fn IsLoaded(&self, hndProcess: HANDLE, pbLoaded: *mut BOOL) -> HRESULT {
        ((*self.vtable).IsLoaded)(self as *const _ as *mut _, hndProcess, pbLoaded)
    }

    #[inline]
    pub unsafe fn LoadErrorString(
        &self,
        iResourceID: u32,
        pwzBuffer: *mut u16,
        pcchBuffer: *mut u32,
        iLocaleID: u32,
    ) -> HRESULT {
        ((*self.vtable).LoadErrorString)(
            self as *const _ as *mut _,
            iResourceID,
            pwzBuffer,
            pcchBuffer,
            iLocaleID,
        )
    }

    #[inline]
    pub unsafe fn LoadLibrary(&self, pwzDllName: *const u16, ppProc: *mut *mut c_void) -> HRESULT {
        ((*self.vtable).LoadLibrary)(self as *const _ as *mut _, pwzDllName, ppProc)
    }

    #[inline]
    pub unsafe fn GetProcAddress(
        &self,
        pszProcName: *const i8,
        ppProc: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).GetProcAddress)(self as *const _ as *mut _, pszProcName, ppProc)
    }

    #[inline]
    pub unsafe fn GetInterface(
        &self,
        rclsid: *const GUID,
        riid: *const GUID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).GetInterface)(self as *const _ as *mut _, rclsid, riid, ppUnk)
    }

    #[inline]
    pub unsafe fn IsLoadable(&self, pbLoadable: *mut BOOL) -> HRESULT {
        ((*self.vtable).IsLoadable)(self as *const _ as *mut _, pbLoadable)
    }

    #[inline]
    pub unsafe fn SetDefaultStartupFlags(
        &self,
        dwStartupFlags: u32,
        pwzHostConfigFile: *const u16,
    ) -> HRESULT {
        ((*self.vtable).SetDefaultStartupFlags)(
            self as *const _ as *mut _,
            dwStartupFlags,
            pwzHostConfigFile,
        )
    }

    #[inline]
    pub unsafe fn GetDefaultStartupFlags(
        &self,
        pdwStartupFlags: *mut u32,
        pwzHostConfigFile: *mut u16,
        pcchHostConfigFile: *mut u32,
    ) -> HRESULT {
        ((*self.vtable).GetDefaultStartupFlags)(
            self as *const _ as *mut _,
            pdwStartupFlags,
            pwzHostConfigFile,
            pcchHostConfigFile,
        )
    }

    #[inline]
    pub unsafe fn BindAsLegacyV2Runtime(&self) -> HRESULT {
        ((*self.vtable).BindAsLegacyV2Runtime)(self as *const _ as *mut _)
    }

    #[inline]
    pub unsafe fn IsStarted(&self, pbStarted: *mut BOOL, pdwStartupFlags: *mut u32) -> HRESULT {
        ((*self.vtable).IsStarted)(self as *const _ as *mut _, pbStarted, pdwStartupFlags)
    }
}

impl Interface for ICLRRuntimeInfo {
    const IID: GUID = GUID::from_values(
        0xBD39D1D2,
        0xBA2F,
        0x486a,
        [0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91],
    );

    fn vtable(&self) -> *const c_void {
        self.vtable as *const _ as *const c_void
    }
}

impl Deref for ICLRRuntimeInfo {
    type Target = IUnknown;

    #[inline]
    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ICLRRuntimeInfo as *const IUnknown) }
    }
}
