use crate::history::{Entry, SolveTime};
use std::ops::{Add, Div};
use std::time::Duration;

const SIZES: [u32; 4] = [5, 12, 50, 100];

#[derive(Debug)]
pub struct Average {
    size: u32,
    best: Duration,
    current: Duration,
    batch: Vec<Duration>,
}

pub fn stats(entries: Vec<&Entry>) -> Vec<Vec<String>> {
    let mut best: Duration = Duration::from_secs(0);
    let mut total: Duration = Duration::from_secs(0);

    if entries.is_empty() {
        return vec![];
    }

    let mut avgs = vec![];
    for size in SIZES {
        avgs.push(Average {
            size,
            best: Duration::from_secs(0),
            current: Duration::from_secs(0),
            batch: vec![],
        });
    }

    for entry in entries.iter() {
        let SolveTime(d) = entry.time;
        if best == Duration::from_secs(0) || best > d {
            best = d;
        }
        total = total.add(d);

        for avg in avgs.iter_mut() {
            avg.batch.push(d);
            if avg.batch.len() == avg.size as usize {
                let st = avg
                    .batch
                    .iter()
                    .fold(Duration::from_secs(0), |a, &l| a.add(l));
                let average = st.div(avg.size);
                avg.current = average;
                if avg.best == Duration::from_secs(0) || average < avg.best {
                    avg.best = average;
                }
                avg.batch.remove(0);
            }
        }
    }

    let SolveTime(current) = entries.last().unwrap().time;
    let mut result = vec![vec![
        String::from("Time"),
        format!("{:?}", current),
        format!("{:?}", best),
    ]];
    for avg in avgs.iter() {
        result.push(vec![
            format!("Ao{}", avg.size),
            format!("{:?}", avg.current),
            format!("{:?}", avg.best),
        ]);
    }
    result.push(vec![]);
    result.push(vec![
        String::from("Average"),
        String::from(""),
        format!("{:?}", total.div(entries.len() as u32)),
    ]);
    result
}
