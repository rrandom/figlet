use std::str::FromStr;
use strum_macros::{Display, EnumIter};

pub enum LayoutType {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LayoutMode {
    FullWidth,
    Fitting,
    ControlledSmush,
    UniversalSmush,
}

#[derive(EnumIter, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum SmushingRule {
    HorizontalEqualChar = 1,
    HorizontalUnderscore = 2,
    HorizontalHierarchy = 4,
    HorizontalOppositePair = 8,
    HorizontalBigX = 16,
    HorizontalHardblank = 32,
    HorizontalFitting = 64,
    HorizontalSmushing = 128,
    VerticalEqualChar = 256,
    VerticalUnderscore = 512,
    VerticalHierarchy = 1024,
    VerticalHorizontalLine = 2048,
    VerticalVerticalLine = 4096,
    VerticalFitting = 8192,
    VerticalSmushing = 16384,
}

impl SmushingRule {
    pub fn smush(self, char1: char, char2: char, hardblank: char) -> Option<char> {
        match self {
            SmushingRule::HorizontalEqualChar => {
                if char1 == char2 && char1 != hardblank {
                    Some(char1)
                } else {
                    None
                }
            }
            SmushingRule::HorizontalUnderscore => {
                let chars = "|/\\[]{}()<>";
                if char1 == '_' && chars.contains(char2) {
                    Some(char2)
                } else if char2 == '_' && chars.contains(char1) {
                    Some(char1)
                } else {
                    None
                }
            }
            SmushingRule::HorizontalHierarchy => {
                let classes = "| /\\ [] {} () <>";
                let pos1 = classes.find(char1);
                let pos2 = classes.find(char2);
                if pos1.is_some() && pos2.is_some() {
                    let pos1 = pos1.unwrap();
                    let pos2 = pos2.unwrap();
                    if pos1 != pos2 && (pos1 as i64 - pos2 as i64).abs() != 1 {
                        let max_pos = pos1.max(pos2);
                        return char::from_str(&classes[max_pos..=max_pos]).ok();
                    }
                }
                None
            }
            SmushingRule::HorizontalOppositePair => {
                let brackets = "[] {} ()";
                let pos1 = brackets.find(char1);
                let pos2 = brackets.find(char2);
                if pos1.is_some() && pos2.is_some() {
                    let pos1 = pos1.unwrap();
                    let pos2 = pos2.unwrap();
                    if (pos1 as i64 - pos2 as i64).abs() == 1 {
                        return Some('|');
                    }
                }
                None
            }
            SmushingRule::HorizontalBigX => {
                if char1 == '/' && char2 == '\\' {
                    Some('|')
                } else if char1 == '\\' && char2 == '/' {
                    Some('Y')
                } else if char1 == '>' && char2 == '<' {
                    Some('X')
                } else {
                    None
                }
            }
            SmushingRule::HorizontalHardblank => {
                if char1 == hardblank && char2 == hardblank {
                    Some(hardblank)
                } else {
                    None
                }
            }
            SmushingRule::HorizontalFitting => {
                if char1 == ' ' && char2 == ' ' {
                    Some(' ')
                } else {
                    None
                }
            }
            SmushingRule::HorizontalSmushing => {
                if char1 != hardblank && char2 != hardblank {
                    Some(char2)
                } else {
                    None
                }
            }
            SmushingRule::VerticalEqualChar => {
                if char1 == char2 && char1 != hardblank {
                    Some(char1)
                } else {
                    None
                }
            }
            SmushingRule::VerticalUnderscore => {
                let chars = "|/\\[]{}()<>";
                if char1 == '_' && chars.contains(char2) {
                    Some(char2)
                } else if char2 == '_' && chars.contains(char1) {
                    Some(char1)
                } else {
                    None
                }
            }
            SmushingRule::VerticalHierarchy => {
                let classes = "| /\\ [] {} () <>";
                let pos1 = classes.find(char1);
                let pos2 = classes.find(char2);
                if pos1.is_some() && pos2.is_some() {
                    let pos1 = pos1.unwrap();
                    let pos2 = pos2.unwrap();
                    if pos1 != pos2 && (pos1 as i64 - pos2 as i64).abs() != 1 {
                        let max_pos = pos1.max(pos2);
                        return char::from_str(&classes[max_pos..=max_pos]).ok();
                    }
                }
                None
            }
            SmushingRule::VerticalHorizontalLine => {
                if char1 == '-' && char2 == '_' || char1 == '_' && char2 == '-' {
                    return Some('=');
                }
                None
            }
            SmushingRule::VerticalVerticalLine => {
                if char1 == '|' && char2 == '|' {
                    return Some('|');
                }
                None
            }
            _ => None,
        }
    }

