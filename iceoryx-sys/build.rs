// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus
// SPDX-FileContributor: Apex.AI

use std::env;
use std::io::{Error, ErrorKind};
use std::process::Command;

const ICEORYX_VERSION: &str = "v2.0.3";

fn make_and_install(source_dir: &str, build_dir: &str, install_dir: &str) -> std::io::Result<()> {
    let cmake_install_prefix = format!("-DCMAKE_INSTALL_PREFIX={}", install_dir);
    let cmake_prefix_path = format!("-DCMAKE_PREFIX_PATH={}", install_dir);

    for iceoryx_component in &["iceoryx_hoofs", "iceoryx_posh"] {
        let component_source_dir = format!("{}/{}", source_dir, iceoryx_component);
        let component_build_dir = format!("{}/{}", build_dir, iceoryx_component);

        if !Command::new("mkdir")
            .args(&["-p", &component_build_dir])
            .status()?
            .success()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Could not create build dir for '{}'!", iceoryx_component),
            ));
        }

        if !Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&[
                "-DCMAKE_BUILD_TYPE=Release",
                "-DBUILD_SHARED_LIBS=OFF",
                "-DROUDI_ENVIRONMENT=ON",
                &cmake_prefix_path,
                &cmake_install_prefix,
                &component_source_dir,
            ])
            .status()?
            .success()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Could not run cmake for '{}'!", iceoryx_component),
            ));
        }

        if !Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&["--build", ".", "--target", "install"])
            .status()?
            .success()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Could not build '{}'!", iceoryx_component),
            ));
        }
    }

    Ok(())
}

fn extract_archive(archive_dir: &str, source_dir: &str, version: &str) -> std::io::Result<()> {
    if !Command::new("mkdir")
        .args(&["-p", &source_dir])
        .status()?
        .success()
    {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Could not create source dir for '{}'!", source_dir),
        ));
    }

    if !Command::new("tar")
        .args(&[
            "-xf",
            &format!("{}/{}.tar.gz", archive_dir, version),
            "-C",
            &source_dir,
            "--strip-components=1",
        ])
        .status()?
        .success()
    {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Could not extract archive '{}' to '{}'!",
                version, source_dir
            ),
        ));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Target output directory");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory");

    let iceoryx_archive_dir = format!("{}/{}", manifest_dir, "iceoryx-cpp");
    let iceoryx_source_dir = format!("{}/{}/", out_dir, "iceoryx-cpp");
    let iceoryx_build_dir = format!("{}/{}", out_dir, "iceoryx-build");
    let iceoryx_install_dir = format!("{}/{}", out_dir, "iceoryx-install");

    extract_archive(&iceoryx_archive_dir, &iceoryx_source_dir, ICEORYX_VERSION)?;

    make_and_install(
        &iceoryx_source_dir,
        &iceoryx_build_dir,
        &iceoryx_install_dir,
    )?;

    let iceoryx_include_dir = format!(
        "{}/{}/iceoryx/{}",
        iceoryx_install_dir, "include", ICEORYX_VERSION
    );
    let iceoryx_lib_dir = format!("{}/{}", iceoryx_install_dir, "lib");

    #[cfg(not(any(target_os = "windows")))]
    cpp_build::Config::new()
        .include(iceoryx_include_dir)
        .flag("-Wno-noexcept-type")
        .flag("-std=c++17")
        .build("src/lib.rs");

    #[cfg(target_os = "windows")]
    cpp_build::Config::new()
        .include(iceoryx_include_dir)
        .flag("/std:c++17")
        .flag("/MD")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search={}", iceoryx_lib_dir);

    println!("cargo:rustc-link-lib=iceoryx_posh_testing");

    println!("cargo:rustc-link-lib=iceoryx_posh_roudi");
    println!("cargo:rustc-link-lib=iceoryx_posh");
    println!("cargo:rustc-link-lib=iceoryx_hoofs");
    println!("cargo:rustc-link-lib=iceoryx_platform");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=acl");

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(any(target_os = "macos"))]
    println!("cargo:rustc-link-lib=c++");

    Ok(())
}
