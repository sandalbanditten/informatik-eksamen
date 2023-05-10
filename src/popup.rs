#[derive(Debug, Default)]
/// struct for keeping track of popup state
pub struct PopupState {
    /// Bool for keeping track of add workout popup should be open
    add_workout: bool,

    /// Bool for keeping track of add exercixe popup should be open
    add_exercise: bool,

    /// Bool for keeping track of add set popup should be open
    add_set: bool,

    /// Title of the exercise
    exercise_title: String,

    /// bool for if set should have reps
    reps: bool,
    /// bool for if set should have weight
    weight: bool,
    /// bool for if set should have distance
    dist: bool,
}

impl PopupState {
    pub fn get_mut_exercise_title(&mut self) -> &mut String {
        &mut self.exercise_title
    }

    /// gets mutable reference to add_workout
    pub fn get_mut_add_workout(&mut self) -> &mut bool {
        &mut self.add_workout
    }

    /// gets mutable reference to add_exercise
    pub fn get_mut_add_exercise(&mut self) -> &mut bool {
        &mut self.add_exercise
    }

    /// gets mutable reference to add_set
    pub fn get_mut_add_set(&mut self) -> &mut bool {
        &mut self.add_set
    }

    /// Sets add_workout to the specified value
    pub fn mut_add_workout(&mut self, value: bool) {
        self.add_workout = value;
    }

    /// Sets add_exercise to the specified value
    pub fn mut_add_exercise(&mut self, value: bool) {
        self.add_exercise = value;
    }

    /// Sets add_set to the specified value
    pub fn mut_add_set(&mut self, value: bool) {
        self.add_set = value;
    }
}

impl PopupState {
    /// If internal bool: add_workout is true, display popup
    pub fn add_workout_popup(&mut self, ctx: &egui::Context /*, ui: &mut egui::Ui*/) {
        if self.add_workout {
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
                        self.add_workout = false;
                    }

                    ui.add_space(10.0);

                    //closes the popup
                    if ui.button("Cancel").clicked() {
                        self.add_workout = false;
                    }
                });
            });
        }
    }

    /// If internal bool: add_exercise is true, display popup
    pub fn add_exercise_popup(&mut self, ctx: &egui::Context) {
        if self.add_exercise {
            egui::Window::new("Add exercise")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Name of the exercise: ");
                    ui.text_edit_singleline(&mut self.exercise_title);

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
    }

    /// If internal bool: add_set is true, display popup
    pub fn add_set_popup (&mut self, ctx: &egui::Context) {
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
}
