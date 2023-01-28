#![feature(portable_simd)]
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::simd::{u16x32, SimdOrd, SimdPartialOrd, SimdUint};
use std::str::FromStr;
use std::{io, io::Write};

#[derive(Debug)]
enum Size {
    Small,
    Large,
}
impl Size {
    fn as_usize(&self) -> usize {
        match self {
            Size::Small => 0,
            Size::Large => 1,
        }
    }
}
impl FromStr for Size {
    type Err = ();
    fn from_str(input: &str) -> Result<Size, Self::Err> {
        match input {
            "S" => Ok(Size::Small),
            "L" => Ok(Size::Large),
            _ => Err(()),
        }
    }
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Small => write!(f, "S"),
            Size::Large => write!(f, "L"),
        }
    }
}

fn char_to_stem_index(value: char) -> usize {
    const LOWER_BOUND: usize = 'a' as usize;
    value as usize - LOWER_BOUND
}
fn stem_index_to_char(stem_index: usize) -> char {
    const LOWER_BOUND: usize = 'a' as usize;
    char::from_u32((LOWER_BOUND + stem_index) as u32).unwrap()
}

#[derive(Debug)]
struct Design {
    name: char,
    size: Size,
    total: u16,
    min_stems: u16x32,
    max_stems: u16x32,
}
impl FromStr for Design {
    type Err = ();
    fn from_str(input: &str) -> Result<Design, Self::Err> {
        lazy_static! {
            static ref DESIGN_RE: Regex = Regex::new(
                r"(?x)^
                (?P<name>[A-Z]
                (?P<size>[SL]))
                (?P<stems>.*?)
                (?P<total>\d+)$"
            )
            .unwrap();
            static ref STEMS_RE: Regex = Regex::new(r"(?P<max>\d+)(?P<species>[a-z])").unwrap();
        }
        let design_match = DESIGN_RE.captures(input).unwrap();
        let name = design_match
            .name("name")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap();
        let size = Size::from_str(design_match.name("size").unwrap().as_str()).unwrap();
        let total = design_match
            .name("total")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let mut min_stems = u16x32::splat(0);
        let mut max_stems = u16x32::splat(0);
        let stems = design_match.name("stems").unwrap().as_str();
        for stem_match in STEMS_RE.captures_iter(stems) {
            let stem_index = char_to_stem_index(
                stem_match
                    .name("species")
                    .unwrap()
                    .as_str()
                    .chars()
                    .nth(0)
                    .unwrap(),
            );
            let max = stem_match
                .name("max")
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            min_stems[stem_index] = 1;
            max_stems[stem_index] = max;
        }
        Ok(Design {
            name,
            size,
            total,
            min_stems,
            max_stems,
        })
    }
}

#[derive(Debug)]
struct ProductionLine {
    stems: u16x32,
    designs: Vec<Design>,
}
impl ProductionLine {
    pub fn new() -> Self {
        Self {
            stems: u16x32::splat(0),
            designs: Vec::new(),
        }
    }
    pub fn add_design(&mut self, design: Design) {
        self.designs.push(design);
    }
    pub fn add_stem(&mut self, stem_index: usize) {
        self.stems[stem_index] += 1;
        for design in &self.designs {
            if self.stems.simd_lt(design.min_stems).any() {
                continue;
            }
            let mut grabbed_stems = self.stems.simd_min(design.max_stems);
            let grabbed_amount = grabbed_stems.reduce_sum();
            if grabbed_amount < design.total {
                continue;
            }
            let excess_stems = grabbed_stems - design.min_stems;
            let mut excess_amount = grabbed_amount - design.total;
            if excess_amount != 0 {
                for stem_index in 0..26 {
                    let stem_amount = excess_stems[stem_index];
                    if stem_amount == 0 {
                        continue;
                    }
                    let return_amount = u16::min(excess_amount, stem_amount);
                    excess_amount -= return_amount;
                    grabbed_stems[stem_index] -= return_amount;
                    if excess_amount == 0 {
                        break;
                    }
                }
            }
            self.stems -= grabbed_stems;
            let out = io::stdout();
            let mut handle = out.lock();
            write!(handle, "{}{}", design.name, design.size).unwrap();
            for stem_index in 0..26 {
                let amount = grabbed_stems[stem_index];
                if amount != 0 {
                    write!(handle, "{}{}", amount, stem_index_to_char(stem_index)).unwrap();
                }
            }
            write!(handle, "\n").unwrap();
            drop(handle);
            return;
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    production_lines: [ProductionLine; 2],
}

impl Warehouse {
    pub fn new() -> Self {
        Self {
            production_lines: [ProductionLine::new(), ProductionLine::new()],
        }
    }
    pub fn add_design(&mut self, design_str: &str) {
        let design = Design::from_str(design_str).unwrap();
        if design.total >= design.min_stems.reduce_sum() {
            // Only push possible designs
            self.production_lines[design.size.as_usize()].add_design(design);
        }
    }
    pub fn add_stem(&mut self, stem_str: &str) {
        let stem_index = char_to_stem_index(stem_str.chars().nth(0).unwrap());
        let size = Size::from_str(&stem_str[1..2]).unwrap();
        self.production_lines[size.as_usize()].add_stem(stem_index);
    }
}

fn main() {
    let mut stdin = io::stdin().lines();
    let lines = stdin.by_ref();
    let mut warehouse = Warehouse::new();
    for line in &mut *lines {
        if line.as_ref().unwrap().is_empty() {
            break;
        }
        warehouse.add_design(line.unwrap().as_str());
    }
    for line in &mut *lines {
        if line.as_ref().unwrap().is_empty() {
            break;
        }
        warehouse.add_stem(line.unwrap().as_str());
    }
}
