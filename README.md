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

# Build

### Dev
``cargo tauri dev``

### Release
``cargo tauri build``
