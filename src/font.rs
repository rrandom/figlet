
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq)]
struct FontOpts {
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

struct Font {
    font_head: FontOpts,
    meta_data: String,
    chars: HashMap<u16, String>,
}

impl Font {}
