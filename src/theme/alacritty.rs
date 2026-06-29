impl super::Theme {
    pub fn from_alacritty(content: &str) -> std::io::Result<Self> {
        let mut background = None;
        let mut foreground = None;
        let mut cursor = None;
        let mut selection_background = None;
        let mut selection_foreground = None;
        let mut normal_colors = super::Colors::default();
        let mut bright_colors = super::Colors::default();

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


    pub fn to_alacritty(&self) -> String {
        let mut s = String::new();


        if self.background.is_some() || self.foreground.is_some() {
            s.push_str("[colors.primary]\n");
            Self::push_to_string_alacritty(&mut s, &self.background, "background");
            Self::push_to_string_alacritty(&mut s, &self.foreground, "foreground");
            s.push('\n');
        }

        if let Some(cursor) = &self.cursor {
            s.push_str("[colors.cursor]\n");
            s.push_str("cursor = \"#");
            s.push_str(cursor);
            s.push_str("\"\n\n");
        }

        if self.selection_background.is_some() || self.selection_foreground.is_some() {
            s.push_str("[colors.selection]\n");
            Self::push_to_string_alacritty(&mut s, &self.selection_background, "selection_background");
            Self::push_to_string_alacritty(&mut s, &self.selection_foreground, "selection_foreground");
            s.push('\n');
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
            s.push('\n');
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
            s.push('\n');
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
