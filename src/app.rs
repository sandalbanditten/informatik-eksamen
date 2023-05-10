use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Seek, Write};

use anyhow::Context;

use crate::{deserialize_workouts, Workout};

#[derive(Debug, Default)]
/// struct for keeping track of popup state
struct PopupState {
    add_workout: bool,
    add_exercise: bool,
    add_set: bool,
}

impl PopupState {
    /// gets mutable reference to add_workout
    fn get_mut_add_workout(&mut self) -> &mut bool {
        &mut self.add_workout
    }

    /// gets mutable reference to add_exercise
    fn get_mut_add_exercise(&mut self) -> &mut bool {
        &mut self.add_exercise
    }

    /// gets mutable reference to add_set
    fn get_mut_add_set(&mut self) -> &mut bool {
        &mut self.add_set
    }

    /// Sets add_workout to the specified value
    fn mut_add_workout(&mut self, value: bool) {
        self.add_workout = value;
    }

    /// Sets add_exercise to the specified value
    fn mut_add_exercise(&mut self, value: bool) {
        self.add_exercise = value;
    }

    /// Sets add_set to the specified value
    fn mut_add_set(&mut self, value: bool) {
        self.add_set = value;
    }
}

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

        if *self.popup_state.get_mut_add_workout() {
            egui::Window::new("Add Workout")
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("Add exercise").clicked() {
                        self.popup_state.mut_add_exercise(true);
                    }

                    // the save and cancel buttons at the bottom of popup:
                    ui.horizontal(|ui| {
                        //currently does nothing but close the popup
                        if ui.button("Save").clicked() {
                            self.popup_state.mut_add_workout(false);
                        }

                        ui.add_space(10.0);

                        //closes the popup
                        if ui.button("Cancel").clicked() {
                            self.popup_state.mut_add_workout(false);
                        }
                    })
                });
        }

        if *self.popup_state.get_mut_add_exercise() {
            egui::Window::new("Add exercise")
                .collapsible(false)
                .show(ctx, |ui| {
                    if ui.button("Add set").clicked() {
                        self.popup_state.mut_add_set(true);
                    }

                    // the save and cancel buttons at the bottom of popup:
                    ui.horizontal(|ui| {
                        //currently does nothing but close the popup
                        if ui.button("Save").clicked() {
                            self.popup_state.mut_add_exercise(false);
                        }

                        ui.add_space(10.0);

                        //closes the popup
                        if ui.button("Cancel").clicked() {
                            self.popup_state.mut_add_exercise(false);
                        }
                    })
                });
        }

        if *self.popup_state.get_mut_add_set() {
            egui::Window::new("Add set")
                .collapsible(false)
                .show(ctx, |ui| {
                    // the save and cancel buttons at the bottom of popup:
                    ui.horizontal(|ui| {
                        //currently does nothing but close the popup
                        if ui.button("Save").clicked() {
                            self.popup_state.mut_add_set(false);
                        }

                        ui.add_space(10.0);

                        //closes the popup
                        if ui.button("Cancel").clicked() {
                            self.popup_state.mut_add_set(false);
                        }
                    })
                });
        }
    }

    // et eller andet lort
}
