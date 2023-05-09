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
    weight: Option<f64>,

    /// The distance of the workout, in meters
    distance: Option<f64>,

    /// The type of workout i.e. "Running" or "Lateral raises"
    kind: String,
}

impl Set {
    /// Constructor
    ///
    /// Automatically inserts the unix timestamp
    pub fn new(
        reps: Option<usize>,
        weight: Option<f64>,
        distance: Option<f64>,
        kind: &str,
    ) -> Self {
        Self {
            reps,
            weight,
            distance,
            kind: kind.to_owned(),
        }
    }

    /// Reps getter
    pub fn reps(&self) -> Option<usize> {
        self.reps
    }

    /// Weight getter
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }

    /// Distance getter
    pub fn distance(&self) -> Option<f64> {
        self.distance
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
        let reps = self.reps.map_or("No reps".to_owned(), |v| format!("{v}"));
        let weight = self
            .weight
            .map_or("No weight".to_owned(), |v| format!("{v}"));
        let distance = self
            .distance
            .map_or("No distance".to_owned(), |v| format!("{v}"));
        f.write_str("bruh")
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
        f.write_str("dawg")
    }
}

impl Default for Workout {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for Workout {}

impl Ord for Workout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
