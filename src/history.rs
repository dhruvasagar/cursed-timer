use std::fmt;
use std::num;
use std::ops::{Add, Div};
use std::str;
use std::time::Duration;

use chrono;
use chrono::prelude::*;
use csv;

use crate::{scramble::Scramble, timer::Timer};

pub struct SolveTime(Duration);

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

struct Entry {
    time: SolveTime,
    scramble: Scramble,
    date: chrono::DateTime<Utc>,
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
        let mut reader = csv::Reader::from_path(file_path).unwrap();
        for result in reader.records() {
            if let Ok(record) = result {
                history.entries.push(Entry {
                    time: record[0].parse::<SolveTime>().unwrap(),
                    scramble: record[1].parse::<Scramble>().unwrap(),
                    date: record[2].parse::<chrono::DateTime<Utc>>().unwrap(),
                })
            }
        }
        history
    }

    pub fn save_csv(&self, file_path: &str) {
        let mut writter = csv::Writer::from_path(file_path).unwrap();
        writter.write_record(&["time", "scramble", "date"]).unwrap();
        for entry in &self.entries {
            writter
                .write_record(&[
                    entry.time.to_string(),
                    entry.scramble.to_string(),
                    entry.date.to_string(),
                ])
                .unwrap();
        }
        writter.flush().unwrap();
    }

    pub fn summarize(&self, n: usize) -> Vec<String> {
        self.entries
            .iter()
            .skip(self.entries.len() - n)
            .map(|Entry { time, .. }| time.to_string())
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

    pub fn push(&mut self, timer: &Timer, scramble: &Scramble) {
        self.entries.push(Entry {
            time: SolveTime(timer.result),
            scramble: scramble.clone(),
            date: chrono::offset::Utc::now(),
        });
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn stats(&self) -> Vec<Vec<String>> {
        let size = self.len() as u32;
        if size == 0 {
            return vec![];
        }

        let mut best: Duration = Duration::from_secs(0);
        let mut tot: Duration = Duration::from_secs(0);

        let mut ao5: Duration = Duration::from_secs(0);
        let mut cao5: Duration = Duration::from_secs(0);
        let mut lao5: Vec<Duration> = vec![];

        let mut ao12: Duration = Duration::from_secs(0);
        let mut cao12: Duration = Duration::from_secs(0);
        let mut lao12: Vec<Duration> = vec![];
        // let mut ao50 = 0;
        // let mut ao100 = 0;
        let SolveTime(last) = self.entries.last().unwrap().time;
        for entry in self.entries.iter() {
            let SolveTime(d) = entry.time;
            if best == Duration::from_secs(0) || best > d {
                best = d;
            }
            tot = tot.add(d);
            lao5.push(d);
            if lao5.len() == 5 {
                let sd = lao5.iter().fold(Duration::from_secs(0), |a, &l| a.add(l));
                cao5 = sd.div(5);
                if ao5 == Duration::from_secs(0) || sd.div(5) < ao5 {
                    ao5 = sd.div(5);
                }
                lao5.remove(0);
            }
            lao12.push(d);
            if lao12.len() == 12 {
                let sd = lao12.iter().fold(Duration::from_secs(0), |a, &l| a.add(l));
                cao12 = sd.div(12);
                if ao12 == Duration::from_secs(0) || sd.div(12) < ao12 {
                    ao12 = sd.div(12);
                }
                lao12.remove(0);
            }
        }
        vec![
            vec![
                String::from("Time"),
                format!("{:?}", last),
                format!("{:?}", best),
            ],
            vec![
                String::from("Ao5"),
                format!("{:?}", cao5),
                format!("{:?}", ao5),
            ],
            vec![
                String::from("Ao12"),
                format!("{:?}", cao12),
                format!("{:?}", ao12),
            ],
            vec![],
            vec![
                String::from("Average"),
                String::from(""),
                format!("{:?}", tot.div(size)),
            ],
        ]
    }
}