    pub fn get_type(self) -> LayoutType {
        match self as isize {
            code if code <= 255 => LayoutType::Horizontal,
            _ => LayoutType::Vertical,
        }
    }

    pub fn get_mode(self) -> LayoutMode {
        match self as isize {
            code if code == 8192 || code == 64 => LayoutMode::Fitting,
            code if code == 128 || code == 8192 => LayoutMode::UniversalSmush,
            _ => LayoutMode::ControlledSmush,
        }
    }
}

#[test]
fn test_horizontal_equal_char() {
    let r = SmushingRule::HorizontalEqualChar;
    assert_eq!(r.smush('a', 'a', '$').unwrap(), 'a');
    assert!(r.smush('$', 'a', '$').is_none());
    assert!(r.smush('$', '$', '$').is_none());
}

#[test]
fn test_horizontal_underscore() {
    let r = SmushingRule::HorizontalUnderscore;
    assert!(r.smush('$', '$', '$').is_none());
    assert!(r.smush('b', 'a', '$').is_none());
    let values = vec!['|', '/', '\\', '[', ']', '{', '}', '(', ')', '<', '>'];
    for v in values.iter() {
        assert!(r.smush('a', *v, '$').is_none());
        assert!(r.smush(*v, 'a', '$').is_none());
        assert_eq!(r.smush('_', *v, '$').unwrap(), *v);
        assert_eq!(r.smush(*v, '_', '$').unwrap(), *v);
    }
}

#[test]
fn test_horizontal_hierarchy() {
    let r = SmushingRule::HorizontalHierarchy;
    assert!(r.smush('|', '|', '$').is_none());
    assert_eq!(r.smush('|', '/', '$').unwrap(), '/');
    assert_eq!(r.smush('|', '>', '$').unwrap(), '>');
    assert_eq!(r.smush('>', '|', '$').unwrap(), '>');
    assert!(r.smush(']', '[', '$').is_none());
}

#[test]
fn test_horizontal_opposite_pair() {
    let r = SmushingRule::HorizontalOppositePair;
    assert!(r.smush('a', 'b', '$').is_none());
    assert!(r.smush('[', '[', '$').is_none());
    assert!(r.smush('[', '}', '$').is_none());
    assert_eq!(r.smush('[', ']', '$').unwrap(), '|');
    assert_eq!(r.smush(')', '(', '$').unwrap(), '|');
}

#[test]
fn test_horizontal_big_x() {
    let r = SmushingRule::HorizontalBigX;
    assert!(r.smush('a', 'b', '$').is_none());
    assert_eq!(r.smush('/', '\\', '$').unwrap(), '|');
    assert_eq!(r.smush('>', '<', '$').unwrap(), 'X');
    assert_eq!(r.smush('\\', '/', '$').unwrap(), 'Y');
}

#[test]
fn test_horinaltal_hardblank() {
    let r = SmushingRule::HorizontalHardblank;
    assert_eq!(r.smush('$', '$', '$').unwrap(), '$');
    assert!(r.smush('a', 'b', '$').is_none());
}
