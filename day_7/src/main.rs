// https://adventofcode.com/2022/day/7

use std::collections::{hash_map, HashMap};

fn main() {
    let mut fs = Fs::new();
    for line in include_str!("../input").lines() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            fs.cd(dir);
        } else if line == "$ ls" || line.starts_with("dir ") {
            // Ignore "$ ls" and "dir ..." lines
        } else {
            let size = line
                .rfind(' ')
                .and_then(|i| line[..i].parse().ok())
                .expect("invalid file");
            fs.add_size_at_path(size);
        }
    }

    let max_size = 100_000;
    let part_1: u64 = fs.sizes().filter(|x| *x < &max_size).sum();
    assert_eq!(1845346, part_1);
    println!("Part 1: {}", part_1);

    let total_disk_space = 70_000_000;
    let required_disk_space = 30_000_000;
    let used_space = fs.root_size();
    let free_space = total_disk_space - used_space;
    let to_free = required_disk_space - free_space;
    let part_2 = fs.sizes().filter(|s| *s > &to_free).min().expect("part 2");
    assert_eq!(3636703, *part_2);
    println!("Part 2: {}", part_2);
}

struct Fs {
    fs: HashMap<Vec<&'static str>, u64>,
    path: Vec<&'static str>,
}

impl Fs {
    fn new() -> Self {
        Self {
            fs: Default::default(),
            path: Vec::new(),
        }
    }

    fn cd(&mut self, dir: &'static str) {
        if dir == ".." {
            self.path.pop();
        } else {
            self.path.push(dir);
        }
    }

    fn add_size_at_path(&mut self, size: u64) {
        let mut path = self.path.clone();
        while !path.is_empty() {
            if self.fs.contains_key(&path) {
                if let Some(s) = self.fs.get_mut(&path) {
                    *s += size;
                }
            } else {
                self.fs.insert(path.clone(), size);
            }
            path.pop();
        }
    }

    fn sizes(&self) -> hash_map::Values<Vec<&str>, u64> {
        self.fs.values()
    }

    fn root_size(&self) -> u64 {
        *self.fs.get(&vec!["/"]).expect("no root size")
    }
}
