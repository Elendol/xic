use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use displaydoc::Display;
use tracing::{event, instrument, Level};

fn existing_file(raw: &str) -> Result<Utf8PathBuf, String> {
    let pathbuf = Utf8Path::new(raw)
        .canonicalize_utf8()
        .map_err(|e| e.to_string())?;
    if pathbuf.exists() {
        Ok(pathbuf)
    } else {
        Err("file not found".to_string())
    }
}

#[derive(Parser, Debug, Display)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// compomics/ThermoRawFileParser xic command JSON input
    #[arg(long = "xic-json", value_name = "JSON_FILE", value_parser = existing_file)]
    pub(crate) thermorawfileparser_xic_json: Utf8PathBuf,
    /// compomics/ThermoRawFileParser xic command output
    #[arg(long = "xic-output", value_name = "JSON_FILE", value_parser = existing_file)]
    pub(crate) thermorawfileparser_xic_output: Utf8PathBuf,
    /// Output file
    #[arg(long = "output-file", value_name = "PNG_FILE", default_value_t = Utf8PathBuf::from("xic.png"))]
    pub(crate) output: Utf8PathBuf,
    /// Width
    #[arg(long = "width", value_name = "PIXELS", default_value_t = 800)]
    pub(crate) width: u32,
    /// Height
    #[arg(long = "height", value_name = "PIXELS", default_value_t = 600)]
    pub(crate) height: u32,
}
