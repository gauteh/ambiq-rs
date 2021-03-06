extern crate bindgen;

use cc;
use fs_extra::dir;
use glob;
use std::env;
use std::path::PathBuf;
use std::process::Command;

enum Board {
    SfRedboard,
    SfRedboardNano,
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build the Board Support Crate for the desired chip.
    println!("Building the Ambiq SDK and Sparkfun BSP");

    let board = if env::var_os("CARGO_FEATURE_SPARKFUN_REDBOARD").is_some() {
        Board::SfRedboard
    } else if env::var_os("CARGO_FEATURE_SPARKFUN_REDBOARD_NANO").is_some() {
        Board::SfRedboardNano
    } else {
        panic!("No board selected.");
    };

    let board_dir = match board {
        Board::SfRedboard => "boards_sfe/redboard_artemis/bsp",
        Board::SfRedboardNano => "boards_sfe/redboard_artemis_nano/bsp",
    };

    let sdk_dir = out_path.join("ambiq-sparkfun-sdk");
    dir::copy(
        "ambiq-sparkfun-sdk",
        &out_path,
        &dir::CopyOptions {
            skip_exist: true,
            ..Default::default()
        },
    )
    .unwrap();

    let board_dir = sdk_dir.join(board_dir);

    Command::new("make")
        .current_dir(board_dir.join("gcc"))
        .status()
        .expect("could not re-build the BSP library");

    Command::new("make")
        .current_dir(sdk_dir.join("mcu/apollo3/hal/gcc"))
        .status()
        .expect("could not re-build the HAL library");

    // The BSP library appears to be statically linked to the am_hal library containing the
    // apollo3 MCU functions (modulo the current chip + MCU).
    println!("cargo:rustc-link-lib=static=am_bsp");
    println!("cargo:rustc-link-lib=static=am_hal");
    println!(
        "cargo:rustc-link-search=native={}",
        board_dir.join("gcc/bin").to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        sdk_dir.join("mcu/apollo3/hal/gcc/bin").to_str().unwrap()
    );
    println!("cargo:lib=am_bsp");
    println!("cargo:lib=am_hal");

    // Entry-point: TODO: Make feature gated! then you don't need cortex_m::entry, but need a much
    // more complicated linker script for interrupt functions etc.
    //
    // cc::Build::new()
    //     .file("ambiq-sparkfun-sdk/boards_sfe/common/tools_sfe/templates/startup_gcc.c")
    //     .compile("startup_gcc");

    // Utils
    let am_utils = if env::var_os("CARGO_FEATURE_UTILS").is_some() {
        let mut compiler = cc::Build::new();
        compiler.warnings(false); // not my problem.
        compiler.include("ambiq-sparkfun-sdk/mcu/apollo3");
        compiler.include("ambiq-sparkfun-sdk/CMSIS/AmbiqMicro/Include");
        compiler.include("ambiq-sparkfun-sdk/CMSIS/ARM/Include");
        compiler.include("ambiq-sparkfun-sdk/devices");

        for path in glob::glob("ambiq-sparkfun-sdk/utils/*.c").unwrap() {
            let path = path.unwrap();
            let spath = path.file_name().unwrap().to_str().unwrap();
            if !(spath.ends_with("regdump.c") || spath.ends_with("faultisr.c"))
            {
                compiler.file(path);
            }
        }
        compiler.compile("am_utils");
        true
    } else {
        false
    };

    // Devices
    let mut compiler = cc::Build::new();
    compiler.include("ambiq-sparkfun-sdk/mcu/apollo3");
    compiler.include("ambiq-sparkfun-sdk/CMSIS/AmbiqMicro/Include");
    compiler.include("ambiq-sparkfun-sdk/CMSIS/ARM/Include");
    compiler.include("ambiq-sparkfun-sdk/devices");
    compiler.include(&board_dir);

    let paths = &["am_devices_button.c", "am_devices_led.c"];

    for path in paths {
        let path = PathBuf::from("ambiq-sparkfun-sdk/devices/").join(&path);
        compiler.file(path);
    }
    compiler.compile("am_devices");

    println!("cargo:rerun-if-changed=wrapper.h");
    // println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("c_types")
        .clang_arg(&format!("-I{}", board_dir.to_str().unwrap()))
        .clang_arg(&format!("-DAM_UTIL_ENABLE={}", if am_utils { "1" } else { "0" }))
        .clang_arg("-Iambiq-sparkfun-sdk/mcu/apollo3")
        .clang_arg("-Iambiq-sparkfun-sdk/CMSIS/AmbiqMicro/Include")
        .clang_arg("-Iambiq-sparkfun-sdk/CMSIS/ARM/Include")
        .clang_arg("-Iambiq-sparkfun-sdk/devices")
        .clang_arg("-Iambiq-sparkfun-sdk/utils")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
