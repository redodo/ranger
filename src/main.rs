#![feature(portable_simd)]
use std::default::Default;
use std::simd::{u16x32, SimdOrd, SimdPartialOrd, SimdUint};
use std::str::FromStr;
use std::{io, io::Write};

use lazy_static::lazy_static;
use regex::Regex;

mod size;

use size::{Size, SizeMap};

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
            .next()
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
        let mut unique_stem_count = 0;
        for stem_match in STEMS_RE.captures_iter(stems) {
            let stem_index = char_to_stem_index(
                stem_match
                    .name("species")
                    .unwrap()
                    .as_str()
                    .chars()
                    .next()
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
            unique_stem_count += 1;
        }

        // @Optimization - Minimize the maximum amount of stems.
        //
        // For example, given the design "AL10a5", it is obvious that the maximum
        // possible amount for 'a' would be '5', so this routine updates it to "AL5a5".
        //
        // This reduces the posibility of grabbing too many stems from the stock, which
        // costs precious time to put back.
        {
            let max_per_stem = 1 + total - unique_stem_count;
            for stem_max in max_stems.as_mut_array().iter_mut() {
                *stem_max = u16::min(*stem_max, max_per_stem);
            }
        }

        // @Optimize - Specify minimum possible amounts per stem.
        //
        // For example, given the design "AL5a5", the only possible bouquet that can be
        // created is "AL5a5".
        //
        // The minimum can be computed with the following formula:
        //
        //     stem_min = max(1, stem_max - sum(other_stems_max))
        //
        // Specifying a minimum amount required per species could allow for stopping
        // the design check early, or perhaps even disregard multiple designs altogether
        // with a SIMD operation.
        {
            let sum_max: u16 = max_stems.reduce_sum();
            for (stem_index, stem_max) in max_stems.as_array().iter().enumerate() {
                if *stem_max == 0 {
                    continue;
                }
                min_stems[stem_index] =
                    u16::max(1, stem_max - u16::min(*stem_max, sum_max - stem_max));
            }
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
    designs: [Option<Design>; 26],
    add_design_index: usize,
    designs_per_stem: [[usize; 26]; 26],
    max_per_stem: [u16; 26],
}
impl Default for ProductionLine {
    fn default() -> Self {
        Self {
            stems: u16x32::splat(0),
            designs: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            add_design_index: 0,
            designs_per_stem: [[usize::MAX; 26]; 26],
            max_per_stem: [0; 26],
        }
    }
}
impl ProductionLine {
    pub fn add_design(&mut self, design: Design) {
        for (stem_index, amount) in design.max_stems.as_array().iter().enumerate() {
            if *amount != 0 {
                self.max_per_stem[stem_index] = u16::max(self.max_per_stem[stem_index], *amount);
                for (insert_index, design_index) in
                    self.designs_per_stem[stem_index].iter().enumerate()
                {
                    if *design_index == usize::MAX {
                        self.designs_per_stem[stem_index][insert_index] = self.add_design_index;
                        break;
                    }
                }
            }
        }
        self.designs[self.add_design_index] = Some(design);
        self.add_design_index += 1;
    }
    pub fn preprocess(&mut self) {
        // @Optimization - Sort the designs per stem array by total amount of stems to
        // prioritize simpler designs.
        for designs in self.designs_per_stem.iter_mut() {
            designs.sort_by_key(|design_index| {
                if *design_index == usize::MAX {
                    return u16::MAX;
                }
                match &self.designs[*design_index] {
                    Some(design) => design.total,
                    None => u16::MAX,
                }
            });
        }
    }
    pub fn add_stem(&mut self, stem_index: usize) {
        self.stems[stem_index] += 1;
        if self.stems[stem_index] > self.max_per_stem[stem_index] {
            // @Optimization - If we already surpassed the max required numbers of stems
            // of this species for all designs, we can't make a new design this round.
            return;
        }
        for design_index in &self.designs_per_stem[stem_index] {
            if *design_index == usize::MAX {
                break;
            }
            let design_option = &self.designs[*design_index];
            let design = match design_option {
                Some(design) => design,
                None => break,
            };
            let mut grabbed_stems = self.stems.simd_min(design.max_stems);
            let grabbed_amount = grabbed_stems.reduce_sum();
            if grabbed_amount < design.total {
                continue;
            }
            if grabbed_stems.simd_lt(design.min_stems).any() {
                continue;
            }
            let mut excess_amount = grabbed_amount - design.total;
            if excess_amount != 0 {
                let excess_stems = grabbed_stems - design.min_stems;
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
            writeln!(handle).unwrap();
            drop(handle);
            return;
        }
    }
}

#[derive(Debug, Default)]
struct Warehouse {
    production_lines: SizeMap<ProductionLine>,
}

impl Warehouse {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_design(&mut self, design_str: &str) {
        let design = Design::from_str(design_str).unwrap();
        if design.total >= design.min_stems.reduce_sum() {
            // Only push possible designs
            self.production_lines
                .get_mut(&design.size)
                .add_design(design);
        }
    }
    pub fn preprocess(&mut self) {
        self.production_lines.get_mut(&Size::Small).preprocess();
        self.production_lines.get_mut(&Size::Large).preprocess();
    }
    pub fn add_stem(&mut self, stem_str: &str) {
        let stem_index = char_to_stem_index(stem_str.chars().next().unwrap());
        let size = Size::from_str(&stem_str[1..2]).unwrap();
        self.production_lines.get_mut(&size).add_stem(stem_index);
    }
}

fn main() {
    let mut stdin = io::stdin().lines();
    let lines = stdin.by_ref();
    let mut warehouse = Warehouse::new();
    for line in &mut *lines {
        let line_ref = line.as_ref().unwrap();
        if line_ref.is_empty() {
            break;
        }
        warehouse.add_design(line_ref.as_str());
    }
    warehouse.preprocess();
    for line in &mut *lines {
        let line_ref = line.as_ref().unwrap();
        if line_ref.is_empty() {
            break;
        }
        warehouse.add_stem(line_ref.as_str());
    }
}
