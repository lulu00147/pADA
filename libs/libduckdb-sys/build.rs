use std::path::{Path, PathBuf};
use std::{env, fs};
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("bindgen.rs");
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bindings_src = PathBuf::from(&cargo_manifest_dir).join("src/bindgen_bundled_version.rs");
    fs::copy(&bindings_src, &out_path)
        .unwrap_or_else(|e| panic!("Failed to copy bindings from {}: {e}", bindings_src.display()));
    let target = env::var("TARGET").unwrap_or_default();
    let is_musl = target.ends_with("-unknown-linux-musl");
    let library_file = if target.contains("windows") {
        "duckdb_static.lib"
    } else {
        "libduckdb_static.a"
    };
    let explicit_lib_dir = env::var("DUCKDB_LIB_DIR").ok();
    let static_link = env::var("DUCKDB_STATIC")
        .map(|value| value == "1")
        .unwrap_or(explicit_lib_dir.is_none());
    let lib_dir = if let Some(dir) = explicit_lib_dir {
        let dir = PathBuf::from(dir.trim());
        if static_link && !dir.join(library_file).is_file() {
            panic!(
                "DUCKDB_LIB_DIR={} does not contain {} for target {}",
                dir.display(),
                library_file,
                target
            );
        }
        Some(dir)
    } else {
        let workspace_root = Path::new(&cargo_manifest_dir)
            .parent()
            .and_then(|path| path.parent());
        workspace_root.and_then(|root| {
            let cache_root = root.join("duckdb-cache");
            let target_cache = cache_root.join(&target);
            let candidates = if is_musl {
                vec![target_cache, cache_root]
            } else {
                vec![cache_root]
            };
            candidates.into_iter().find(|dir| dir.join(library_file).is_file())
        })
    };
    if let Some(dir) = lib_dir {
        println!("cargo:rustc-link-search=native={}", dir.display());
        if static_link {
            println!("cargo:rustc-link-lib=static=duckdb_static");
            if target.ends_with("-unknown-linux-musl") || target.contains("-linux-gnu") {
                for library in [
                    "duckdb_generated_extension_loader",
                    "json_extension",
                    "parquet_extension",
                    "core_functions_extension",
                ] {
                    if !dir.join(format!("lib{library}.a")).is_file() {
                        panic!(
                            "DUCKDB_LIB_DIR={} does not contain lib{library}.a for target {}",
                            dir.display(),
                            target
                        );
                    }
                    println!("cargo:rustc-link-lib=static={library}");
                }
            }
        } else {
            println!("cargo:rustc-link-lib=duckdb");
        }
        if target.contains("windows") {
            println!("cargo:rustc-link-lib=rstrtmgr");
            println!("cargo:rustc-link-lib=dylib=ws2_32");
        } else if target.ends_with("-unknown-linux-musl") {
            println!("cargo:rustc-link-lib=dl");
            println!("cargo:rustc-link-lib=pthread");
            println!("cargo:rustc-link-lib=m");
            let cxx_lib = env::var("DUCKDB_CXX_LIB").unwrap_or_else(|_| "c++".into());
            println!("cargo:rustc-link-lib={cxx_lib}");
            if cxx_lib == "stdc++" {
                println!("cargo:rustc-link-lib=unwind");
            }
        } else if target.contains("-linux-gnu") {
            let cxx_lib = env::var("DUCKDB_CXX_LIB").unwrap_or_else(|_| "c++".into());
            println!("cargo:rustc-link-lib={cxx_lib}");
        }
        if let Ok(inc) = env::var("DUCKDB_INCLUDE_DIR") {
            println!("cargo:include={}", inc.trim());
        }
    } else {
        println!(
            "cargo:warning=No DuckDB library found for target {}. Expected {} in DUCKDB_LIB_DIR or duckdb-cache/{}/",
            target,
            library_file,
            target
        );
    }
    println!("cargo:rerun-if-env-changed=DUCKDB_LIB_DIR");
    println!("cargo:rerun-if-env-changed=DUCKDB_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=DUCKDB_STATIC");
    println!("cargo:rerun-if-env-changed=DUCKDB_CXX_LIB");
}
