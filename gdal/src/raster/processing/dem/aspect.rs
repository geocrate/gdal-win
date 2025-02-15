use std::num::NonZeroUsize;

use super::options::{common_dem_options, CommonOptions};
use crate::cpl::CslStringList;
use crate::errors;
use crate::raster::processing::dem::DemSlopeAlg;

/// Configuration options for [`aspect()`][super::aspect()].
#[derive(Debug, Clone, Default)]
pub struct AspectOptions {
    common_options: CommonOptions,
    algorithm: Option<DemSlopeAlg>,
    zero_for_flat: Option<bool>,
    trigonometric: Option<bool>,
}

impl AspectOptions {
    /// Create a DEM-aspect options set.
    pub fn new() -> Self {
        Default::default()
    }

    common_dem_options!();

    /// Specify the slope computation algorithm.
    pub fn with_algorithm(&mut self, algorithm: DemSlopeAlg) -> &mut Self {
        self.algorithm = Some(algorithm);
        self
    }

    /// Return `0` for flat areas with `slope=0`, instead of `-9999`.
    ///
    /// See: [`zero_for_flat`](https://gdal.org/programs/gdaldem.html#cmdoption-zero_for_flat)
    pub fn with_zero_for_flat(&mut self, state: bool) -> &mut Self {
        self.zero_for_flat = Some(state);
        self
    }

    /// Return trigonometric angle instead of azimuth. Thus 0° means East, 90° North, 180° West, 270° South.
    pub fn with_trigonometric_angles(&mut self, state: bool) -> &mut Self {
        self.trigonometric = Some(state);
        self
    }

    /// Render relevant common options into [`CslStringList`] values, as compatible with
    /// [`gdal_bind::GDALDEMProcessing`].
    pub fn to_options_list(&self) -> errors::Result<CslStringList> {
        let mut opts = CslStringList::default();

        self.store_common_options_to(&mut opts)?;

        if let Some(alg) = self.algorithm {
            opts.add_string("-alg")?;
            opts.add_string(alg.to_gdal_option())?;
        }

        if self.zero_for_flat == Some(true) {
            opts.add_string("-zero_for_flat")?;
        }

        if self.trigonometric == Some(true) {
            opts.add_string("-trigonometric")?;
        }

        Ok(opts)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::cpl::CslStringList;
    use crate::errors::Result;
    use crate::raster::processing::dem::aspect;
    use crate::raster::StatisticsAll;
    use crate::test_utils::{fixture, InMemoryFixture};
    use crate::Dataset;

    use super::*;

    #[test]
    fn test_options() -> Result<()> {
        let mut proc = AspectOptions::new();
        proc.with_input_band(2.try_into().unwrap())
            .with_algorithm(DemSlopeAlg::ZevenbergenThorne)
            .with_compute_edges(true)
            .with_zero_for_flat(true)
            .with_trigonometric_angles(true)
            .with_output_format("GTiff")
            .with_additional_options("CPL_DEBUG=ON".parse()?);

        let expected: CslStringList =
            "-compute_edges -b 2 -of GTiff CPL_DEBUG=ON -alg ZevenbergenThorne -zero_for_flat -trigonometric"
                .parse()?;
        assert_eq!(expected.to_string(), proc.to_options_list()?.to_string());

        Ok(())
    }

    #[test]
    fn test_aspect() -> Result<()> {
        let mut opts = AspectOptions::new();
        opts.with_algorithm(DemSlopeAlg::Horn)
            .with_zero_for_flat(true);

        let ds = Dataset::open(fixture("dem-hills.tiff"))?;

        let output = InMemoryFixture::new("dem-hills-aspect.tiff");
        let aspect = aspect(&ds, output.path(), &opts)?;

        let stats = aspect.rasterband(1)?.get_statistics(true, false)?.unwrap();

        // These numbers were generated by extracting the output from:
        //    gdaldem aspect -alg Horn -zero_for_flat fixtures/dem-hills.tiff target/dest.tiff
        //    gdalinfo -stats target/dest.tiff
        let expected = StatisticsAll {
            min: 0.0,
            max: 359.9951171875,
            mean: 165.72752499998,
            std_dev: 98.590199951445,
        };
        assert_near!(StatisticsAll, stats, expected, epsilon = 1e-10);
        Ok(())
    }
}
