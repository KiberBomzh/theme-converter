use std::fs;
use std::path::Path;
use std::io::Result;


impl super::Theme {
    pub fn from_kitty(file: &Path) -> Result<Self> {
        use std::io::Read;


        let mut file = fs::File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut background = None;
        let mut foreground = None;
        let mut cursor = None;
        let mut selection_background = None;
        let mut selection_foreground = None;

        let mut normal_colors = super::Colors::default();
        let mut bright_colors = super::Colors::default();


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

        Ok(Self{
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
}
