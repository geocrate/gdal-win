#![allow(warnings)]

#[path = "../gdal/src/programs/mod.rs"]
pub mod programs;

#[path = "../gdal/src/raster/mod.rs"]
pub mod raster;

#[path = "../gdal/src/spatial_ref/mod.rs"]
pub mod spatial_ref;

#[path = "../gdal/src/vector/mod.rs"]
pub mod vector;

#[path = "../gdal/src/cpl.rs"]
pub mod cpl;

#[path = "../gdal/src/dataset.rs"]
pub mod dataset;

pub use dataset::Dataset;

#[path = "../gdal/src/driver.rs"]
pub mod driver;

pub use driver::{Driver, DriverManager, DriverType};

#[path = "../gdal/src/gdal_major_object.rs"]
pub mod gdal_major_object;

#[path = "../gdal/src/errors.rs"]
pub mod errors;

#[path = "../gdal/src/geo_transform.rs"]
pub mod geo_transform;

pub use geo_transform::{GeoTransform, GeoTransformEx};

#[path = "../gdal/src/metadata.rs"]
pub mod metadata;

pub use metadata::{Metadata, MetadataEntry};

#[path = "../gdal/src/options.rs"]
pub mod options;

pub use options::{DatasetOptions, GdalOpenFlags};

#[path = "../gdal/src/test_utils.rs"]
mod test_utils;

#[path = "../gdal/src/utils.rs"]
pub mod utils;

#[path = "../gdal/src/version.rs"]
pub mod version;

#[path = "../gdal/src/vsi.rs"]
pub mod vsi;

#[cfg(test)]
fn assert_almost_eq(a: f64, b: f64) {
    let f: f64 = a / b;
    assert!(f < 1.00001);
    assert!(f > 0.99999);
}

// ---------------------------------    gdal_win::build()    ---------------------------------

use std::{env, fs, path::PathBuf};
use reqwest::blocking::get;
use std::io::Write;
use sevenz_rust::decompress_file;

pub fn build() {
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
