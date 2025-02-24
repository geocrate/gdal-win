use std::num::NonZeroUsize;

use crate::cpl::CslStringList;
use crate::errors;
use crate::raster::processing::dem::options::common_dem_options;

use super::options::CommonOptions;

/// Configuration options for [`roughness()`][super::roughness()].

#[derive(Debug, Clone, Default)]
pub struct RoughnessOptions {
    common_options: CommonOptions,
}

impl RoughnessOptions {
    /// Create a DEM-roughness options set.
    pub fn new() -> Self {
        Default::default()
    }

    common_dem_options!();

    /// Render relevant options into [`CslStringList`] values, as compatible with
    /// [`crate::gdal_sys::GDALDEMProcessing`].
    pub fn to_options_list(&self) -> errors::Result<CslStringList> {
        let mut opts = CslStringList::new();
        self.store_common_options_to(&mut opts)?;
        Ok(opts)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::cpl::CslStringList;
    use crate::errors::Result;
    use crate::raster::processing::dem::roughness;
    use crate::raster::processing::dem::roughness::RoughnessOptions;
    use crate::raster::StatisticsAll;
    use crate::test_utils::{fixture, InMemoryFixture};
    use crate::Dataset;

    #[test]
    fn test_options() -> Result<()> {
        let mut proc = RoughnessOptions::new();
        proc.with_input_band(2.try_into().unwrap())
            .with_compute_edges(true)
            .with_output_format("GTiff")
            .with_additional_options("CPL_DEBUG=ON".parse()?);

        let expected: CslStringList = "-compute_edges -b 2 -of GTiff CPL_DEBUG=ON".parse()?;
        assert_eq!(expected.to_string(), proc.to_options_list()?.to_string());

        Ok(())
    }

    #[test]
    fn test_roughness() -> Result<()> {
        let opts = RoughnessOptions::new();

        let ds = Dataset::open(fixture("dem-hills.tiff"))?;

        let output = InMemoryFixture::new("dem-hills-roughness.tiff");
        let roughness = roughness(&ds, output.path(), &opts)?;

        let stats = roughness
            .rasterband(1)?
            .get_statistics(true, false)?
            .unwrap();

        // These numbers were generated by extracting the output from:
        //    gdaldem roughness fixtures/dem-hills.tiff target/dest.tiff
        //    gdalinfo -stats target/dest.tiff
        let expected = StatisticsAll {
            min: 0.0,
            max: 14.36100769043,
            mean: 1.5128357817365,
            std_dev: 2.0120679959608,
        };

        assert_near!(StatisticsAll, stats, expected, epsilon = 1e-10);
        Ok(())
    }
}
