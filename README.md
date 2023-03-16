# ClrOxide
`ClrOxide` is a rust library that allows you to host the CLR and dynamically execute dotnet binaries.

I wanted to call it `Kepler` for no particular reason, but there's already a package named `kepler` in cargo. :(

I have been working on hosting CLR with rust on and off for 2 years now, and finally something clicked two weeks ago!

This library wouldn't be possible without the following projects:

- [NimPlant](https://github.com/chvancooten/NimPlant) and its [execute assembly](https://github.com/chvancooten/NimPlant/tree/main/client/commands/risky/executeAssembly.nim) implementation
  - The elegance with which `winim/clr` allows overwriting the output buffer for `Console.Write` and gets the output! Striving for the same elegance is the only reason this library took two years. 
How can I convince Cas to dabble with rust if he can't replicate this!? My work for a rust implant for `NimPlant` is also how I got into this rabbit hole in the first place.
- [go-clr](https://github.com/ropnop/go-clr) by [ropnop](https://github.com/ropnop)
  - A very special thank you to ropnop here! This whole library is the result of 3 days of work thanks to something in `go-clr` that just made everything click for me!
- [dinvoke_rs](https://github.com/Kudaes/DInvoke_rs) by [Kudaes](https://github.com/Kudaes)
  - Similar to `go-clr`, Kurosh's `dinvoke_rs` project also made some rust/win32 intricacies clearer and allowed the project to move forward.
- Various CLR-related rust libraries
  - https://github.com/ZerothLaw/mscorlib-rs-sys
  - https://github.com/ZerothLaw/mscoree-rs
  - and likely a few more...


## Usage

`ClrOxide` will load the CLR in the current process, resolve `mscorlib` and redirect the output for `System.Console`, finally loading and running your executable and returning its output as a string.  

Streaming the output is not currently supported, although I'm sure the CLR wrangling magic used for redirecting the output could be a good guide for anyone willing to implement it.

```rust
use clroxide::clr::Clr;
use std::{env, fs, process::exit};

fn main() -> Result<(), String> {
    let (path, args) = prepare_args();

    let contents = fs::read(path).expect("Unable to read file");
    let mut context = Clr::new(contents, args)?;

    let results = context.run()?;

    println!("[*] Results:\n\n{}", results);

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
```
