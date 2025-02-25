use std::fs;

fn main() {
    let contents = fs::read_to_string("./day02/input.txt").expect("read file");

    let mut reports: Vec<Report> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        reports.push(Report::from(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("parse"))
                .collect::<Vec<i32>>(),
        ));
    }

    println!(
        "safe count: {}",
        reports.iter().filter(|r| r.safe()).count()
    );

    println!(
        "dampened safe count: {}",
        reports.iter().filter(|r| r.dampened_safe()).count()
    );
}

struct Report(Vec<i32>);

impl From<Vec<i32>> for Report {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}

impl Report {
    fn safe(&self) -> bool {
        let mut prev = None;
        let mut dir = 0;
        for val in self.0.iter() {
            if let Some(p) = prev {
                let delta: i32 = p - *val;
                let dist = delta.abs();
                let d = if delta > 0 {
                    1
                } else if delta == 0 {
                    0
                } else {
                    -1
                };

                if d == 0 {
                    // No change
                    return false;
                }
                if dir == 0 {
                    dir = d;
                } else if dir != d {
                    // Inconsistent direction
                    return false;
                }

                if dist > 3 {
                    // Too big of a change
                    return false;
                }
            }
            prev = Some(*val);
        }
        true
    }

    fn dampened_safe(&self) -> bool {
        let mut dampened_vals = self.0.clone()[1..].to_vec();

        for i in 0..self.0.len() {
            if i > 0 {
                dampened_vals[i - 1] = self.0[i - 1];
            }
            if Report(dampened_vals.clone()).safe() {
                return true;
            }
        }

        false
    }
}
