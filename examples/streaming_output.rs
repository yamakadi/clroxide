use clroxide::{
    clr::Clr,
    primitives::{
        _MethodInfo, unpack_byte_array, wrap_bool_in_variant, wrap_i64_in_variant,
        wrap_method_arguments,
    },
};
use std::{env, fs, process::exit, thread, time::Duration};
use windows::Win32::System::Com::VARIANT;

fn main() -> Result<(), String> {
    let (path, args) = prepare_args();
    let contents = fs::read(path).expect("Unable to read file");

    let mut clr = Clr::context_only()?;
    let mut context = clr.get_context()?;
    let app_domain = context.app_domain;
    let mscorlib = unsafe { (*app_domain).load_library("mscorlib")? };

    unsafe {
        // Get memory stream type & instance
        let memory_stream = (*mscorlib).get_type("System.IO.MemoryStream")?;

        let to_array = (*memory_stream).get_method("ToArray")?;
        let set_position = (*memory_stream).get_method("set_Position")?;
        let set_length = (*memory_stream).get_method("SetLength")?;
        let memory_stream_instance = (*mscorlib).create_instance("System.IO.MemoryStream")?;

        // Get stream writer type & instance
        let stream_writer = (*mscorlib).get_type("System.IO.StreamWriter")?;
        let stream_writer_constructor =
            (*stream_writer).get_constructor_with_signature("Void .ctor(System.IO.Stream)")?;

        let stream_writer_instance = (*stream_writer_constructor)
            .invoke(wrap_method_arguments(vec![memory_stream_instance.clone()])?)?;

        // Set stream writer instance to auto flush
        let auto_flush_property = (*stream_writer).get_property("AutoFlush")?;
        (*auto_flush_property).set_value(
            wrap_bool_in_variant(true),
            Some(stream_writer_instance.clone()),
        )?;

        // Get necessary console functions and properties
        let console = unsafe { (*mscorlib).get_type("System.Console")? };
        let get_out = unsafe { (*console).get_method("get_Out")? };
        let set_out = unsafe { (*console).get_method("SetOut")? };
        let old_out = unsafe { (*get_out).invoke_without_args(None)? };
        let get_err = unsafe { (*console).get_method("get_Error")? };
        let set_err = unsafe { (*console).get_method("SetError")? };
        let old_err = unsafe { (*get_err).invoke_without_args(None)? };

        // Set stdout and stderr to our stream writer
        let method_args = wrap_method_arguments(vec![stream_writer_instance.clone()])?;
        unsafe { (*set_out).invoke(method_args, None)? };
        unsafe { (*set_err).invoke(method_args, None)? };

        let ad = unsafe { &*app_domain };

        let t_handle = thread::spawn(move || {
            let assembly = match unsafe { ad.load_assembly(&contents) } {
                Ok(a) => a,
                Err(_) => return,
            };

            let _ = unsafe { (*assembly).run_entrypoint(&args) };
        });

        println!("[*] Results:\n");

        while !t_handle.is_finished() {
            let result = (*to_array).invoke_without_args(Some(memory_stream_instance.clone()))?;
            reset_memory_stream(memory_stream_instance.clone(), set_position, set_length)?;

            let array = result.Anonymous.Anonymous.Anonymous.parray;
            let bytes = unpack_byte_array(array)?;

            print!("{}", String::from_utf8(bytes).unwrap());

            thread::sleep(Duration::from_millis(100));
        }

        let result = (*to_array).invoke_without_args(Some(memory_stream_instance.clone()))?;

        reset_memory_stream(memory_stream_instance.clone(), set_position, set_length)?;

        let array = result.Anonymous.Anonymous.Anonymous.parray;
        let bytes = unpack_byte_array(array)?;

        print!("{}", String::from_utf8(bytes).unwrap());

        unsafe { (*set_out).invoke(wrap_method_arguments(vec![old_out])?, None)? };
        unsafe { (*set_err).invoke(wrap_method_arguments(vec![old_err])?, None)? };
    }

    Ok(())
}

fn reset_memory_stream(
    memory_stream_instance: VARIANT,
    set_position: *mut _MethodInfo,
    set_length: *mut _MethodInfo,
) -> Result<(), String> {
    unsafe {
        (*set_position).invoke(
            wrap_method_arguments(vec![wrap_i64_in_variant(0)])?,
            Some(memory_stream_instance.clone()),
        )?;
        (*set_length).invoke(
            wrap_method_arguments(vec![wrap_i64_in_variant(0)])?,
            Some(memory_stream_instance.clone()),
        )?;
    }

    Ok(())
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
