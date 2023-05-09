#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs // TODO: Turn this on at some moment
)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BTreeSet;

use eframe::egui;

use workout::{Set, Workout};

mod workout;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(360.0, 800.0)),
        ..Default::default()
    };
    let mut app = App::default();
    let test_set = Set::new(Some(10), Some(5.5), None, "Lateral raises");
    let mut test_workout = Workout::new();
    test_workout.push(test_set);
    app.workouts.insert(test_workout);
    eframe::run_native("rST", options, Box::new(|_| Box::new(app)))
}

/// The general state of our application
#[derive(Debug, Clone)]
struct App {
    /// The workouts
    ///
    /// Note: If two workouts are made at the same millisecond timestamp
    /// only one of them will be inserted.
    /// This is a highly improbably edge case.
    workouts: BTreeSet<Workout>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            workouts: BTreeSet::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rST");
            ui.separator();

            // Workouts
            ui.heading("Workouts:");
            // Iterator is reversed to show more recent workouts
            // (with bigger timestamps) before earlier workouts
            for (i, workout) in self.workouts.iter().rev().enumerate() {
                ui.label(format!("Workout {i}:"));
                ui.label(format!("{}", workout));
            }
        });
    }
}
