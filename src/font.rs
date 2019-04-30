#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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
}

#[test]
fn load_font() {
    let f = Font::load_font("4Max.flf");
    dbg!(f);
}
