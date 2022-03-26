# protoc-gen-tonic

A `protoc` plugin to generate Tonic code.

This is the complementary plugin to
[`protoc-gen-prost`](https://github.com/Tuetuopay/protoc-gen-prost), but with
the added gRPC stack provided by Tonic.

While the recommended way to use `tonic` in Rust projects is with `tonic-build`
and running `protoc` through Cargo and `build.rs`, a  protoc plugin allows to
get a standard workflow in the protobuf ecosystem. Also, precompiling proto
files to Rust code as files has some advantages:

- easier to share compiled code across multiple projects, since they don't all
  need to setup the prost build
- rust code can be stored in an easy to browse fashion (git, ...)
- integrates well with standard protobuf/grpc/... tooling
- compatible with `buf`

## Usage

```bash
cargo install protoc-gen-tonic
protoc -I proto --tonic_out=hello-rs hello.proto
```

Please note that, as opposed to `protoc-gen-go-grpc` that works _in addition_
to `protoc-gen-go`, `protoc-gen-tonic` integrates the functionality of the
`prost` plugin. Thus you do not need to run both plugin to get the message and
service code generation, only this plugin.

Please refer to the `protoc-gen-prost` for detailed usage reference, including
module / crate generation. All options exposed by `protoc-gen-prost` are
available with this plugin through the `--tonic_opt` flag. The following options
are specific to `protoc-gen-tonic` and are imported from `tonic_build`, please
refer to
[`tonic_build::Builder`](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html)
documentation for detailed documentation. Note that a few of those are imported
from `prost_build` and exposed as well, but by `protoc-gen-prost`.

- `no_build_client`: disable client code generation
- `no_build_server`: disable server code generation
- `server_mod_attribute=<key>=<value>`, repeatable
- `server_attribute=<key>=<value>`, repeatable
- `client_mod_attribute=<key>=<value>`, repeatable
- `client_attribute=<key>=<value>`, repeatable
- `proto_path=<value>`: string
- `disable_package_emission`: boolean
