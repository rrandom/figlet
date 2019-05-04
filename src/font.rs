#![allow(unused_variables)]
#![allow(dead_code)]

use crate::layout::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FontOpts {
    hard_blank: char,
    height: usize,
    baseline: usize,
    max_length: usize,
    old_layout: isize,
    comment_lines: usize,
    print_direction: usize,
    full_layout: Option<isize>,
    codetag_count: Option<usize>,
}

impl FontOpts {
    pub fn parse(line: &str) -> Result<FontOpts, std::num::ParseIntError> {
        let mut head = line.split_ascii_whitespace();
        let signature = head.next().unwrap();
        let height: usize = head.next().unwrap().parse()?;
        let baseline: usize = head.next().unwrap().parse()?;
        let max_length: usize = head.next().unwrap().parse()?;
        let old_layout: isize = head.next().unwrap().parse()?;
        let comment_lines: usize = head.next().unwrap().parse()?;
        let print_direction: usize = head.next().unwrap_or("0").parse()?;
        let full_layout = head.next().and_then(|fl| fl.parse::<isize>().ok());
        let codetag_count = head.next().and_then(|cc| cc.parse::<usize>().ok());

        Ok(FontOpts {
            hard_blank: signature.chars().last().unwrap(),
            height,
            baseline,
            max_length,
            old_layout,
            comment_lines,
            print_direction,
            full_layout,
            codetag_count,
        })
    }
}

#[test]
fn parse_font_head() {
    assert_eq!(
        FontOpts::parse("flf2a$ 8 8 20 -1 6").unwrap(),
        FontOpts {
            hard_blank: '$',
            height: 8,
            baseline: 8,
            max_length: 20,
            old_layout: -1,
            comment_lines: 6,
            print_direction: 0,
            full_layout: None,
            codetag_count: None,
        }
    );
}

#[derive(Debug)]
struct Rules {
    horizontal_layout: LayoutMode,
    vertical_layout: LayoutMode,
    horizontal_rules: Vec<SmushingRule>,
    vertical_rules: Vec<SmushingRule>,
}

#[derive(Debug, Default)]
pub struct Font {
    pub name: String,
    pub font_head: FontOpts,
    pub meta_data: String,
    pub chars: HashMap<u16, Vec<String>>,
}

impl Font {
    pub fn load_font(name: &str) -> Result<Font, std::num::ParseIntError> {
        let file_name: PathBuf = [".", "fonts", name].iter().collect();
        let mut file = File::open(file_name).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        Font::parse_font(name, &content)
    }

    pub fn parse_font(name: &str, data: &str) -> Result<Font, std::num::ParseIntError> {
        let lines = &mut data.lines();

        let font_head = FontOpts::parse(lines.next().unwrap())?;

        let char_nums = (32..126).chain(vec![196, 214, 220, 228, 246, 252, 223].into_iter());

        let comment: String = lines
            .take(font_head.comment_lines)
            .collect::<Vec<&str>>()
            .join("\n");

        let line_vec: Vec<_> = lines
            .map(|l| {
                let last_char = &l[l.len() - 1..];
                l.replace(last_char, "")
            })
            .collect();

        let fig_chars: Vec<Vec<_>> = line_vec
            .chunks(font_head.height)
            .map(|l| l.to_vec())
            .collect();

        let fig_chars: HashMap<u16, Vec<String>> = char_nums.zip(fig_chars.into_iter()).collect();

        Ok(Font {
            name: String::from(name),
            font_head,
            meta_data: comment,
            chars: fig_chars,
        })
    }

    fn get_layout_rules(&self) -> Rules {
        Font::get_layout(self.font_head.full_layout, self.font_head.old_layout)
    }

    fn get_layout(full_layout: Option<isize>, old_layout: isize) -> Rules {
        let mut horizontal_rules = vec![];
        let mut vertical_rules = vec![];
        let mut horizontal_layout: Option<LayoutMode> = None;
        let mut vertical_layout: Option<LayoutMode> = None;
        let mut ly = full_layout.unwrap_or(old_layout);

        let rules: Vec<_> = SmushingRule::iter().collect();
        for code in rules.into_iter().rev() {
            if ly >= code as isize {
                ly -= code as isize;
                match code.get_type() {
                    LayoutType::Horizontal => {
                        horizontal_rules.push(code);
                        horizontal_layout = Some(code.get_mode());
                    }
                    LayoutType::Vertical => {
                        vertical_rules.push(code);
                        vertical_layout = Some(code.get_mode());
                    }
                }
            }
        }
        if horizontal_layout.is_none() {
            if old_layout == 0 {
                horizontal_layout = Some(LayoutMode::Fitting);
                vertical_rules.push(SmushingRule::HorizontalFitting);
            } else if old_layout == -1 {
                horizontal_layout = Some(LayoutMode::FullWidth);
            }
        } else {
            let hl = horizontal_layout.as_ref().unwrap();
            if *hl == LayoutMode::ControlledSmush {
                horizontal_rules.retain(|r| *r != SmushingRule::HorizontalSmushing);
            }
        }

        if vertical_layout.is_none() {
            vertical_layout = Some(LayoutMode::FullWidth);
        } else {
            let vl = vertical_layout.as_ref().unwrap();
            if *vl == LayoutMode::ControlledSmush {
                vertical_rules.retain(|r| *r != SmushingRule::VerticalSmushing);
            }
        }

        Rules {
            horizontal_layout: horizontal_layout.unwrap(),
            vertical_layout: vertical_layout.unwrap(),
            horizontal_rules,
            vertical_rules,
        }
    }

