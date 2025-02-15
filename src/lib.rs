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
mod cpl;

#[path = "../gdal/src/dataset.rs"]
mod dataset;

pub use dataset::Dataset;

#[path = "../gdal/src/driver.rs"]
mod driver;

pub use driver::{Driver, DriverManager, DriverType};

#[path = "../gdal/src/gdal_major_object.rs"]
mod gdal_major_object;

#[path = "../gdal/src/errors.rs"]
mod errors;

#[path = "../gdal/src/geo_transform.rs"]
mod geo_transform;

pub use geo_transform::{GeoTransform, GeoTransformEx};

#[path = "../gdal/src/metadata.rs"]
mod metadata;

pub use metadata::{Metadata, MetadataEntry};

#[path = "../gdal/src/options.rs"]
mod options;

pub use options::{DatasetOptions, GdalOpenFlags};

#[path = "../gdal/src/test_utils.rs"]
mod test_utils;

#[path = "../gdal/src/utils.rs"]
mod utils;

#[path = "../gdal/src/version.rs"]
mod version;

#[path = "../gdal/src/vsi.rs"]
mod vsi;

#[cfg(test)]
fn assert_almost_eq(a: f64, b: f64) {
    let f: f64 = a / b;
    assert!(f < 1.00001);
    assert!(f > 0.99999);
}
