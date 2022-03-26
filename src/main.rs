use anyhow::{bail, Result};
use prost_types::compiler::{code_generator_response::File, CodeGeneratorResponse};
use protoc_gen_prost::{utils::*, Generator};
use tonic_build::{Builder, ServiceGenerator};

fn main() {
    let res = match gen_files() {
        Ok(file) => CodeGeneratorResponse { file, ..Default::default() },
        Err(e) => CodeGeneratorResponse { error: Some(format!("{e:?}")), ..Default::default() },
    };
    response_to_env(res).unwrap();
}

fn gen_files() -> Result<Vec<File>> {
    let req = request_from_env()?;

    let (mut gen, opts) = Generator::new_from_opts(split_escaped(req.parameter(), ','));
    let (builder, opts) = builder_from_opts(opts);

    if !opts.is_empty() {
        bail!("Unknown opts:\n - {}", opts.join("\n - "));
    }

    gen.config.service_generator(Box::new(ServiceGenerator::new(builder)));
    gen.generate(req.proto_file)
}

fn builder_from_opts(opts: Vec<String>) -> (Builder, Vec<String>) {
    let mut builder = tonic_build::configure();
    let mut leftovers = Vec::new();

    for opt in opts {
        builder = match opt.trim().splitn(3, '=').collect::<Vec<_>>().as_slice() {
            [] | [""] => builder,
            ["no_build_client"] => builder.build_client(false),
            ["no_build_server"] => builder.build_server(false),
            ["server_mod_attribute", k, v] => builder.server_mod_attribute(k, v),
            ["server_attribute", k, v] => builder.server_attribute(k, v),
            ["client_mod_attribute", k, v] => builder.client_mod_attribute(k, v),
            ["client_attribute", k, v] => builder.client_attribute(k, v),
            ["proto_path", v] => builder.proto_path(v),
            ["disable_package_emission"] => builder.disable_package_emission(),
            _ => {
                leftovers.push(opt);
                builder
            }
        }
    }

    (builder, leftovers)
}
