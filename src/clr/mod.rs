use crate::primitives::{
    ICLRMetaHost, ICLRRuntimeInfo, ICorRuntimeHost, IUnknown, Interface, _AppDomain, _MethodInfo,
    _StringWriter, wrap_method_arguments, GUID, HRESULT,
};
use std::{ffi::c_void, ptr};
use windows::Win32::System::Com::VARIANT;
#[cfg(feature = "default-loader")]
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};

pub struct Clr {
    contents: Vec<u8>,
    arguments: Vec<String>,
    create_interface: isize,
    context: Option<ClrContext>,
    output_context: Option<OutputContext>,
}

pub struct ClrContext {
    pub has_started: bool,
    pub host: *mut ICLRMetaHost,
    pub runtime_info: *mut ICLRRuntimeInfo,
    pub runtime_host: *mut ICorRuntimeHost,
    pub app_domain: *mut _AppDomain,
}

pub struct OutputContext {
    pub set_out: *mut _MethodInfo,
    pub original_stdout: VARIANT,
    pub redirected_stdout: VARIANT,
}

impl Clr {
    #[cfg(feature = "default-loader")]
    pub fn new(contents: Vec<u8>, arguments: Vec<String>) -> Result<Clr, String> {
        let create_interface = load_function("mscoree.dll", "CreateInterface")?;

        Ok(Clr {
            contents,
            arguments,
            create_interface,
            context: None,
            output_context: None,
        })
    }

    #[cfg(feature = "default-loader")]
    pub fn context_only() -> Result<Clr, String> {
        let create_interface = load_function("mscoree.dll", "CreateInterface")?;

        Ok(Clr {
            contents: vec![],
            arguments: vec![],
            create_interface,
            context: None,
            output_context: None,
        })
    }

    #[cfg(not(feature = "default-loader"))]
    pub fn new(
        contents: Vec<u8>,
        arguments: Vec<String>,
        load_function: fn() -> Result<isize, String>,
    ) -> Result<Clr, String> {
        let create_interface = load_function()?;

        Ok(Clr {
            contents,
            arguments,
            create_interface,
            context: None,
            output_context: None,
        })
    }

    #[cfg(not(feature = "default-loader"))]
    pub fn context_only(load_function: fn() -> Result<isize, String>) -> Result<Clr, String> {
        let create_interface = load_function()?;

        Ok(Clr {
            contents: vec![],
            arguments: vec![],
            create_interface,
            context: None,
            output_context: None,
        })
    }

    pub fn using_runtime_host<T>(
        &mut self,
        callback: fn(*mut ICorRuntimeHost) -> Result<T, String>,
    ) -> Result<T, String> {
        let context = self.get_context()?;
        let runtime_host = context.runtime_host;

        callback(runtime_host)
    }

    pub fn use_app_domain(&mut self, app_domain: *mut _AppDomain) -> Result<(), String> {
        if self.context.is_none() {
            return Err("CLR Context has not been initialized".into());
        }

        let context = self.context.as_mut().unwrap();

        context.app_domain = app_domain;

        Ok(())
    }

    pub fn run(&mut self) -> Result<String, String> {
        self.redirect_output()?;

        let context = self.get_context()?;
        let assembly = unsafe { (*(&context).app_domain).load_assembly(&self.contents)? };

        unsafe { (*assembly).run_entrypoint(&self.arguments)? };

        self.restore_output()?;

        self.get_redirected_output()
    }

    pub fn redirect_output(&mut self) -> Result<(), String> {
        let context = self.get_context()?;

        let assembly = unsafe { (*(&context).app_domain).load_library("mscorlib")? };

        let console = unsafe { (*assembly).get_type("System.Console")? };

        let get_out = unsafe { (*console).get_method("get_Out")? };

        let set_out = unsafe { (*console).get_method("SetOut")? };

        let old_console = unsafe { (*get_out).invoke_without_args(None)? };

        let string_writer_instance =
            unsafe { (*assembly).create_instance("System.IO.StringWriter")? };

        let method_args = wrap_method_arguments(vec![string_writer_instance.clone()])?;

        unsafe { (*set_out).invoke(method_args, None)? };

        self.output_context = Some(OutputContext {
            set_out,
            original_stdout: old_console,
            redirected_stdout: string_writer_instance,
        });

        Ok(())
    }

