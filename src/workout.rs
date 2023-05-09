use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
/// A workout struct
pub struct Workout {
    /// The number of repetitions in this workout
    reps: usize,
    /// The weight used in this workout, optional
    weight: Option<usize>,
    /// The unix timestamp in milliseconds
    timestamp: u128,
}

impl Workout {
    /// Constructor
    ///
    /// Automatically inserts the unix timestamp
    pub fn new(reps: usize, weight: Option<usize>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        Self {
            reps,
            weight,
            timestamp,
        }
    }

    /// Reps getter
    pub fn reps(&self) -> usize {
        self.reps
    }

    /// Weight getter
    pub fn weight(&self) -> Option<usize> {
        self.weight
    }
}

impl Default for Workout {
    fn default() -> Self {
        Workout::new(0, None)
    }
}

impl Ord for Workout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
