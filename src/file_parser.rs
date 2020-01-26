use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::line_parser;
use crate::line_parser::LineType;

#[derive(PartialEq, Debug, Default)]
pub struct Filelist {
    pub files: Vec<String>,
    pub incdirs: Vec<String>,
    pub defines: HashMap<String, String>,
    pub comments_present: bool,
}

impl Filelist {
    pub fn new() -> Filelist {
        Filelist {
            files: Vec::new(),
            incdirs: Vec::new(),
            defines: HashMap::new(),
            comments_present: false,
        }
    }

    pub fn extend(&mut self, other: Filelist) {
        self.files.extend(other.files);
        self.incdirs.extend(other.incdirs);
        self.defines.extend(other.defines);
        self.comments_present |= other.comments_present;
    }
}

pub fn parse_file(path: &str) -> Result<Filelist, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let mut filelist = Filelist::new();

    for line in contents.lines() {
        match line_parser::parse_line(line) {
            LineType::File(file) => filelist.files.push(file.to_string()),
            LineType::Define(define_map) => {
                for define in define_map.into_iter() {
                    filelist.defines.insert(define.0.to_string(), define.1.to_string());
                }
            },
            LineType::IncDir(incdirs) => {
                for dir in incdirs {
                    filelist.incdirs.push(dir.to_string());
                }
            }
            LineType::Comment => filelist.comments_present = true,
            LineType::Filelist(path) => {
                filelist.extend(parse_file(path)?);
            },
        }
    }
    Ok(filelist)
}
