#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A workout tracking program

// TODO: Logging of some kind

use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::time::Duration;
use std::{collections::BTreeSet, env};

use eframe::egui;

use workout::{Set, Workout};

mod workout;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(375.0, 812.0)),
        ..Default::default()
    };

    let _data_file = create_or_open();

    let mut app = App::default();

    // Two test workouts
    let mut test_workout = Workout::new();
    let test_set = Set::new(Some(10), Some(5.5), None, "Lateral raises");
    test_workout.push(test_set);
    let test_set = Set::new(None, Some(25.0), Some(5.0), "Trail running");
    test_workout.push(test_set);
    app.workouts.insert(test_workout);

    std::thread::sleep(Duration::from_millis(10));

    let mut test_workout = Workout::new();
    let test_set = Set::new(Some(15), Some(7.5), None, "Lateral raises");
    test_workout.push(test_set);
    let test_set = Set::new(None, Some(27.0), Some(6.3), "Trail running");
    test_workout.push(test_set);
    app.workouts.insert(test_workout);

    eframe::run_native("rST", options, Box::new(|_| Box::new(app)))
}

// TODO: Conditional file depending on DOS/UNIX
// Currently it just works on unix
#[cfg(unix)]
fn create_or_open() -> File {
    let home = env::var("HOME").expect("$HOME must be set to use this program");
    let path = format!("{home}/.rst");
    let path = Path::new(&path);

    match File::open(path) {
        Ok(file) => {
            println!("Opening data file: {:?}", path);
            file
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("Creating data file: {:?}", path);
            File::create(path).expect("Unable to create data file")
        }
        Err(_) => panic!("Error opening the data file"),
    }
}

// TODO: Put app in a seperate module

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
        let mut style = (*ctx.style()).clone();

        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = 20.0 // whatever size you want here
        }

        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("rST");
                ui.separator();

                // Workouts
                ui.heading("Workouts:");
                // Iterator is reversed to show more recent workouts
                // (with bigger timestamps) before earlier workouts
                for (i, workout) in self.workouts.iter().rev().enumerate() {
                    ui.label(format!("Workout {}:", i + 1));
                    ui.label(format!("{workout}"));
                }
            });
        });
    }
}
