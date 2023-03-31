use clroxide::{clr::Clr, primitives::wrap_unknown_ptr_in_variant};
use std::{env, ffi::c_void, fs, mem::size_of, process::exit, ptr, slice};
use windows::Win32::System::{
    Com::{VARIANT, VARIANT_0_0_0},
    Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
};

fn main() -> Result<(), String> {
    let (path, args) = prepare_args();

    let contents = fs::read(path).expect("Unable to read file");

    let results = unsafe { run(contents, args)? };

    println!("[*] Results:\n\n{}", results);

    Ok(())
}

pub unsafe fn run(contents: Vec<u8>, args: Vec<String>) -> Result<String, String> {
    let mut clr = Clr::context_only(None)?;

    clr.redirect_output()?;

    /// Get the assemblies we need
    let mut context = clr.get_context()?;
    let app_domain = context.app_domain;
    let mscorlib = (*app_domain).load_library("mscorlib")?;

    /// Get all the necessary pieces to reach the function pointer for System.Environment.Exit
    let environment = (*mscorlib).get_type("System.Environment")?;
    let exit_fn = (*environment).get_method("Exit")?;
    let method_info = (*mscorlib).get_type("System.Reflection.MethodInfo")?;
    let method_handle = (*method_info).get_property("MethodHandle")?;
    let exit_fn_instance = wrap_unknown_ptr_in_variant(exit_fn as *mut c_void);
    let method_handle_value = (*method_handle).get_value(Some(exit_fn_instance))?;
    let runtime_method_handle = (*mscorlib).get_type("System.RuntimeMethodHandle")?;
    let get_func_pointer = (*runtime_method_handle).get_method("GetFunctionPointer")?;
    let pointer_variant = (*get_func_pointer).invoke_without_args(Some(method_handle_value))?;

    /// Get the actual pointer value
    let mut base_ptr = pointer_variant.Anonymous.Anonymous.Anonymous.byref;
    let mut exit_ptr = pointer_variant.Anonymous.Anonymous.Anonymous.byref;

    let value = ptr::read(exit_ptr as *mut u8);
    println!("[+] `System.Environment.Exit` is: `0x{:x}`", value);

    let mut old: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);
    let mut restored: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);

    println!("[+] Patching `System.Environment.Exit`");

    if VirtualProtect(base_ptr as *const c_void, 1, PAGE_READWRITE, &mut old).0 != 1 {
        return Err("[!] Unable to change memory permissions at the function pointer for `System.Environment.Exit`".into());
    }

    ptr::write(exit_ptr as *mut u8, 0xc3);

    let value = ptr::read(exit_ptr as *mut u8);
    println!(
        "[+] `System.Environment.Exit` was patched to: `0x{:x}`",
        value
    );

    if VirtualProtect(base_ptr as *const c_void, 1, old, &mut restored).0 != 1 {
        println!("[!] Unable to change memory permissions at the function pointer back to the original value");
    }

    println!("[+] Executing the given assembly");

    let context = clr.get_context()?;
    let assembly = (*(&context).app_domain).load_assembly(&contents)?;

    unsafe { (*assembly).run_entrypoint(&args)? };

    clr.restore_output()?;

    clr.get_redirected_output()
}

fn prepare_args() -> (String, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a path to a dotnet executable");

        exit(1)
    }

    let mut command_args: Vec<String> = vec![];

    if args.len() > 2 {
        command_args = args.split_off(2)
    }

    let path = args[1].clone();

    println!("[+] Running `{}` with given args: {:?}", path, command_args);

    return (path, command_args);
}
