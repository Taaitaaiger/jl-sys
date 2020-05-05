#![allow(unused_imports)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
fn find_julia() -> Option<String> {
    if let Ok(path) = env::var("JULIA_DIR") {
        return Some(path);
    }

    if Path::new("/usr/include/julia/julia.h").exists() {
        return Some("/usr".to_string());
    }

    None
}

#[cfg(target_os = "windows")]
fn flags() -> Vec<String> {
    let julia_dir = env::var("JULIA_DIR").expect("Julia cannot be found. You can specify the Julia installation path with the JULIA_DIR environment variable.");
    let cygwin_path = env::var("CYGWIN_DIR").expect("Cygwin cannot be found. You can specify the Cygwin installation path with the CYGWIN_DIR environment variable.");

    let jl_include_path = format!("-I{}/include/julia/", julia_dir);
    let cygwin_include_path = format!("-I{}/usr/include", cygwin_path);
    let w32api_include_path = format!("-I{}/usr/include/w32api", cygwin_path);
    let jl_lib_path = format!("-L{}/bin/", julia_dir);

    println!("cargo:rustc-flags={}", &jl_lib_path);
    println!("cargo:rustc-link-lib=julia");
    vec![
        jl_include_path,
        cygwin_include_path,
        w32api_include_path,
        jl_lib_path,
    ]
}

#[cfg(target_os = "linux")]
fn flags() -> Vec<String> {
    let flags = match find_julia() {
        Some(julia_dir) => {
            let jl_include_path = format!("-I{}/include/julia/", julia_dir);
            let jl_lib_path = format!("-L{}/lib/", julia_dir);

            println!("cargo:rustc-flags={}", &jl_lib_path);
            vec![jl_include_path, jl_lib_path]
        }
        None => Vec::new(),
    };

    println!("cargo:rustc-link-lib=julia");
    flags
}

fn main() {
    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path.push("bindings.rs");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=JULIA_DIR");
    println!("cargo:rerun-if-env-changed=CYGWIN_DIR");

    if env::var("CARGO_FEATURE_DOCS_RS").is_ok() {
        fs::copy("dummy-bindings.rs", &out_path)
            .expect("Couldn't create bindings from dummy bindings.");
        return;
    }

    let flags = flags();

    // Only generate bindings if it's used by Jlrs
    let bindings = bindgen::Builder::default()
        .clang_args(&flags)
        .header("wrapper.h")
        .size_t_is_usize(true)
        // Initializing and stopping
        .whitelist_function("jl_init__threading")
        .whitelist_function("jl_is_initialized")
        .whitelist_function("jl_atexit_hook")
        // Gc
        .whitelist_function("jl_pgcstack")
        .whitelist_function("jl_get_ptls_states")
        .whitelist_function("jl_gc_queue_root")
        // boxing and unboxing primitives
        .whitelist_function("jl_box_bool")
        .whitelist_function("jl_box_char")
        .whitelist_function("jl_box_int8")
        .whitelist_function("jl_unbox_int8")
        .whitelist_function("jl_box_int16")
        .whitelist_function("jl_unbox_int16")
        .whitelist_function("jl_box_int32")
        .whitelist_function("jl_unbox_int32")
        .whitelist_function("jl_box_int64")
        .whitelist_function("jl_unbox_int64")
        .whitelist_function("jl_box_uint8")
        .whitelist_function("jl_unbox_uint8")
        .whitelist_function("jl_box_uint16")
        .whitelist_function("jl_unbox_uint16")
        .whitelist_function("jl_box_uint32")
        .whitelist_function("jl_unbox_uint32")
        .whitelist_function("jl_box_uint64")
        .whitelist_function("jl_unbox_uint64")
        .whitelist_function("jl_box_float32")
        .whitelist_function("jl_unbox_float32")
        .whitelist_function("jl_box_float64")
        .whitelist_function("jl_unbox_float64")
        // call functions
        .whitelist_function("jl_call0")
        .whitelist_function("jl_call1")
        .whitelist_function("jl_call2")
        .whitelist_function("jl_call3")
        .whitelist_function("jl_call")
        .whitelist_function("jl_exception_occurred")
        .whitelist_function("jl_eval_string")
        // symbols and globals
        .whitelist_function("jl_symbol_n")
        .whitelist_function("jl_get_global")
        .whitelist_function("jl_set_global")
        // structs
        .whitelist_function("jl_field_index")
        .whitelist_function("jl_get_nth_field")
        .whitelist_function("jl_get_nth_field_noalloc")
        .whitelist_function("jl_get_field")
        .whitelist_function("jl_field_isdefined")
        .whitelist_function("jl_compute_fieldtypes")
        // tuples
        .whitelist_function("jl_apply_type")
        .whitelist_function("jl_new_structv")
        .whitelist_function("jl_tupletype_fill")
        .whitelist_function("jl_apply_tuple_type_v")
        .whitelist_function("jl_new_struct_uninit")
        // n-dimensional arrays
        .whitelist_function("jl_apply_array_type")
        .whitelist_function("jl_array_eltype")
        .whitelist_function("jl_alloc_array_1d")
        .whitelist_function("jl_alloc_array_2d")
        .whitelist_function("jl_alloc_array_3d")
        .whitelist_function("jl_new_array")
        .whitelist_function("jl_ptr_to_array_1d")
        .whitelist_function("jl_ptr_to_array")
        // strings
        .whitelist_function("jl_pchar_to_string")
        .whitelist_function("jl_typeof_str")
        .whitelist_function("jl_typename_str")
        // modules
        .whitelist_var("jl_base_module")
        .whitelist_var("jl_core_module")
        .whitelist_var("jl_main_module")
        // types
        .whitelist_type("jl_value_t")
        .whitelist_type("jl_taggedvalue_t")
        .whitelist_type("jl_datatype_t")
        .whitelist_var("jl_bool_type")
        .whitelist_var("jl_char_type")
        .whitelist_var("jl_int8_type")
        .whitelist_var("jl_int16_type")
        .whitelist_var("jl_int32_type")
        .whitelist_var("jl_int64_type")
        .whitelist_var("jl_uint8_type")
        .whitelist_var("jl_uint16_type")
        .whitelist_var("jl_uint32_type")
        .whitelist_var("jl_uint64_type")
        .whitelist_var("jl_float32_type")
        .whitelist_var("jl_float64_type")
        .whitelist_var("jl_string_type")
        .whitelist_var("jl_datatype_type")
        .whitelist_var("jl_array_typename")
        .whitelist_var("jl_module_type")
        .whitelist_var("jl_nothing")
        .whitelist_var("jl_tuple_typename")
        .whitelist_var("jl_namedtuple_typename")
        .whitelist_var("jl_simplevector_type")
        .whitelist_var("jl_uniontype_type")
        .whitelist_var("jl_tvar_type")
        .whitelist_var("jl_unionall_type")
        .whitelist_var("jl_typename_type")
        .whitelist_var("jl_symbol_type")
        .whitelist_var("jl_task_type")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");
}
