// SPDX-License-Identifier: GPL-3.0-or-later

use std::borrow::Cow;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use anyhow::{bail, ensure, Context};
use clap::Parser;
use gip_metadata_compiler::Metadata;

/// A compiler/decompiler for GIP device metadata.
#[derive(clap::Parser, Debug)]
struct Arguments {
    /// The path to the input file.
    input_path: PathBuf,
    /// The path to write the output file to.
    ///
    /// Defaults to the input path with its file extension changed.
    output_path: Option<PathBuf>,
    /// Allow output path to overwrite an existing file.
    #[arg(short = 'o', long)]
    allow_overwrite: bool,
}

fn main() -> anyhow::Result<()> {
    let Arguments { input_path, output_path, allow_overwrite } = Arguments::parse();

    match input_path.extension() {
        Some(extension) if extension == "json" => compile(&input_path, output_path.as_deref(), allow_overwrite),
        Some(extension) if extension == "bin" => decompile(&input_path, output_path.as_deref(), allow_overwrite),
        _ => bail!("invalid input path {}", input_path.display()),
    }
}

fn validate_paths(path: &Path, extension: &str) -> anyhow::Result<()> {
    fn check_extension(path: &Path, extension: &str) -> bool {
        path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case(extension))
    }

    ensure!(
        check_extension(path, extension),
        "invalid path {} (expected extension to be {extension})",
        path.display()
    );

    Ok(())
}

fn create_file(path: &Path, allow_overwrite: bool) -> anyhow::Result<BufWriter<File>> {
    let file = match allow_overwrite {
        true => File::create(path),
        false => File::create_new(path),
    };
    file.map(BufWriter::new).context("couldn't create output file")
}

fn compile(input: &Path, output: Option<&Path>, allow_overwrite: bool) -> anyhow::Result<()> {
    let output = output
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(input.with_extension("bin")));
    validate_paths(&output, "bin")?;

    let input_file = File::open(input).context("failed to open input file")?;
    let metadata: Metadata = serde_json::from_reader(&input_file).context("failed to parse input file")?;

    let mut output_file = create_file(&output, allow_overwrite)?;
    metadata
        .compile(&mut output_file)
        .context("failed to write compiled metadata")?;

    Ok(())
}

fn decompile(input: &Path, output: Option<&Path>, allow_overwrite: bool) -> anyhow::Result<()> {
    let output = output
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(input.with_extension("json")));
    validate_paths(&output, "json")?;

    let mut input_file = File::open(input).context("failed to open input file")?;
    let metadata = Metadata::from_reader(&mut input_file).context("failed to read input file")?;

    let mut output_file = create_file(&output, allow_overwrite)?;
    serde_json::to_writer_pretty(&mut output_file, &metadata).context("failed to write decompiled metadata")?;

    Ok(())
}
