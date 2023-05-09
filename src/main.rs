#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A workout tracking program

// TODO: Logging of some kind
// TODO: Add some tests
// TODO: Better error handling

use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Write};
use std::path::Path;

use eframe::egui;

use workout::Workout;

mod workout;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(375.0, 812.0)),
        ..Default::default()
    };

    // TODO: Implement App::new and data serialization
    // Make (de)serialization methods on App
    let data_file = create_or_open();
    let app = App::new(data_file);

    eframe::run_native("rST", options, Box::new(|_| Box::new(app)))
}

fn deserialize_workouts(file: &File) -> Vec<Workout> {
    let data_file = BufReader::new(file);

    // TODO: Better error handling
    serde_json::from_reader(data_file).expect("Error in deserializing data from data file")
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
#[derive(Debug)]
struct App {
    /// The workouts
    ///
    /// Note: If two workouts are made at the same millisecond timestamp
    /// only one of them will be inserted.
    /// This is a highly improbably edge case.
    workouts: BTreeSet<Workout>,

    /// The data file to be read from and written to
    file: File,
}

impl App {
    pub fn new(file: File) -> Self {
        Self {
            workouts: deserialize_workouts(&file).into_iter().collect(),
            file,
        }
    }

    pub fn save_workouts(&mut self) -> io::Result<()> {
        let mut writer = BufWriter::new(&self.file);
        let json = serde_json::to_string(&self.workouts.iter().collect::<Vec<_>>())?;
        let bytes_written = writer.write(json.as_bytes())?;

        Ok(())
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

    // et eller andet lort
}
