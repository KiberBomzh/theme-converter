impl super::Theme {
    pub fn from_termux(content: &str) -> std::io::Result<Self> {
        let mut normal_colors = super::Colors::default();
        let mut bright_colors = super::Colors::default();


        for line in content.lines() {
            let l = line.trim();

            if l.starts_with("color10") {
                bright_colors.green = Self::get_value_from_termux(l, "color10");
            } else if l.starts_with("color11") {
                bright_colors.yellow = Self::get_value_from_termux(l, "color11");
            } else if l.starts_with("color12") {
                bright_colors.blue = Self::get_value_from_termux(l, "color12");
            } else if l.starts_with("color13") {
                bright_colors.magenta = Self::get_value_from_termux(l, "color13");
            } else if l.starts_with("color14") {
                bright_colors.cyan = Self::get_value_from_termux(l, "color14");
            } else if l.starts_with("color15") {
                bright_colors.white = Self::get_value_from_termux(l, "color15");
            } else if l.starts_with("color0") {
                normal_colors.black = Self::get_value_from_termux(l, "color0");
            } else if l.starts_with("color1") {
                normal_colors.red = Self::get_value_from_termux(l, "color1");
            } else if l.starts_with("color2") {
                normal_colors.green = Self::get_value_from_termux(l, "color2");
            } else if l.starts_with("color3") {
                normal_colors.yellow = Self::get_value_from_termux(l, "color3");
            } else if l.starts_with("color4") {
                normal_colors.blue = Self::get_value_from_termux(l, "color4");
            } else if l.starts_with("color5") {
                normal_colors.magenta = Self::get_value_from_termux(l, "color5");
            } else if l.starts_with("color6") {
                normal_colors.cyan = Self::get_value_from_termux(l, "color6");
            } else if l.starts_with("color7") {
                normal_colors.white = Self::get_value_from_termux(l, "color7");
            } else if l.starts_with("color8") {
                bright_colors.black = Self::get_value_from_termux(l, "color8");
            } else if l.starts_with("color9") {
                bright_colors.red = Self::get_value_from_termux(l, "color9");
            }
        }

        Ok(Self{
            background: None,
            foreground: None,
            cursor: None,
            selection_background: None,
            selection_foreground: None,
            normal_colors,
            bright_colors,
        })
    }
    fn get_value_from_termux(line: &str, keyword: &str) -> Option<String> {
        let value = line[keyword.len() + 1..].trim().replace('#', "").to_uppercase();

        if value.len() == 6 {
            Some(value.to_string())
        } else {
            None
        }
    }

    pub fn to_termux(&self) -> String {
        let mut s = String::new();


        Self::push_to_string_termux(&mut s, &self.normal_colors.black, "color0");
        Self::push_to_string_termux(&mut s, &self.bright_colors.black, "color8");

        Self::push_to_string_termux(&mut s, &self.normal_colors.red, "color1");
        Self::push_to_string_termux(&mut s, &self.bright_colors.red, "color9");

        Self::push_to_string_termux(&mut s, &self.normal_colors.green, "color2");
        Self::push_to_string_termux(&mut s, &self.bright_colors.green, "color10");

        Self::push_to_string_termux(&mut s, &self.normal_colors.yellow, "color3");
        Self::push_to_string_termux(&mut s, &self.bright_colors.yellow, "color11");

        Self::push_to_string_termux(&mut s, &self.normal_colors.blue, "color4");
        Self::push_to_string_termux(&mut s, &self.bright_colors.blue, "color12");

        Self::push_to_string_termux(&mut s, &self.normal_colors.magenta, "color5");
        Self::push_to_string_termux(&mut s, &self.bright_colors.magenta, "color13");

        Self::push_to_string_termux(&mut s, &self.normal_colors.cyan, "color6");
        Self::push_to_string_termux(&mut s, &self.bright_colors.cyan, "color14");

        Self::push_to_string_termux(&mut s, &self.normal_colors.white, "color7");
        Self::push_to_string_termux(&mut s, &self.bright_colors.white, "color15");


        s
    }
    fn push_to_string_termux(s: &mut String, value: &Option<String>, keyword: &str) {
        if let Some(c) = value {
            s.push_str(keyword);
            s.push_str(":\t#");
            s.push_str(c);
            s.push('\n');
        }
    }
}
