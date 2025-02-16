# 🌍 gdal-win

**🗺️ GDAL for Rust / Windows 🎯**

`gdal-win` provides prebuilt GDAL libraries and bindings for Rust development on Windows. It automatically sets up GDAL binaries obtained from GISInternals. It includes necessary DLLs and libraries sourced from [GISInternals](https://www.gisinternals.com) and utilizes the GDAL Rust wrapper from [georust/gdal](https://github.com/georust/gdal).

## ✨ Features

- ⚡ Precompiled GDAL binaries for Windows x64 from [GISInternals](https://www.gisinternals.com).
- 🦀 Rust bindings to GDAL from [georust/gdal](https://github.com/georust/gdal).
- 🛠️ Simplifies GDAL setup for Rust developers on Windows.

## 📦 Setup

Ensure you have Rust installed. You can then add `gdal-win` to your Rust project by including the following in your `Cargo.toml`:

```toml
[package]
name = "gdal-win example"
version = "0.1.0"
edition = "2021"

[dependencies]
gdal-win = "*"

[build-dependencies]
gdal-setup = "*"
```

Ensure your project includes a `build.rs` file to set up GDAL automatically:

```rust
// build.rs
fn main() {
    gdal_setup::setup();
}
```

This will configure the necessary environment for GDAL during the build process.

## 🔥 Example Usage

Here is an example `main.rs` to get the GDAL version report:

```rust
use gdal_win::version::VersionInfo;

fn main() {
    let report = VersionInfo::version_report();
    println!("{report}");
}
```

## 🖥️ Compatibility

- ✅ Tested and working on Windows x64.
- 🏗️ Requires a compatible version of GDAL binaries from GISInternals.

## 📜 License

This project follows the licensing terms of [georust/gdal](https://github.com/georust/gdal) and the respective GDAL libraries.

## 💖 Acknowledgments

- 🎉 [GISInternals](https://www.gisinternals.com) for providing Windows builds of GDAL.
- 🦀 [GeoRust](https://github.com/georust) for maintaining the Rust GDAL bindings.

---

**🤝 Contributions & Issues**
Feel free to open an issue or contribute to the project by submitting a pull request! 🚀

