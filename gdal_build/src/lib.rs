use std::{env, fs, path::PathBuf};
use reqwest::blocking::get;
use std::io::Write;
use sevenz_rust::decompress_file;

pub fn gdal_build() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target_dir = PathBuf::from(&out_dir).join("target");
    let binary_dir = PathBuf::from(&target_dir).join(profile);

    let gdal_lib_url = "https://github.com/geocrate/files/releases/download/gdal-3100/gdal-3100-lib.7z";
    let gdal_bin_url = "https://github.com/geocrate/files/releases/download/gdal-3100/gdal-3100-bin.7z";
    
    let lib_7z_file = target_dir.join("gdal-3100-lib.7z");
    let bin_7z_file = target_dir.join("gdal-3100-bin.7z");
    
    let gdal_lib_dir = target_dir.join("gdal-3100");

    // Ensure extract_path exists
    fs::create_dir_all(&gdal_lib_dir).unwrap();

    // Download the file
    if !lib_7z_file.exists() || !bin_7z_file.exists() {
        let response = get(gdal_lib_url).unwrap();
        let mut file = fs::File::create(&lib_7z_file).unwrap();
        let content = response.bytes().unwrap();
        file.write_all(&content).unwrap();

        let response = get(gdal_bin_url).unwrap();
        let mut file = fs::File::create(&bin_7z_file).unwrap();
        let content = response.bytes().unwrap();
        file.write_all(&content).unwrap();

        decompress_file(&lib_7z_file, &gdal_lib_dir).unwrap();
        decompress_file(&bin_7z_file, &binary_dir).unwrap();
    }

    println!("cargo:rustc-link-lib=static=gdal_i");
    println!("cargo:rustc-link-search={}", gdal_lib_dir.to_str().unwrap());
    
    println!("cargo:rerun-if-changed=build.rs");
}
