use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Seek, Write};

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

    /// When true, opens a popup for adding workouts
    add_workout_popup: bool,
    add_exercise: bool,
    add_set: bool
}

impl App {
    /// Constructer, taking the file that will be read from and written to
    pub fn new(file: File) -> anyhow::Result<Self> {
        let workouts = deserialize_workouts(&file).into_iter().collect();
        let file = BufWriter::new(file);
        Ok(Self {
            workouts,
            file,
            add_workout_popup: false,
            add_exercise: false,
            add_set: false
        })
    }

    /// Saves workouts
    ///
    /// Returns the number of bytes read
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
            .seek(std::io::SeekFrom::Start(0))
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
                .show(ctx, |ui| {
                    if ui.button("Add exercise").clicked() {
                        self.add_exercise = true;
                    }                    

                    // the save and cancel buttons at the bottom of popup:
                    ui.horizontal(|ui| {
                        //currently does nothing but close the popup
                        if ui.button("Save").clicked() {
                            self.add_workout_popup = false;
                        }

                        ui.add_space(10.0);

                        //closes the popup
                        if ui.button("Cancel").clicked() {
                            self.add_workout_popup = false;
                        }
                    })
                    
            });  
        }

        if self.add_exercise {
            egui::Window::new("Add exercise")
            .collapsible(false)
            .show(ctx, |ui| {
                if ui.button("Add set").clicked() {
                    self.add_set = true;
                }                    

                // the save and cancel buttons at the bottom of popup:
                ui.horizontal(|ui| {
                    //currently does nothing but close the popup
                    if ui.button("Save").clicked() {
                        self.add_exercise = false;
                    }

                    ui.add_space(10.0);

                    //closes the popup
                    if ui.button("Cancel").clicked() {
                        self.add_exercise = false;
                    }
                })
            });
        }

        if self.add_set {
            egui::Window::new("Add set")
            .collapsible(false)
            .show(ctx, |ui| {
                

                // the save and cancel buttons at the bottom of popup:
                ui.horizontal(|ui| {
                    //currently does nothing but close the popup
                    if ui.button("Save").clicked() {
                        self.add_set = false;
                    }

                    ui.add_space(10.0);

                    //closes the popup
                    if ui.button("Cancel").clicked() {
                        self.add_set = false;
                    }
                })
            });
        }
    }

    // et eller andet lort
}
