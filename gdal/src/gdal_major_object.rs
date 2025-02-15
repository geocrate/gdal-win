use gdal_bind::GDALMajorObjectH;

/// Common trait for GDAL data types backed by [`GDALMajorObjectH`].
pub trait MajorObject {
    fn gdal_object_ptr(&self) -> GDALMajorObjectH;
}
