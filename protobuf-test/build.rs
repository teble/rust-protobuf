extern crate env_logger;
extern crate glob;
extern crate log;

extern crate protoc;
extern crate protoc_rust;

extern crate protobuf_test_common;

use protobuf_test_common::build::*;
use protoc::Protoc;
use std::path::PathBuf;

fn test_protoc_bin_path() -> PathBuf {
    let path = protoc_bin_vendored::protoc_bin_path().unwrap();
    assert!(Protoc::from_path(&path).version().unwrap().is_3());
    path
}

fn codegen() -> protoc_rust::Codegen {
    let mut codegen = protoc_rust::Codegen::new();
    codegen.protoc_path(test_protoc_bin_path());
    codegen.extra_arg("--experimental_allow_proto3_optional");
    codegen
}

fn gen_in_dir(dir: &str, include_dir: &str) {
    gen_in_dir_impl(
        dir,
        |GenInDirArgs {
             out_dir,
             input,
             customize,
         }| {
            codegen()
                .out_dir(out_dir)
                .inputs(input)
                .includes(&["../proto", include_dir])
                .customize(customize)
                .run()
        },
    );
}

fn generate_in_common() {
    gen_in_dir("src/common/v2", "src/common/v2");

    copy_tests_v2_v3("src/common/v2", "src/common/v3");
    gen_in_dir("src/common/v3", "src/common/v3");
}

fn generate_in_v2_v3() {
    gen_in_dir("src/v2", "src/v2");

    gen_in_dir("src/v3", "src/v3");

    gen_in_dir("src/google/protobuf", "src");
}

fn generate_interop() {
    codegen()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .input("../interop/cxx/interop_pb.proto")
        .run()
        .unwrap();
}

fn generate_pb_rs() {
    generate_in_common();
    generate_in_v2_v3();
    generate_interop();
}

fn main() {
    env_logger::init();

    cfg_serde();

    clean_old_files();

    generate_pb_rs();
}
