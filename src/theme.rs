#[cfg(feature = "alacritty")]
mod alacritty;

#[cfg(feature = "kitty")]
mod kitty;

#[cfg(feature = "foot")]
mod foot;

#[cfg(feature = "termux")]
mod termux;



#[derive(Debug)]
pub struct Theme {
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
