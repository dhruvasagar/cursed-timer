use std::fmt;
use std::num;
use std::ops::Add;
use std::str;
use std::time::Duration;

use chrono;
use chrono::prelude::*;
use csv;

use crate::{scramble::Scramble, timer::Timer};

#[derive(Clone)]
pub struct SolveTime(pub Duration);

impl str::FromStr for SolveTime {
    type Err = num::ParseIntError;
    // fmt: mm:ss.lll  (l = millisecond)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let minute: u64 = s[..2].parse()?;
        let second: u64 = s[3..5].parse()?;
        let millis: u64 = s[6..].parse()?;
        Ok(SolveTime(
            Duration::from_secs(minute * 60_u64 + second) + Duration::from_millis(millis),
        ))
    }
}

impl fmt::Display for SolveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}.{:0>3}",
            self.0.as_secs() / 60,
            self.0.as_secs() % 60,
            self.0.as_millis() % 1000
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Penalty {
    No,
    DNS,
    DNF,
    Time,
}

impl str::FromStr for Penalty {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "No" => Ok(Penalty::No),
            "DNS" => Ok(Penalty::DNS),
            "DNF" => Ok(Penalty::DNF),
            "Time" => Ok(Penalty::Time),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Penalty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Entry {
    pub time: SolveTime,
    scramble: Scramble,
    date: chrono::DateTime<Utc>,
    penalty: Penalty,
}

pub struct History {
    entries: Vec<Entry>,
    deleted: Vec<Entry>,
}

const VEC_START_SIZE: usize = 200;

impl History {
    pub fn from_csv(file_path: &str) -> History {
        let mut history = History {
            entries: Vec::with_capacity(VEC_START_SIZE),
            deleted: Vec::new(),
        };
        match csv::Reader::from_path(file_path) {
            Ok(mut reader) => {
                for result in reader.records() {
                    if let Ok(record) = result {
                        history.entries.push(Entry {
                            time: record[0].parse::<SolveTime>().unwrap(),
                            scramble: record[1].parse::<Scramble>().unwrap(),
                            date: record[2].parse::<chrono::DateTime<Utc>>().unwrap(),
                            penalty: record[3].parse::<Penalty>().unwrap(),
                        })
                    }
                }
            }
            Err(_) => {}
        };
        history
    }

    pub fn save_csv(&self, file_path: &str) {
        let mut writter = csv::Writer::from_path(file_path).unwrap();
        writter
            .write_record(&["time", "scramble", "date", "penalty"])
            .unwrap();
        for entry in &self.entries {
            writter
                .write_record(&[
                    entry.time.to_string(),
                    entry.scramble.to_string(),
                    entry.date.to_string(),
                    entry.penalty.to_string(),
                ])
                .unwrap();
        }
        writter.flush().unwrap();
    }

    pub fn summarize(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|Entry { time, penalty, .. }| match penalty {
                Penalty::No | Penalty::Time => time.to_string(),
                _ => penalty.to_string(),
            })
            .collect()
    }

    pub fn pop(&mut self) {
        if let Some(e) = self.entries.pop() {
            self.deleted.push(e);
        }
    }

    pub fn undo_pop(&mut self) {
        if let Some(e) = self.deleted.pop() {
            self.entries.push(e);
        }
    }

    pub fn push(&mut self, timer: &Timer, scramble: &Scramble, penalty: Penalty) {
        self.entries.push(Entry {
            time: SolveTime(timer.result),
            scramble: scramble.clone(),
            date: chrono::offset::Utc::now(),
            penalty,
        });
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn penalize(&mut self, index: usize, penalty: Penalty) {
        if let Some(last) = self.entries.get_mut(index) {
            if penalty == Penalty::Time {
                last.time = SolveTime(last.time.0.add(Duration::from_secs(2)));
            }
            last.penalty = penalty;
        }
    }

    pub fn penalize_last(&mut self, penalty: Penalty) {
        self.penalize(self.entries.len() - 1, penalty)
    }

    pub fn valid_entries(&self) -> Vec<&Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.penalty == Penalty::No || entry.penalty == Penalty::Time)
            .collect()
    }

    pub fn points(&self) -> (Vec<(f64, f64)>, [f64; 2], [f64; 2]) {
        let mut points: Vec<(f64, f64)> = vec![];
        let mut xs: Vec<f64> = vec![];
        let mut ys: Vec<f64> = vec![];
        for entry in self.valid_entries().iter() {
            let SolveTime(s) = entry.time;
            ys.push(s.as_secs_f64());
            xs.push(entry.date.timestamp() as f64);
            points.push((entry.date.timestamp() as f64, s.as_secs_f64()));
        }
        let xbounds: [f64; 2] = [
            xs.iter().copied().fold(f64::NAN, f64::min),
            xs.iter().copied().fold(f64::NAN, f64::max),
        ];
        let ybounds: [f64; 2] = [
            ys.iter().copied().fold(f64::NAN, f64::min),
            ys.iter().copied().fold(f64::NAN, f64::max),
        ];
        (points, xbounds, ybounds)
    }
}
