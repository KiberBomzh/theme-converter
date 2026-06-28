mod theme;

use std::path::{PathBuf, Path};
use std::fs;

use clap::{Parser, ValueEnum};

use theme::Theme;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    input: PathBuf,

    /// Output path
    output: PathBuf,

    /// Type of the input file
    #[arg(short, long, required = true)]
    input_type: Type,

    /// Type of the output file
    #[arg(short, long, required = true)]
    output_type: Type,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Type {
    Kitty,
    Alacritty,
    Foot
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if !args.input.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Input is not exists!"));
    }

    let input_extension = get_file_extension(args.input_type);
    let output_extension = get_file_extension(args.output_type);

    if args.input.is_file() {
        convert_theme(
            &args.input,
            &args.output,
            args.input_type,
            args.output_type,
        )?;
    } else if args.input.is_dir() {
        for entry in fs::read_dir(args.input)? {
            let path = entry?.path();
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
                println!("Error in file {}", file_stem);
                println!("{:#?}\n", err);
            }
        }
    }


    Ok(())
}
fn convert_theme(
    input: &Path,
    output: &Path,
    input_type: Type,
    output_type: Type,
) -> std::io::Result<()> {
    use std::io::Write;


    let theme = match input_type {
        Type::Alacritty =>
            Theme::from_alacritty(input)?,
        Type::Kitty =>
            Theme::from_kitty(input)?,
        Type::Foot =>
            Theme::from_foot(input)?,
    };
    if !theme.is_some() {
        return Err(std::io::Error::other("Theme is empty!"));
    }

    let content = match output_type {
        Type::Alacritty => 
            theme.to_alacritty(),
        Type::Kitty =>
            theme.to_kitty(),
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
fn get_file_extension(t: Type) -> String {
    use Type::*;


    match t {
        Alacritty => String::from("toml"),
        Kitty => String::from("conf"),
        Foot => String::from("ini"),
    }
}

