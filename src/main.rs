mod theme;
mod cli;


use std::path::Path;
use std::fs;

use theme::Theme;
use cli::{Args, Type};


fn main() {
    let args = match Args::parse() {
        Ok(a) => a,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        },
    };
    if !args.input.exists() {
        eprintln!("Input is not exists!");
        std::process::exit(1);
    }

    let input_extension = args.input_type.extension();
    let output_extension = args.output_type.extension();

    if args.input.is_file() {
        if let Err(err) = convert_theme(
            &args.input,
            &args.output,
            args.input_type,
            args.output_type,
        ) {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    } else if args.input.is_dir() {
        let entries = match fs::read_dir(args.input) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        for entry in entries {
            let path = if let Ok(e) = entry { e.path() }
                else { continue };
            if path.is_dir() { continue }


            if let Some(e) = path.extension() && e.to_string_lossy() == input_extension {}
                else { continue }

            let file_stem = if let Some(stem) = path.file_stem() { stem.to_string_lossy() }
                else { continue };


            if let Err(err) = convert_theme(
                &path,
                &args.output.join(format!("{file_stem}.{output_extension}")),
                args.input_type,
                args.output_type,
            ) {
                eprintln!("Error in file {}", file_stem);
                eprintln!("{}\n", err);
            }
        }
    }
}
fn convert_theme(
    input: &Path,
    output: &Path,
    input_type: Type,
    output_type: Type,
) -> std::io::Result<()> {
    use std::io::Write;


    let theme: Theme = match input_type {
        #[cfg(feature = "alacritty")]
        Type::Alacritty =>
            Theme::from_alacritty(input)?,

        #[cfg(feature = "kitty")]
        Type::Kitty =>
            Theme::from_kitty(input)?,

        #[cfg(feature = "foot")]
        Type::Foot =>
            Theme::from_foot(input)?,
    };
    if !theme.is_some() {
        return Err(std::io::Error::other("Theme is empty!"));
    }

    let content: String = match output_type {
        #[cfg(feature = "alacritty")]
        Type::Alacritty => 
            theme.to_alacritty(),

        #[cfg(feature = "kitty")]
        Type::Kitty =>
            theme.to_kitty(),

        #[cfg(feature = "foot")]
        Type::Foot =>
            theme.to_foot(),
    };
    if let Some(parent) = output.parent() && !parent.exists() {
        fs::create_dir_all(parent)?;
    }
    let file = fs::File::create(output)?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(content.as_bytes())?;


    Ok(())
}
