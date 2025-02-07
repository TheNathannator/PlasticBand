// SPDX-License-Identifier: GPL-3.0-or-later

use gip_metadata_compiler::*;

fn main() {
    println!("Hello, world!");

    let metadata_text = include_str!("../docs/example_metadata.json");
    let metadata: Metadata = serde_json::from_str(metadata_text).unwrap();
    println!("{metadata:#?}");

    let metadata_ser = serde_json::to_string_pretty(&metadata).unwrap();
    println!("{metadata_ser}");
}
