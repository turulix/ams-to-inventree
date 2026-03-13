use std::fs;
use std::path::Path;

fn main() {
    let lib_rs_path = Path::new("src/lib.rs");
    let apis_dir = Path::new("src/apis");
    let models_dir = Path::new("src/models");

    // Tell Cargo to re-run this script if anything in the apis or models directories changes
    println!("cargo:rerun-if-changed=src/apis");
    println!("cargo:rerun-if-changed=src/models");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // ---------------------------------------------------------
    // Task 1: Patch `bon::Builder` collisions in all API files
    // ---------------------------------------------------------
    if apis_dir.exists() && apis_dir.is_dir() {
        let entries = fs::read_dir(apis_dir).expect("Failed to read apis directory");

        for entry in entries.flatten() {
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(&file_path)
                    .unwrap_or_else(|_| panic!("Failed to read {:?}", file_path));

                if content.contains("derive(::bon::Builder))]")
                    && !content.contains("finish_fn = build_struct")
                {
                    let patched_content = content.replace(
                        "derive(::bon::Builder))]",
                        "derive(::bon::Builder), builder(finish_fn = build_struct))]",
                    );

                    fs::write(&file_path, patched_content).unwrap_or_else(|_| {
                        panic!("Failed to write patched file: {:?}", file_path)
                    });
                }
            }
        }
    }

    // ---------------------------------------------------------
    // Task 2: Delete null_enum.rs and remove it from mod.rs
    // ---------------------------------------------------------
    if models_dir.exists() && models_dir.is_dir() {
        let null_enum_path = models_dir.join("null_enum.rs");
        let mod_rs_path = models_dir.join("mod.rs");

        // Delete the null_enum.rs file if it exists
        if null_enum_path.exists() {
            let _ = fs::remove_file(&null_enum_path); // We ignore the result, if it fails it's likely already gone
        }

        // Clean up the exports in mod.rs
        if mod_rs_path.exists() {
            let content = fs::read_to_string(&mod_rs_path).expect("Failed to read mod.rs");

            // Only process if it actually contains the bad reference
            if content.contains("null_enum") {
                let mut new_content = String::new();

                // Keep all lines that DO NOT contain "null_enum"
                for line in content.lines() {
                    if !line.contains("null_enum") {
                        new_content.push_str(line);
                        new_content.push('\n');
                    }
                }

                fs::write(&mod_rs_path, new_content).expect("Failed to write cleaned mod.rs");
            }
        }
    }

    // ---------------------------------------------------------
    // Task 3: Patch `#![allow(non_snake_case)]` into lib.rs
    // ---------------------------------------------------------
    if lib_rs_path.exists() && lib_rs_path.is_file() {
        let content = fs::read_to_string(lib_rs_path).expect("Failed to read lib.rs");

        // Only modify if the attribute isn't already there
        if !content.contains("#![allow(non_snake_case)]") {
            // Prepend the attribute to the very top of the file
            let patched_content = format!("#![allow(non_snake_case)]\n{}", content);

            fs::write(lib_rs_path, patched_content).expect("Failed to write patched lib.rs");
        }
    }
}
