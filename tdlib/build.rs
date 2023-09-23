// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use tdlib_tl_gen::generate_rust_code;
use tdlib_tl_parser::parse_tl_file;
use tdlib_tl_parser::tl::Definition;

/// Load the type language definitions from a network resource.
/// Parse errors will be printed to `stderr`, and only the
/// valid results will be returned.
fn load_tl(path: &str) -> io::Result<Vec<Definition>> {
    let contents = ureq::get(path).call().unwrap().into_string()?;
    Ok(parse_tl_file(contents)
        .filter_map(|d| match d {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("TL: parse error: {:?}", e);
                None
            }
        })
        .collect())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-env-changed=TDLIB_COMMIT_HASH");

    // Prevent linking libraries to avoid documentation failure
    #[cfg(not(feature = "dox"))]
    system_deps::Config::new().probe().unwrap();

    let commit_hash = env::var("TDLIB_COMMIT_HASH").unwrap_or_else(|_| "master".into());

    let path =
        format!("https://github.com/tdlib/td/raw/{commit_hash}/td/generate/scheme/td_api.tl");

    let definitions = load_tl(&path)?;

    let mut file = BufWriter::new(File::create(
        Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs"),
    )?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;
    Ok(())
}
