use std::path::{PathBuf, Path};
use std::io::{Read, Write};
use std::fs;

use clap::{Parser, ValueEnum};
use anyhow::{Result, anyhow};


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


fn main() -> Result<()> {
    let args = Args::parse();
    if !args.input.exists() {
        return Err(anyhow!("Input is not exists!"));
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
) -> Result<()> {
    let theme = match input_type {
        Type::Alacritty =>
            Theme::from_alacritty(input)?,
        Type::Kitty =>
            Theme::from_kitty(input)?,
        Type::Foot =>
            Theme::from_foot(input)?,
    };
    if !theme.is_some() {
        return Err(anyhow!("Theme is empty!"));
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

#[derive(Debug)]
struct Theme {
    background: Option<String>,
    foreground: Option<String>,
    selection_background: Option<String>,
    selection_foreground: Option<String>,
    cursor: Option<String>,
    normal_colors: Colors,
    bright_colors: Colors,
}

#[derive(Default, Debug)]
struct Colors {
    black: Option<String>,
    red: Option<String>,
    green: Option<String>,
    yellow: Option<String>,
    blue: Option<String>,
    magenta: Option<String>,
    cyan: Option<String>,
    white: Option<String>,
}

impl Theme {
    pub fn is_some(&self) -> bool {
        self.background.is_some() ||
        self.foreground.is_some() ||
        self.cursor.is_some() ||
        self.normal_colors.is_some() ||
        self.bright_colors.is_some()
    }

    pub fn from_alacritty(file: &Path) -> Result<Self> {
        let mut file = fs::File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut background = None;
        let mut foreground = None;
        let mut cursor = None;
        let mut selection_background = None;
        let mut selection_foreground = None;
        let mut normal_colors = Colors::default();
        let mut bright_colors = Colors::default();

        let mut in_normal = false;
        let mut in_bright = false;
        let mut in_primary = false;
        let mut in_selection = false;
        for line in content.lines() {
            let l = line.trim();

            if l.starts_with("[colors.normal]") {
                in_normal = true;
                in_bright = false;
                in_primary = false;
                in_selection = false;
            } else if l.starts_with("[colors.bright]") {
                in_bright = true;
                in_normal = false;
                in_primary = false;
                in_selection = false;
            } else if l.starts_with("[colors.primary]") {
                in_bright = false;
                in_normal = false;
                in_primary = true;
                in_selection = false;
            } else if l.starts_with("[colors.selection]") {
                in_bright = false;
                in_normal = false;
                in_primary = false;
                in_selection = true;
            } else if l.starts_with("[") {
                in_bright = false;
                in_normal = false;
                in_primary = false;
                in_selection = false;
            } else if in_normal {
                if l.starts_with("black") {
                    normal_colors.black = Self::get_value_from_alacritty(l);
                } else if l.starts_with("red") {
                    normal_colors.red = Self::get_value_from_alacritty(l);
                } else if l.starts_with("green") {
                    normal_colors.green = Self::get_value_from_alacritty(l);
                } else if l.starts_with("yellow") {
                    normal_colors.yellow = Self::get_value_from_alacritty(l);
                } else if l.starts_with("blue") {
                    normal_colors.blue = Self::get_value_from_alacritty(l);
                } else if l.starts_with("magenta") {
                    normal_colors.magenta = Self::get_value_from_alacritty(l);
                } else if l.starts_with("cyan") {
                    normal_colors.cyan = Self::get_value_from_alacritty(l);
                } else if l.starts_with("white") {
                    normal_colors.white = Self::get_value_from_alacritty(l);
                }
            } else if in_bright {
                if l.starts_with("black") {
                    bright_colors.black = Self::get_value_from_alacritty(l);
                } else if l.starts_with("red") {
                    bright_colors.red = Self::get_value_from_alacritty(l);
                } else if l.starts_with("green") {
                    bright_colors.green = Self::get_value_from_alacritty(l);
                } else if l.starts_with("yellow") {
                    bright_colors.yellow = Self::get_value_from_alacritty(l);
                } else if l.starts_with("blue") {
                    bright_colors.blue = Self::get_value_from_alacritty(l);
                } else if l.starts_with("magenta") {
                    bright_colors.magenta = Self::get_value_from_alacritty(l);
                } else if l.starts_with("cyan") {
                    bright_colors.cyan = Self::get_value_from_alacritty(l);
                } else if l.starts_with("white") {
                    bright_colors.white = Self::get_value_from_alacritty(l);
                }
            } else if in_primary {
                if l.starts_with("background") {
                    background = Self::get_value_from_alacritty(l);
                } else if l.starts_with("foreground") {
                    foreground = Self::get_value_from_alacritty(l);
                }
            } else if in_selection {
                if l.starts_with("background") {
                    selection_background = Self::get_value_from_alacritty(l);
                } else if l.starts_with("foreground") {
                    selection_foreground = Self::get_value_from_alacritty(l);
                }
            } else if l.starts_with("cursor") {
                cursor = Self::get_value_from_alacritty(l);
            }
        }


        Ok(Theme{
            background,
            foreground,
            cursor,
            selection_background,
            selection_foreground,
            normal_colors,
            bright_colors,
        })
    }
    fn get_value_from_alacritty(line: &str) -> Option<String> {
        let start_index = line.find('=')? + 1;
        let value = line[start_index..]
            .replace(['\'', '"', '#'], "")
            .trim()
            .to_uppercase();

        if value.len() == 6 {
            Some(value.to_string())
        } else {
            None
        }

    }

    pub fn from_kitty(file: &Path) -> Result<Self> {
        let mut file = fs::File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut background = None;
        let mut foreground = None;
        let mut cursor = None;
        let mut selection_background = None;
        let mut selection_foreground = None;

        let mut normal_colors = Colors::default();
        let mut bright_colors = Colors::default();


        for line in content.lines() {
            let l = line.trim();

            if l.starts_with("background") {
                background = Self::get_value_from_kitty(l, "background");
            } else if l.starts_with("foreground") {
                foreground = Self::get_value_from_kitty(l, "foreground");
            } else if l.starts_with("cursor") {
                cursor = Self::get_value_from_kitty(l, "cursor");
            } else if l.starts_with("selection_background") {
                selection_background = Self::get_value_from_kitty(l, "selection_background");
            } else if l.starts_with("selection_foreground") {
                selection_foreground = Self::get_value_from_kitty(l, "selection_foreground");
            } else if l.starts_with("color10") {
                bright_colors.green = Self::get_value_from_kitty(l, "color10");
            } else if l.starts_with("color11") {
                bright_colors.yellow = Self::get_value_from_kitty(l, "color11");
            } else if l.starts_with("color12") {
                bright_colors.blue = Self::get_value_from_kitty(l, "color12");
            } else if l.starts_with("color13") {
                bright_colors.magenta = Self::get_value_from_kitty(l, "color13");
            } else if l.starts_with("color14") {
                bright_colors.cyan = Self::get_value_from_kitty(l, "color14");
            } else if l.starts_with("color15") {
                bright_colors.white = Self::get_value_from_kitty(l, "color15");
            } else if l.starts_with("color0") {
                normal_colors.black = Self::get_value_from_kitty(l, "color0");
            } else if l.starts_with("color1") {
                normal_colors.red = Self::get_value_from_kitty(l, "color1");
            } else if l.starts_with("color2") {
                normal_colors.green = Self::get_value_from_kitty(l, "color2");
            } else if l.starts_with("color3") {
                normal_colors.yellow = Self::get_value_from_kitty(l, "color3");
            } else if l.starts_with("color4") {
                normal_colors.blue = Self::get_value_from_kitty(l, "color4");
            } else if l.starts_with("color5") {
                normal_colors.magenta = Self::get_value_from_kitty(l, "color5");
            } else if l.starts_with("color6") {
                normal_colors.cyan = Self::get_value_from_kitty(l, "color6");
            } else if l.starts_with("color7") {
                normal_colors.white = Self::get_value_from_kitty(l, "color7");
            } else if l.starts_with("color8") {
                bright_colors.black = Self::get_value_from_kitty(l, "color8");
            } else if l.starts_with("color9") {
                bright_colors.red = Self::get_value_from_kitty(l, "color9");
            }
        }

        Ok(Theme{
            background,
            foreground,
            cursor,
            selection_background,
            selection_foreground,
            normal_colors,
            bright_colors,
        })
    }
    fn get_value_from_kitty(line: &str, keyword: &str) -> Option<String> {
        let value = line[keyword.len()..].trim().replace('#', "").to_uppercase();

        if value.len() == 6 {
            Some(value.to_string())
        } else {
            None
        }
    }

    pub fn from_foot(file: &Path) -> Result<Self> {
        let mut file = fs::File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut background = None;
        let mut foreground = None;
        let mut cursor = None;
        let mut selection_background = None;
        let mut selection_foreground = None;

        let mut normal_colors = Colors::default();
        let mut bright_colors = Colors::default();


        for line in content.lines() {
            let l = line.trim();

            if l.starts_with("background") {
                background = Self::get_value_from_foot(l);
            } else if l.starts_with("foreground") {
                foreground = Self::get_value_from_foot(l);
            } else if l.starts_with("cursor") {
                cursor = Self::get_value_from_foot(l);
            } else if l.starts_with("selection-background") {
                selection_background = Self::get_value_from_foot(l);
            } else if l.starts_with("selection-foreground") {
                selection_foreground = Self::get_value_from_foot(l);
            } else if l.starts_with("regular0") {
                normal_colors.black = Self::get_value_from_foot(l);
            } else if l.starts_with("regular1") {
                normal_colors.red = Self::get_value_from_foot(l);
            } else if l.starts_with("regular2") {
                normal_colors.green = Self::get_value_from_foot(l);
            } else if l.starts_with("regular3") {
                normal_colors.yellow = Self::get_value_from_foot(l);
            } else if l.starts_with("regular4") {
                normal_colors.blue = Self::get_value_from_foot(l);
            } else if l.starts_with("regular5") {
                normal_colors.magenta = Self::get_value_from_foot(l);
            } else if l.starts_with("regular6") {
                normal_colors.cyan = Self::get_value_from_foot(l);
            } else if l.starts_with("regular7") {
                normal_colors.white = Self::get_value_from_foot(l);
            } else if l.starts_with("bright0") {
                bright_colors.black = Self::get_value_from_foot(l);
            } else if l.starts_with("bright1") {
                bright_colors.red = Self::get_value_from_foot(l);
            } else if l.starts_with("bright2") {
                bright_colors.green = Self::get_value_from_foot(l);
            } else if l.starts_with("bright3") {
                bright_colors.yellow = Self::get_value_from_foot(l);
            } else if l.starts_with("bright4") {
                bright_colors.blue = Self::get_value_from_foot(l);
            } else if l.starts_with("bright5") {
                bright_colors.magenta = Self::get_value_from_foot(l);
            } else if l.starts_with("bright6") {
                bright_colors.cyan = Self::get_value_from_foot(l);
            } else if l.starts_with("bright7") {
                bright_colors.white = Self::get_value_from_foot(l);
            }
        }

        Ok(Theme{
            background,
            foreground,
            cursor,
            selection_background,
            selection_foreground,
            normal_colors,
            bright_colors,
        })
    }
    fn get_value_from_foot(line: &str) -> Option<String> {
        let start_index = line.find('=')? + 1;
        let mut value = line[start_index..].trim();

        if value.contains(' ') {
            let end_index = value.rfind(' ')?;
            value = value[..end_index].trim();
        }

        if value.len() == 6 {
            Some(value.to_uppercase().to_string())
        } else {
            None
        }
    }


    pub fn to_foot(&self) -> String {
        let mut s = "[colors]\n".to_string();

        Self::push_to_string_foot(&mut s, &self.background, "background");
        Self::push_to_string_foot(&mut s, &self.foreground, "foreground");

        Self::push_to_string_foot(&mut s, &self.selection_background, "selection-background");
        Self::push_to_string_foot(&mut s, &self.selection_foreground, "selection-foreground");


        Self::push_to_string_foot(&mut s, &self.normal_colors.black, "regular0");
        Self::push_to_string_foot(&mut s, &self.normal_colors.red, "regular1");
        Self::push_to_string_foot(&mut s, &self.normal_colors.green, "regular2");
        Self::push_to_string_foot(&mut s, &self.normal_colors.yellow, "regular3");
        Self::push_to_string_foot(&mut s, &self.normal_colors.blue, "regular4");
        Self::push_to_string_foot(&mut s, &self.normal_colors.magenta, "regular5");
        Self::push_to_string_foot(&mut s, &self.normal_colors.cyan, "regular6");
        Self::push_to_string_foot(&mut s, &self.normal_colors.white, "regular7");

        Self::push_to_string_foot(&mut s, &self.bright_colors.black, "bright0");
        Self::push_to_string_foot(&mut s, &self.bright_colors.red, "bright1");
        Self::push_to_string_foot(&mut s, &self.bright_colors.green, "bright2");
        Self::push_to_string_foot(&mut s, &self.bright_colors.yellow, "bright3");
        Self::push_to_string_foot(&mut s, &self.bright_colors.blue, "bright4");
        Self::push_to_string_foot(&mut s, &self.bright_colors.magenta, "bright5");
        Self::push_to_string_foot(&mut s, &self.bright_colors.cyan, "bright6");
        Self::push_to_string_foot(&mut s, &self.bright_colors.white, "bright7");


        s
    }
    fn push_to_string_foot(s: &mut String, value: &Option<String>, keyword: &str) {
        if let Some(c) = value {
            s.push_str(keyword);
            s.push('=');
            s.push_str(c);
            s.push('\n');
        }
    }

    pub fn to_kitty(&self) -> String {
        let mut s = String::new();


        Self::push_to_string_kitty(&mut s, &self.background, "background");
        Self::push_to_string_kitty(&mut s, &self.foreground, "foreground");

        Self::push_to_string_kitty(&mut s, &self.selection_background, "selection_background");
        Self::push_to_string_kitty(&mut s, &self.selection_foreground, "selection_foreground");

        Self::push_to_string_kitty(&mut s, &self.cursor, "cursor");


        Self::push_to_string_kitty(&mut s, &self.normal_colors.black, "color0");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.black, "color8");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.red, "color1");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.red, "color9");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.green, "color2");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.green, "color10");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.yellow, "color3");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.yellow, "color11");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.blue, "color4");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.blue, "color12");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.magenta, "color5");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.magenta, "color13");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.cyan, "color6");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.cyan, "color14");

        Self::push_to_string_kitty(&mut s, &self.normal_colors.white, "color7");
        Self::push_to_string_kitty(&mut s, &self.bright_colors.white, "color15");


        s
    }
    fn push_to_string_kitty(s: &mut String, value: &Option<String>, keyword: &str) {
        if let Some(c) = value {
            s.push_str(keyword);
            s.push_str("\t\t#");
            s.push_str(c);
            s.push('\n');
        }
    }

    pub fn to_alacritty(&self) -> String {
        let mut s = String::new();


        if self.background.is_some() || self.foreground.is_some() {
            s.push_str("[colors.primary]\n");
            Self::push_to_string_alacritty(&mut s, &self.background, "background");
            Self::push_to_string_alacritty(&mut s, &self.foreground, "foreground");
        }

        if let Some(cursor) = &self.cursor {
            s.push_str("[colors.cursor]\n");
            s.push_str("cursor = \"#");
            s.push_str(cursor);
            s.push_str("\"\n");
        }

        if self.selection_background.is_some() || self.selection_foreground.is_some() {
            s.push_str("[colors.selection]\n");
            Self::push_to_string_alacritty(&mut s, &self.selection_background, "selection_background");
            Self::push_to_string_alacritty(&mut s, &self.selection_foreground, "selection_foreground");
        }


        if self.normal_colors.is_some() {
            s.push_str("[colors.normal]\n");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.black, "black");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.red, "red");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.green, "green");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.yellow, "yellow");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.blue, "blue");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.magenta, "magenta");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.cyan, "cyan");
            Self::push_to_string_alacritty(&mut s, &self.normal_colors.white, "white");
        }

        if self.normal_colors.is_some() {
            s.push_str("[colors.bright]\n");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.black, "black");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.red, "red");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.green, "green");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.yellow, "yellow");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.blue, "blue");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.magenta, "magenta");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.cyan, "cyan");
            Self::push_to_string_alacritty(&mut s, &self.bright_colors.white, "white");
        }


        s
    }
    fn push_to_string_alacritty(s: &mut String, value: &Option<String>, keyword: &str) {
        if let Some(c) = value {
            s.push_str(keyword);
            s.push_str(" = \"#");
            s.push_str(c);
            s.push_str("\"\n");
        }
    }
}

impl Colors {
    pub fn is_some(&self) -> bool {
        self.black.is_some() ||
        self.red.is_some() ||
        self.green.is_some() ||
        self.yellow.is_some() ||
        self.blue.is_some() ||
        self.magenta.is_some() ||
        self.cyan.is_some() ||
        self.white.is_some()
    }
}
