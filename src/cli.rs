use std::path::PathBuf;
use std::str::FromStr;


pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
    pub input_type: Type,
    pub output_type: Type,
}
impl Args {
    pub fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut input: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut i_type: Option<Type> = None;
        let mut o_type: Option<Type> = None;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('h') | Long("help") => {
                    let types = Type::get_all_as_string();
                    println!(r#"Usage: theme-converter --input-type <INPUT_TYPE> --output-type <OUTPUT_TYPE> <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input path
  <OUTPUT>  Output path

Options:
  -i, --input-type <INPUT_TYPE>    Type of the input file [possible values: {types}]
  -o, --output-type <OUTPUT_TYPE>  Type of the output file [possible values: {types}]
  -h, --help                       Print help"#);
                    std::process::exit(0);
                },
                Short('i') | Long("input-type") => {
                    let i = parser.value()?.string()?;
                    i_type = match Type::from_str(&i) {
                        Ok(t) => Some(t),
                        Err(err) => return Err(lexopt::Error::Custom(err.into()))
                    }
                },
                Short('o') | Long("output-type") => {
                    let o = parser.value()?.string()?;
                    o_type = match Type::from_str(&o) {
                        Ok(t) => Some(t),
                        Err(err) => return Err(lexopt::Error::Custom(err.into()))
                    }
                },
                Value(val) if input.is_none() =>
                    input = Some(PathBuf::from(val.string()?)),
                Value(val) if output.is_none() =>
                    output = Some(PathBuf::from(val.string()?)),
                _ => return Err(arg.unexpected()),
            }
        }


        Ok(Self {
            input: input.ok_or("Missing argument: INPUT")?,
            output: output.ok_or("Missing argument: OUTPUT")?,
            input_type: i_type.ok_or("Missing argument: INPUT_TYPE")?,
            output_type: o_type.ok_or("Missing argument: OUTPUT_TYPE")?,
        })
    }
}


#[derive(Clone, Copy)]
pub enum Type {
    #[cfg(feature = "alacritty")]
    Alacritty,

    #[cfg(feature = "kitty")]
    Kitty,

    #[cfg(feature = "foot")]
    Foot
}
impl Type {
    pub fn extension(&self) -> String {
        use Type::*;

        match self {
            #[cfg(feature = "alacritty")]
            Alacritty => String::from("toml"),
    
            #[cfg(feature = "kitty")]
            Kitty => String::from("conf"),
    
            #[cfg(feature = "foot")]
            Foot => String::from("ini"),
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn get_all_as_string() -> String {
        let mut types = Vec::new();


        #[cfg(feature = "alacritty")]
        types.push("alacritty");

        #[cfg(feature = "kitty")]
        types.push("kitty");

        #[cfg(feature = "foot")]
        types.push("foot");


        types.join(", ")
    }
}
impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            #[cfg(feature = "alacritty")]
            "alacritty" => Ok(Self::Alacritty),

            #[cfg(feature = "kitty")]
            "kitty" => Ok(Self::Kitty),

            #[cfg(feature = "foot")]
            "foot" => Ok(Self::Foot),


            _ => Err(format!("There's no such Type: {s}. Allowed: [{}]", Type::get_all_as_string())),
        }
    }
}
