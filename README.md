# 🌍 gdal-win

**🗺️ GDAL for Rust / Windows 🎯**

`gdal-win` configures prebuilt GDAL libraries and bindings for Rust development on Windows. It automatically sets up GDAL binaries obtained from [GISInternals](https://www.gisinternals.com) and utilizes the GDAL Rust wrapper / bindings from [georust/gdal](https://github.com/georust/gdal).

## ✨ Features

- 🔧 Automatically configures GDAL using ⚡precompiled GDAL binaries for Windows x64 from [GISInternals](https://www.gisinternals.com) during the build process using [`gdal-setup`](https://github.com/geocrate/gdal-setup) internally.
- 🦀 Rust bindings to GDAL from [georust/gdal](https://github.com/georust/gdal).
- 🛠️ Simplifies GDAL setup for Rust developers on Windows.

## 📦 Setup

Ensure you have Rust installed. You can then add `gdal-win` to your Rust project by including the following in your `Cargo.toml`:

```toml
[package]
name = "gdal-win-example"
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

This will configure the necessary environment for GDAL during the build process, including setting up Static Libraries (`gdal_i.lib`) and DLLs.

## 🔥 Example Usage

Here is an example `main.rs` to get the GDAL version report:

```rust
use gdal_win::version::VersionInfo;

fn main() {
    let report = VersionInfo::version_report();
    println!("GDAL Version Report:\n{}", report);
}
```

## 🖥️ Compatibility

- ✅ Tested and working on Windows x64.

## 📜 License

This project follows the licensing terms of [georust/gdal](https://github.com/georust/gdal) and GDAL libraries from [GISInternals](https://www.gisinternals.com).

## 💖 Acknowledgments

- 🦀 [GeoRust](https://github.com/georust) for maintaining the Rust GDAL bindings.
- 🎉 [GISInternals](https://www.gisinternals.com) for providing Windows builds of GDAL.

---

**🤝 Contributions & Issues**

We welcome feature requests, bug reports, and improvements! Feel free to open an issue or submit a pull request. 🚀

