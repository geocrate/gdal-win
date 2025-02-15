use gdal_win::version::VersionInfo;

fn main() {
    let report = VersionInfo::version_report();
    println!("{report}");
}
