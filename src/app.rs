use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Seek, Write};

use anyhow::Context;

use crate::popup::PopupState;
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

    /// When true, opens a popup for adding workouts
    popup_state: PopupState,
}

impl App {
    /// Constructer, taking the file that will be read from and written to
    pub fn new(file: File) -> anyhow::Result<Self> {
        let workouts = deserialize_workouts(&file).into_iter().collect();
        let file = BufWriter::new(file);
        Ok(Self {
            workouts,
            file,
            popup_state: PopupState::default(),
        })
    }

    /// Saves workouts
    ///
    /// Returns the number of bytes written
    pub fn save_workouts(&mut self) -> anyhow::Result<usize> {
        let json = serde_json::to_string_pretty(&self.workouts.iter().collect::<Vec<_>>())
            .context("Failed to serialize workout data")?;

        // Truncate the file to zero, before rewriting data
        self.file
            .get_mut()
            .set_len(0)
            .context(format!("Failed to truncate {:?} to length 0", self.file))?;

        let _ = self
            .file
            .get_mut()
            .rewind()
            .context(format!("Failed to set cursor to 0 on file {:?}", self.file))?;

        let bytes_written = self
            .file
            .write(json.as_bytes())
            .context("Failed to write serialized workout data to file")?;

        self.file
            .flush()
            .context(format!("Failed to flush writer {:?}", self.file))?;

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
                    self.popup_state.mut_add_workout(true);
                }

                if ui.button("Save workouts").clicked() {
                    // TODO: Handle error
                    let _bytes_written = self.save_workouts().unwrap();
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

        self.popup_state.add_workout_popup(ctx);
        self.popup_state.add_exercise_popup(ctx);
        self.popup_state.add_set_popup(ctx);
    }

    // et eller andet lort
}