    pub fn restore_output(&mut self) -> Result<(), String> {
        if self.output_context.is_none() {
            return Err("Output context has not been initialized".into());
        }

        let context = self.output_context.as_ref().unwrap();

        let method_args = wrap_method_arguments(vec![context.original_stdout.clone()])?;

        unsafe { (*(&context).set_out).invoke(method_args, None)? };

        Ok(())
    }

    pub fn get_redirected_output(&mut self) -> Result<String, String> {
        if self.output_context.is_none() {
            return Err("Output context has not been initialized".into());
        }

        let context = self.output_context.as_ref().unwrap();

        let dispatchable: &*mut IUnknown = unsafe {
            std::mem::transmute(
                context
                    .redirected_stdout
                    .Anonymous
                    .Anonymous
                    .Anonymous
                    .pdispVal
                    .as_ref()
                    .unwrap(),
            )
        };
        let mut string_writer_ptr: *mut _StringWriter = ptr::null_mut();

        if dispatchable.is_null() {
            return Err("Could not retrieve StringWriter".into());
        }

        let hr = unsafe {
            (**dispatchable).QueryInterface(
                &_StringWriter::IID,
                &mut string_writer_ptr as *mut *mut _ as *mut *mut c_void,
            )
        };

        if hr.is_err() {
            return Err(format!("Error while retrieving StringWriter: 0x{:x}", hr.0));
        }

        if string_writer_ptr.is_null() {
            return Err("Could not retrieve StringWriter".into());
        }

        Ok(unsafe { (*string_writer_ptr).to_string()? })
    }

    pub fn get_context(&mut self) -> Result<&ClrContext, String> {
        if self.context.is_some() {
            return Ok(self.context.as_ref().unwrap());
        }

        let host = self.get_clr_host()?;
        let runtime_info = unsafe { (*host).get_first_available_runtime()? };
        let runtime_host = unsafe { (*runtime_info).get_runtime_host()? };

        unsafe { (*runtime_host).start()? };

        let app_domain = unsafe { (*runtime_host).get_default_domain()? };

        self.context = Some(ClrContext {
            has_started: true,
            host,
            runtime_info,
            runtime_host,
            app_domain,
        });

        Ok(self.context.as_ref().unwrap())
    }

    fn get_clr_host(&self) -> Result<*mut ICLRMetaHost, String> {
        pub type CreateInterface = fn(
            class_id: *const GUID,
            interface_id: *const GUID,
            interface: *mut *mut c_void,
        ) -> HRESULT;

        let create_interface: CreateInterface =
            unsafe { std::mem::transmute(self.create_interface) };

        let host: *mut ICLRMetaHost = ICLRMetaHost::new(create_interface)?;

        return Ok(host);
    }
}

#[cfg(feature = "default-loader")]
fn load_function(library_name: &str, function_name: &str) -> Result<isize, String> {
    let library = match unsafe {
        LoadLibraryA(windows::core::PCSTR::from_raw(
            format!("{}\0", library_name).as_ptr(),
        ))
    } {
        Ok(hinstance) => hinstance,
        Err(e) => return Err(format!("Error while loading `{}`: {}", library_name, e)),
    };

    return match unsafe {
        GetProcAddress(
            library,
            windows::core::PCSTR::from_raw(format!("{}\0", function_name).as_ptr()),
        )
    } {
        None => Err(format!(
            "Could not locate `{}` in `{}`",
            function_name, library_name
        )),
        Some(f) => Ok(f as isize),
    };
}
