use crate::layout::*;

#[derive(Debug)]
pub struct Rules {
    pub horizontal_layout: LayoutMode,
    pub vertical_layout: LayoutMode,
    pub horizontal_rules: Vec<SmushingRule>,
    pub vertical_rules: Vec<SmushingRule>,
}

impl Rules {
    pub fn smushes_horizontal(&self, char1: char, char2: char, hardblank: char) -> bool {
        self.horizontal_rules
            .iter()
            .any(|r| r.smush(char1, char2, hardblank).is_some())
    }
    pub fn smush_horizontal(&self, char1: char, char2: char, hardblank: char) -> Option<char> {
        if char1 == ' ' {
            return Some(char2);
        }
        if char2 == ' ' {
            return Some(char1);
        }

        if self.horizontal_layout == LayoutMode::UniversalSmush {
            return SmushingRule::HorizontalSmushing.smush(char1, char2, hardblank);
        }
        for r in self.horizontal_rules.iter() {
            let smush = r.smush(char1, char2, hardblank);
            if smush.is_some() {
                return smush;
            }
        }
        None
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            horizontal_layout: LayoutMode::FullWidth,
            vertical_layout: LayoutMode::FullWidth,
            horizontal_rules: vec![],
            vertical_rules: vec![],
        }
    }
}
