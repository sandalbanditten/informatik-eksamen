use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Context;

use crate::{deserialize_workouts, Workout};

/// The general state of our application
#[derive(Debug)]
pub struct App {
    /// The workouts
    ///
    /// Note: If two workouts are made at the same millisecond timestamp
    /// only one of them will be inserted.
    /// This is a highly improbably edge case.
    workouts: BTreeSet<Workout>,

    /// The data file to be read from and written to
    file: BufWriter<File>,
    
    add_workout_popup: bool,
}

impl App {
    /// Constructer, taking the file that will be read from and written to
    pub fn new(file: File) -> anyhow::Result<Self> {
        let workouts = deserialize_workouts(&file)?.into_iter().collect();
        dbg!(&file);
        let file = BufWriter::new(file);
        dbg!(&file);
        Ok(Self { workouts, file, add_workout_popup: false})
    }

    /// Saves workouts
    ///
    /// Returns the number of bytes read
    pub fn save_workouts(&mut self) -> anyhow::Result<usize> {
        dbg!(&self.file);
        let json = serde_json::to_string_pretty(&self.workouts.iter().collect::<Vec<_>>())
            .context("Failed to serialize workout data")?;

        let bytes_written = self
            .file
            .write(json.as_bytes())
            .context("Failed to write serialized workout data to file")?;

        self.file
            .flush()
            .context(format!("Failed to flush writer {:?}", self.file))?;

        // println!("Wrote {bytes_written} bytes to {:?}", dbg!(self.file));

        Ok(bytes_written)
    }

    pub fn push(&mut self, workout: Workout) {
        self.workouts.insert(workout);
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
            ui.vertical(|ui| {
                ui.heading("rST");
                ui.separator();

                // TODO: Some local buffer for saved workouts
                // than have not yet been written to the file
                // Perhaps a separate save button for the workout currently
                // being edited and one for all added workouts
                if ui.button("Add Workout").clicked() {
                    self.add_workout_popup = true;
                }
                
                if ui.button("Save workouts").clicked() {
                    // TODO: Handle error
                    let bytes_written = self.save_workouts().unwrap();
                }

                // TODO: UI for adding a workout

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

        if self.add_workout_popup {
            egui::Window::new("Add Workout")
                .collapsible(false)
                .movable(false)
                .show(ctx, |ui| {

                    ui.horizontal(|ui| {
                        if ui.button("add set").clicked() {

                        }
                    });
            });
        }
    }

    // et eller andet lort
}
