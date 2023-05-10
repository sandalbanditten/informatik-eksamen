#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! A workout tracking program

// TODO: Add some tests

use std::env;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use anyhow::Context;
use eframe::egui;

use app::App;
use workout::Workout;

mod app;
mod workout;

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(375.0, 812.0)),
        ..Default::default()
    };

    let data_file = create_or_open()?;
    let data_file = data_file;
    let app = App::new(data_file)?;

    // TODO: Why doesn't this work with anyhow?
    eframe::run_native("rST", options, Box::new(|_| Box::new(app))).expect("Error in egui");

    Ok(())
}

fn deserialize_workouts(file: &File) -> anyhow::Result<Vec<Workout>> {
    let data_file = BufReader::new(file);

    // TODO: Better error handling
    let vec = match serde_json::from_reader(data_file)
        .context("Error in deserializing data from data file")
    {
        // If there is an error reading the data, just use no data
        Ok(vec) => vec,
        Err(_) => Vec::new(),
    };

    Ok(vec)
}

// TODO: Conditional file depending on DOS/UNIX
// Currently it just works on unix
#[cfg(unix)]
fn create_or_open() -> anyhow::Result<File> {
    use std::fs::OpenOptions;

    let home =
        env::var("HOME").context("$HOME environment variable not set\nNo storage file created")?;
    let path = format!("{home}/.rst");
    let path = Path::new(&path);

    match OpenOptions::new().write(true).read(true).open(path) {
        Ok(file) => Ok(file),
        Err(err) if err.kind() == ErrorKind::NotFound => File::create(path).context(format!(
            "Unable to create data file: {:?}\nNo user data read",
            path
        )),
        Err(err) => Err(err.into()),
    }
}
