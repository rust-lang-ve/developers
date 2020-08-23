<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars3.githubusercontent.com/u/68873317?s=120&v=4" height="120" width="120" />
  </div>
  <h1 align="center">developers</h1>
  <span align="center">Developers, Developers, Developers, Developers, Developers...</span>
</div>

**developers** is a directory of developers to help people interested to find and publish profiles.

## Development

### Requisites

It's required to have `wasm-pack`, `cargo-make` and `simple-http-server` installed with `cargo install`.

### Reloading

You can serve an `simple-http-server` to server files at `static` directory by issuing: `cargo make serve`.

This command will run the `serve` task available in the `Makefile.toml`.

### Build

This project is build using `wasm-pack`, the `build.sh` is a "shorcut" script that issues a `wasm-pack`
command which outputs the source to the `static` directory

```
bash ./build.sh
```
