Project abandonned and replaced by [Archypix](https://github.com/Archypix/archypix-app-back).
# PicturesManager

An efficient pictures manager based on custom tags and file system organization.

Developed with [Tauri](https://tauri.app) (web app) with a Rust backend and a Rust WebAssembly frontend ([Yew](https://yew.rs)), this app aims to be fast, reliable, and beautiful.

# Goals

- Each gallery is a directory containing the images, with or without subdirectories. A file ``pictures_manager.json`` at the root of the gallery is storing the gallery configuration and cache data about each image: location on disk, date and location (base exif data), and tags.
- All images can have tags from tag groups.<br>
 <font size="-1">For example, you can create a tag group ``Picture Type`` that have tags ``Document``, ``Screenshot``, ``Family Pictures``.</font>
- The gallery directory can be reorganized with a new custom architecture depending on tags, date and location.
- The gallery cache can be updated from the filesystem, adding tags to new images depending on gallery architecture and images location on disk.
- It would also be possible to import directly a folder without updating the cache, associating new tags depending on the directory architecture.
- Exif tools: fix exif data, add exif to images that do not have exif data.
- This app is not only a utility app. It also aims to have complete viewing features.

# Main frameworks and libraries

- [Tauri](https://github.com/tauri-apps/tauri) framework for building desktop web apps based on a rust backend.
- [Yew](https://github.com/yewstack/yew) WebAssembly framework for building client web apps.
- [Fuent](https://github.com/projectfluent/fluent-rs) Fluent Project translations file format support.

# Build

## Cargo dev dependencies
```shell
cargo install tauri trunk
rustup target add wasm32-unknown-unknown
```

## Gexiv2 dependency
### Windows
Install and build dependencies with vcpkg :
```shell
vcpkg install exiv2
vcpkg install glib
```
Install pkg-config with chocolatey and add all vcpkg installs to the pkg-config path.
```shell
choco install pkgconfiglite
set PKG_CONFIG_PATH=C:\vcpkg\installed\x64-windows\lib\pkgconfig
```
Download gexiv2 source code and build it with meson and ninja that can be installed with:
```shell
pip install meson ninja
```
In Windows, functions must be exported manually, adding ``__declspec(dllexport)`` before each function declaration in the header files.
In the Visual Studio build environment (Developer Command Prompt for VS), run:
```shell
vcpkg install gobject-introspection
meson setup build --backend=ninja --buildtype=release -Dintrospection=false -Dvapi=false -Dpython3=false
cd build
ninja
ninja install
```
Add the gexiv2.dll to the PATH and the gexiv2.lib to the PKG_CONFIG_PATH environment variable.


## Dev
``cargo tauri dev``

## Release
``cargo tauri build``
