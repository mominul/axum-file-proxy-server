# axum-file-proxy-server
An example file proxy server implementation with Axum and Rust

It proxy the files from the [OpenBangla Keyboard's 2.0.0 version releases](https://github.com/OpenBangla/OpenBangla-Keyboard/releases/tag/2.0.0) and provides the endpoint `/file/{file_name}`
to download the named file from it.

## Example usage
```bash
cargo run
wget http://0.0.0.0:3000/file/OpenBangla-Keyboard_2.0.0-ubuntu20.04.deb
```

which will proxy the download from `https://github.com/OpenBangla/OpenBangla-Keyboard/releases/download/2.0.0/OpenBangla-Keyboard_2.0.0-ubuntu20.04.deb`

## Dependencies
* Axum `0.6`
* Reqwest `0.11`
* Tokio `1.27`