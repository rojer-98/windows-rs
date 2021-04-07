use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let tokens = windows_macros::generate!(
        Windows::Foundation::{IReference, IStringable, PropertyValue},
        Windows::Win32::Automation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
        Windows::Win32::WinRT::{IRestrictedErrorInfo, ILanguageExceptionErrorInfo2},
        Windows::Win32::Debug::{GetLastError, FormatMessageW},
        Windows::Win32::WindowsProgramming::CloseHandle,
        Windows::Win32::Com::{
            CoCreateGuid, CoTaskMemAlloc, CoTaskMemFree, CLSIDFromProgID, CoInitializeEx, CoCreateInstance,
        },
        Windows::Win32::SystemServices::{
            CreateEventW, SetEvent, WaitForSingleObject, GetProcessHeap, HeapAlloc, HeapFree, GetProcAddress,
            LoadLibraryA, FreeLibrary,
        },
    );

    let mut path = windows_gen::workspace_dir();
    path.push("src");
    path.push("bindings.rs");

    let mut file = std::fs::File::create(&path)?;
    file.write_all(
        "// This file was generated by the `windows` crate - do not edit by hand!\n\n".as_bytes(),
    )?;

    // TODO: Rust doesn't seem to support something like a C++ static_assert so that we can assert
    // that the generated file matches the crate it is being built with.

    // file.write_all(
    //     format!(
    //         "assert!(env!(\"CARGO_PKG_VERSION\") == \"{}\");\n\n",
    //         env!("CARGO_PKG_VERSION")
    //     )
    //     .as_bytes(),
    // )?;

    file.write_all(tokens.as_bytes())?;
    drop(file);

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}