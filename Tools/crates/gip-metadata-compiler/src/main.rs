// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs::File;
use std::io::{BufWriter, Cursor};
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

    /// When compiling, output the compiled data as hexadecimal text.
    ///
    /// Ignored when decompiling, mode is inferred from the input file extension (.bin for binary, .txt for text).
    #[arg(short = 't', long)]
    text_mode: bool,
}

fn main() -> anyhow::Result<()> {
    let mut args = Arguments::parse();

    match args.input_path.extension() {
        Some(extension) if extension == "json" => compile(args),
        Some(extension) if extension == "txt" => {
            args.text_mode = true;
            decompile(args)
        },
        Some(extension) if extension == "bin" => {
            args.text_mode = false;
            decompile(args)
        },
        _ => bail!("invalid input path {}", args.input_path.display()),
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

fn parse_hex_bytes(text: &str) -> Result<Vec<u8>, anyhow::Error> {
    text.split(|c: char| c.is_ascii_whitespace() || c == '-')
        .filter(|t| !t.is_empty())
        .map(|t| {
            ensure!(t.len() == 2, "invalid hex byte length {}, expected 2 characters", t.len());
            u8::from_str_radix(t, 16).with_context(|| format!("invalid hex byte {}", t))
        })
        .collect()
}

fn compile(args: Arguments) -> anyhow::Result<()> {
    let Arguments {
        input_path,
        output_path,
        allow_overwrite,
        text_mode,
    } = args;

    let output_extension = match text_mode {
        true => "txt",
        false => "bin",
    };

    let output_path = match output_path {
        Some(output_path) => {
            validate_paths(&output_path, output_extension)?;
            output_path
        },
        None => input_path.with_extension(output_extension),
    };

    let input_file = File::open(input_path).context("failed to open input file")?;
    let metadata: Metadata = serde_json::from_reader(&input_file).context("failed to parse input file")?;

    let mut output_file = create_file(&output_path, allow_overwrite)?;
    metadata
        .compile(&mut output_file)
        .context("failed to write compiled metadata")?;

    Ok(())
}

fn decompile(args: Arguments) -> anyhow::Result<()> {
    let Arguments {
        input_path,
        output_path,
        allow_overwrite,
        text_mode,
    } = args;

    let output_path = match output_path {
        Some(output_path) => {
            validate_paths(&output_path, "json")?;
            output_path
        },
        None => input_path.with_extension("json"),
    };

    let mut input_file = File::open(input_path).context("failed to open input file")?;
    let metadata = match text_mode {
        true => {
            let input_text = std::io::read_to_string(input_file).context("failed to read input text")?;
            let input_bytes = parse_hex_bytes(&input_text)?;
            Metadata::from_reader(Cursor::new(input_bytes)).context("failed to read input file")?
        },
        false => Metadata::from_reader(&mut input_file).context("failed to read input file")?,
    };

    let mut output_file = create_file(&output_path, allow_overwrite)?;
    serde_json::to_writer_pretty(&mut output_file, &metadata).context("failed to write decompiled metadata")?;

    Ok(())
}