    fn convert(&self, message: &str) {}
}

#[test]
#[ignore]
fn load_font() {
    let f = Font::load_font("4Max.flf");
    dbg!(f);
}

#[test]
fn get_layout_full_width() {
    let l = Font::get_layout(Some(0), -1);
    assert_eq!(l.horizontal_layout, LayoutMode::FullWidth);
    assert_eq!(l.vertical_layout, LayoutMode::FullWidth);
    assert_eq!(l.horizontal_rules.len(), 0);
    assert_eq!(l.vertical_rules.len(), 0);

    let l = Font::get_layout(None, -1);
    assert_eq!(l.horizontal_layout, LayoutMode::FullWidth);
    assert_eq!(l.vertical_layout, LayoutMode::FullWidth);
    assert_eq!(l.horizontal_rules.len(), 0);
    assert_eq!(l.vertical_rules.len(), 0);
}

#[test]
fn get_layout_kerning() {
    let l = Font::get_layout(Some(64), 0);
    assert_eq!(l.horizontal_layout, LayoutMode::Fitting);
    assert_eq!(l.vertical_layout, LayoutMode::FullWidth);
    assert_eq!(l.horizontal_rules.len(), 1);
    assert_eq!(l.horizontal_rules.get(0).unwrap(), &SmushingRule::HorizontalFitting);
    assert_eq!(l.vertical_rules.len(), 0);
}

#[test]
fn get_layout_smushing() {
    let l = Font::get_layout(Some(128), 0);
    assert_eq!(l.horizontal_layout, LayoutMode::UniversalSmush);
    assert_eq!(l.vertical_layout, LayoutMode::FullWidth);
    assert_eq!(l.horizontal_rules.len(), 1);
    assert_eq!(l.horizontal_rules.get(0).unwrap(), &SmushingRule::HorizontalSmushing);
    assert_eq!(l.vertical_rules.len(), 0);
}

#[test]
fn get_layout_controlled_smushing_slant() {
    // slant.flf
    let l = Font::get_layout(Some(18319), 15);
    assert_eq!(l.horizontal_layout, LayoutMode::ControlledSmush);
    assert_eq!(l.vertical_layout, LayoutMode::ControlledSmush);
    assert_eq!(l.horizontal_rules.len(), 4);
    assert_eq!(l.vertical_rules.len(), 3);

    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalOppositePair));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalHierarchy));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalUnderscore));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalEqualChar));

    assert!(l.vertical_rules.contains(&SmushingRule::VerticalHierarchy));
    assert!(l.vertical_rules.contains(&SmushingRule::VerticalUnderscore));
    assert!(l.vertical_rules.contains(&SmushingRule::VerticalEqualChar));
}

#[test]
fn get_layout_controlled_smushing_standard() {
    // starndard.flf
    let l = Font::get_layout(Some(24463), 15);
    assert_eq!(l.horizontal_layout, LayoutMode::ControlledSmush);
    assert_eq!(l.vertical_layout, LayoutMode::ControlledSmush);
    assert_eq!(l.horizontal_rules.len(), 4);
    assert_eq!(l.vertical_rules.len(), 5);

    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalOppositePair));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalHierarchy));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalUnderscore));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalEqualChar));

    assert!(l.vertical_rules.contains(&SmushingRule::VerticalVerticalLine));
    assert!(l.vertical_rules.contains(&SmushingRule::VerticalHierarchy));
    assert!(l.vertical_rules.contains(&SmushingRule::VerticalUnderscore));
    assert!(l.vertical_rules.contains(&SmushingRule::VerticalEqualChar));

    let l = Font::get_layout(None, 15);
    assert_eq!(l.horizontal_layout, LayoutMode::ControlledSmush);
    assert_eq!(l.horizontal_rules.len(), 4);
    assert_eq!(l.vertical_layout, LayoutMode::FullWidth);

    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalOppositePair));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalHierarchy));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalUnderscore));
    assert!(l.horizontal_rules.contains(&SmushingRule::HorizontalEqualChar));
}