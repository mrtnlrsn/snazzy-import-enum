extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/common.proto", "src/items.proto"], &["src/"]).unwrap();
}
