use crate::layout::*;

#[derive(Debug)]
pub struct Rules {
    pub horizontal_layout: LayoutMode,
    pub vertical_layout: LayoutMode,
    pub horizontal_rules: Vec<SmushingRule>,
    pub vertical_rules: Vec<SmushingRule>,
}

impl Rules {
    fn smushes_horizontal(char1: char, char2: char, hardblank: char) {
        unimplemented!()
    }
    fn smush_horizontal(char1: char, char2: char, hardblank: char) {
        unimplemented!()
    }
}
