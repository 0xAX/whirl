# TODO list for whirl

## Building and testing

  * Add CI job for `cargo test`
  * For now libwl is linked with luajit-5.1 and it's path is kind of hard-coded
  * Add cargo fmt
  * Add clippy
  * whirl could be launched via `cargo run` without any issues but could be issues
with linking of libwl if we will run it as binary.

## Documentation

  * Add documentation of all possible items within `workload` object
  * Add schema for dictionaries format

## radius

  * add support for structured attrbiutes:

    * `3GPP-User-Location-Info`
    * `MS-TimeZone`
    * `Packet-Filter`

for more information see TS29.061

## Lua API

  * Implement `libwl.radius_send` API
  * Implement `libwl.stop_with_error()` API
