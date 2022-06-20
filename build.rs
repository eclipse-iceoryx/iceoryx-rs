// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: © Contributors to the iceoryx-rs project
// SPDX-FileContributor: Mathias Kraus
// SPDX-FileContributor: Apex.AI

use std::env;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

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

    const ICEORYX_VERSION: &str = "v2.0.2";
    const ICEORYX_GIT_BRANCH: &str = ICEORYX_VERSION;
    clone_repo(
        "https://github.com/eclipse-iceoryx/iceoryx.git",
        ICEORYX_GIT_BRANCH,
        &iceoryx_source_dir,
    )?;

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
