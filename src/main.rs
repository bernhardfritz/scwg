use std::path::PathBuf;

use clap::{builder::TypedValueParser, Parser};

#[derive(Clone)]
struct ValidatorRegexValueParser {
    regex: regex::Regex,
}

impl ValidatorRegexValueParser {
    fn new(regex: &str) -> Self {
        Self {
            regex: regex::Regex::new(regex).unwrap(),
        }
    }
}

macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

impl TypedValueParser for ValidatorRegexValueParser {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = ok!(value.to_str().ok_or_else(|| {
            clap::Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd)
        }));
        if self.regex.is_match(value) {
            Ok(value.to_owned())
        } else {
            let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(
                    clap::error::ContextKind::InvalidArg,
                    clap::error::ContextValue::String(arg.to_string()),
                );
            }
            err.insert(
                clap::error::ContextKind::InvalidValue,
                clap::error::ContextValue::String(value.to_owned()),
            );
            Err(err)
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(value_parser=clap::value_parser!(u32).range(1..))]
    width: u32,
    #[arg(value_parser=clap::value_parser!(u32).range(1..))]
    height: u32,
    #[arg(value_parser=ValidatorRegexValueParser::new("^[0-9a-fA-F]{6}$"))]
    hex: String,
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let hex = u32::from_str_radix(&args.hex, 16).unwrap();
    let r = ((hex & 0xff0000) >> 16).try_into().unwrap();
    let g = ((hex & 0x00ff00) >> 8).try_into().unwrap();
    let b = ((hex & 0x0000ff) >> 0).try_into().unwrap();
    let rgb = [r, g, b];
    let capacity = (args.width * args.height * 3).try_into().unwrap();
    let mut buf = Vec::with_capacity(capacity);
    for i in 0..capacity {
        buf.push(rgb[i % 3]);
    }
    image::save_buffer(
        args.path,
        &buf,
        args.width,
        args.height,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
