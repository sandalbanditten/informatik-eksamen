#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs // TODO: Turn this on at some moment
)]

use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

/// A single set in a workout struct
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Set {
    /// The number of repetitions in this workout
    reps: Option<usize>,

    /// The weight used in this workout, in kilograms
    wght: Option<f64>,

    /// The distance of the workout, in meters
    dist: Option<f64>,

    /// The type of workout i.e. "Running" or "Lateral raises"
    kind: String,
}

impl Set {
    /// Constructor
    ///
    /// Automatically inserts the unix timestamp
    pub fn new(reps: Option<usize>, wght: Option<f64>, dist: Option<f64>, kind: &str) -> Self {
        Self {
            reps,
            wght,
            dist,
            kind: kind.to_owned(),
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new(None, None, None, "")
    }
}

impl Eq for Set {}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reps = self
            .reps
            .map_or("No reps\n".to_owned(), |v| format!("Repetitions: {v}\n"));
        let wght = self
            .wght
            .map_or("No weight\n".to_owned(), |v| format!("Weight: {v}\n"));
        let dist = self
            .dist
            .map_or("No distance\n".to_owned(), |v| format!("Distance: {v}\n"));

        f.write_str(&reps)?;
        f.write_str(&wght)?;
        f.write_str(&dist)?;

        Ok(())
    }
}

/// A workout containing many sets
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Workout {
    /// The sets
    sets: Vec<Set>,

    /// The unix timestamp in milliseconds
    timestamp: u128,
}

impl Workout {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        Self {
            sets: Vec::new(),
            timestamp,
        }
    }

    pub fn push(&mut self, set: Set) {
        self.sets.push(set);
    }
}

impl Display for Workout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sets:\n")?;
        for (i, set) in self.sets.iter().enumerate() {
            f.write_str(&format!("Set {i}:\n{set}"))?;
        }

        Ok(())
    }
}

impl Eq for Workout {}

impl Ord for Workout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
