use std::fs;
use std::path::Path;
use std::io::Result;


impl super::Theme {
    pub fn from_foot(file: &Path) -> Result<Self> {
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
}
