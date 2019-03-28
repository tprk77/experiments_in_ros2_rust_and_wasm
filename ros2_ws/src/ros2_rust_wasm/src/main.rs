// Copyright (c) 2019 Tim Perkins

use std::ffi::c_void;
use std::fs::File;
use std::io::{self, Read, Write};
use getopts::Options;
use wasmer_emscripten;
use wasmer_runtime;

mod ros;

fn print_usage_and_exit(program: &str, opts: &Options) -> ! {
    let _ = writeln!(io::stderr(), "{}", opts.usage(&opts.short_usage(&program)));
    std::process::exit(1);
}

fn main() {
    // Argument parsing for the WASM file
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("w", "wasm", "Execute WASM", "PATH");
    opts.optflag("h", "help", "Print this help menu");
    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(error) => {
            let _ = writeln!(io::stderr(), "Error: {}", error);
            print_usage_and_exit(&program, &opts);
        }
    };
    if opt_matches.free.len() > 0 || opt_matches.opt_present("h") {
        print_usage_and_exit(&program, &opts);
    }
    let wasm_path = match opt_matches.opt_str("w") {
        Some(p) => p,
        None => {
            let _ = writeln!(io::stderr(), "Error: You must provide a WASM file");
            print_usage_and_exit(&program, &opts);
        }
    };
    // Load the file...
    let mut wasm_file = match File::open(wasm_path) {
        Ok(f) => f,
        Err(error) => {
            let _ = writeln!(io::stderr(), "Error: {}", error);
            std::process::exit(1);
        }
    };
    let mut wasm_contents = Vec::new();
    wasm_file.read_to_end(&mut wasm_contents).unwrap();
    // Try compiling the code into a module
    let module = wasmer_runtime::compile(&wasm_contents).unwrap();
    // Setup the imports for the module
    let ros_imports = ros::get_imports();
    let em_import_object = if wasmer_emscripten::is_emscripten_module(&module) {
        println!("[INFO] This is an Emscripten module!");
        let mut emscripten_globals = wasmer_emscripten::EmscriptenGlobals::new(&module);
        wasmer_emscripten::generate_emscripten_env(&mut emscripten_globals)
    } else {
        println!("[INFO] This is NOT an Emscripten module!");
        wasmer_runtime::ImportObject::new()
    };
    // Get a merged import object
    let mut merged_import_object = wasmer_runtime::ImportObject::new();
    merged_import_object.extend(ros_imports);
    merged_import_object.extend(em_import_object);
    // Instantiate the module so that it can be run
    let mut instance = module.instantiate(&merged_import_object).unwrap();
    // We need to inject the data we want to use into the VM context. This code is adapted form
    // `run_emscripten_instance` which adds some Emscripten data to the context. (Note that we can't
    // use that function if want to use custom data, because it will overwrite what's there.)
    let mut ros_data = ros::RosData::new(&mut instance as *mut wasmer_runtime::Instance);
    let ros_data_ptr = &mut ros_data as *mut _ as *mut c_void;
    instance.context_mut().data = ros_data_ptr;
    // Call our exported function!
    instance.call("ros_main", &[]).unwrap();
}
