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

pub mod build;

#[cfg(test)]
fn assert_almost_eq(a: f64, b: f64) {
    let f: f64 = a / b;
    assert!(f < 1.00001);
    assert!(f > 0.99999);
}
