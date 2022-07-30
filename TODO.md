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

  * Add valiadtion of RADIUS attributes according to the given types within schema
  * If `vendor` is not set to `None` - check that attribute id is `26` according
to the RFC 2865 (see Vendor-Specific).
  * add support for structured attrbiutes:

    * `3GPP-User-Location-Info`
    * `MS-TimeZone`
    * `Packet-Filter`

for more information see TS29.061
  * md5 - consider simd implemenetation
  * md5 - maybe we can find replacement for `put_u64_le` in stdlib
  * md5 - clean-up and simplify current implementation

## Lua API

  * Implement `libwl.radius_send` API
  * Implement `libwl.stop_with_error()` API
  * Condier to use [new_with](https://docs.rs/mlua/0.5.0/mlua/struct.Lua.html#method.new_with)
for sandboxing

## whirl

  * Add ability to pass list of RADIUS dictionaries
  * Check that scenario script was loaded without any errors
  * Check that config was parsed properly

## I/O

  * Check that given `imsi_range` is correct in the Ev::run()
