// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus

use cpp_build;

use std::env;
use std::path::Path;
use std::process::Command;

fn make_and_install(source_dir: &str, build_dir: &str, install_dir: &str) -> std::io::Result<()> {
    let cmake_install_prefix = format!("-DCMAKE_INSTALL_PREFIX={}", install_dir);

    for iceoryx_component in &["iceoryx_utils", "iceoryx_posh"] {
        let component_source_dir = format!("{}/{}", source_dir, iceoryx_component);
        let component_build_dir = format!("{}/{}", build_dir, iceoryx_component);

        Command::new("mkdir")
            .args(&["-p", &component_build_dir])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;

        Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&[
                "-DCMAKE_BUILD_TYPE=Release",
                "-DBUILD_SHARED_LIBS=OFF",
                &cmake_install_prefix,
                &component_source_dir,
            ])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;

        Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&["--build", ".", "--target", "install"])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;
    }

    Ok(())
}

fn clone_repo(repo: &str, branch: &str, source_dir: &str) -> std::io::Result<()> {
    if !Path::new(source_dir).join(".git").exists() {
        Command::new("git")
            .args(&[
                "clone",
                repo,
                &format!("--branch={}", branch),
                "--recursive",
                source_dir,
            ])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;
    } else {
        Command::new("git")
            .current_dir(source_dir)
            .args(&["checkout", branch])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Valid dir");

    let iceoryx_source_dir = format!("{}/{}/{}", current_dir, "target", "iceoryx-git");
    let iceoryx_build_dir = format!("{}/{}/{}", current_dir, "target", "iceoryx-build");
    let iceoryx_install_dir = format!("{}/{}/{}", current_dir, "target", "iceoryx-install");

    clone_repo(
        "https://github.com/eclipse-iceoryx/iceoryx.git",
        "v1.0.1",
        &iceoryx_source_dir,
    )?;

    make_and_install(
        &iceoryx_source_dir,
        &iceoryx_build_dir,
        &iceoryx_install_dir,
    )?;

    let iceoryx_include_dir = format!("{}/{}", iceoryx_install_dir, "include");
    let iceoryx_lib_dir = format!("{}/{}", iceoryx_install_dir, "lib");
    cpp_build::Config::new()
        .include(iceoryx_include_dir)
        .flag("-Wno-noexcept-type")
        .flag("-std=c++14")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search={}", iceoryx_lib_dir);

    println!("cargo:rustc-link-lib=iceoryx_posh_roudi");
    println!("cargo:rustc-link-lib=iceoryx_posh");
    println!("cargo:rustc-link-lib=iceoryx_utils");
    println!("cargo:rustc-link-lib=iceoryx_platform");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    Ok(())
}
