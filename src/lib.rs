use std::{sync::atomic::{AtomicU64, self}};
use chrono::{DateTime, Local, TimeZone};
use egui::{self, Ui};
use serde::{Serialize, Deserialize};


// Define a global atomic counter for unique task IDs
static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
    date_added: DateTime<Local>,
    deadline: Option<DateTime<Local>>,
    #[serde(skip)]
    show_options: bool,
    #[serde(skip)]
    deadline_input: String, // Add this field to capture the user's deadline input
}

impl Task {
    fn set_deadline(&mut self, deadline: DateTime<Local>) {
        self.deadline = Some(deadline);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    name: String,
    tasks: Vec<Task>,
    #[serde(default)]
    new_task: String, // Ensure this field is handled correctly during deserialization
    #[serde(skip)]
    show_options: bool, // Track whether the list options are shown
    #[serde(skip)]
    display_id: bool, // Toggle display of the id field
    #[serde(skip)]
    display_id_title: bool, // Toggle display of the id title
    #[serde(skip)]
    display_name: bool, // Toggle display of the name field
    #[serde(skip)]
    display_name_title: bool, // Toggle display of the name title
    // Add more display toggles as needed
}

impl List {
    fn add_task(&mut self) {
        let task = Task {
            task: self.new_task.clone(),
            done_status: false,
            id: UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst),
            date_added: Local::now(),
            deadline: None,
            show_options: false,
            deadline_input: String::new(), // Initialize the deadline_input as an empty string
        };
        self.tasks.push(task);
        self.new_task.clear(); // Clear the input after adding the task
    }
}

#[derive(Debug)]
pub struct ListApp {
    lists: Vec<List>,
    new_list: String, // Add this field to persist list input
}

impl ListApp {
    pub fn new() -> Self {
        ListApp { 
            lists: Vec::new(),
            new_list: String::new(), // Initialize the new_list field
        }
    }

    fn add_list(&mut self, name: &str) {
        self.lists.push(List {
            name: name.to_string(),
            tasks: Vec::new(),
            new_task: String::new(), // Initialize new_task as an empty String
            show_options: false,
            display_id: true, // Show id by default
            display_id_title: true, // Show id title by default
            display_name: true, // Show name by default
            display_name_title: true, // Show name title by default
            // Initialize other fields as needed
        });
        self.new_list.clear(); // Clear the input after adding the list
    }

    fn save_to_json(&self, file_path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.lists)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }

    fn load_from_json(&mut self, file_path: &str) -> std::io::Result<()> {
        let json = std::fs::read_to_string(file_path)?;
        self.lists = serde_json::from_str(&json)?;
        Ok(())
    }
}

pub fn run_gui(ui: &mut Ui, app: &mut ListApp) {
    ui.heading("Title");

    // List creation section
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.new_list);

        // Clone the new_list string to avoid borrowing issues
        let new_list_clone = app.new_list.clone();

        if ui.button("Create List").clicked() && !new_list_clone.is_empty() {
            app.add_list(&new_list_clone); // Use the cloned string
        }
    });

    ui.separator();

    // Collect indices of lists to be deleted
    let mut lists_to_remove = Vec::new();

    // Display each list and its tasks
    for (list_index, list) in app.lists.iter_mut().enumerate() {
        ui.group(|ui| {
            // First horizontal layout for the list heading
            ui.horizontal(|ui| {
                ui.heading(&list.name);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Options").clicked() {
                        list.show_options = !list.show_options;
                    }
                });
            });

            // Second horizontal layout for the options if toggled on
            if list.show_options {
                                    
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Delete List").clicked() {
                            lists_to_remove.push(list_index); // Mark this list for removal
                        }

                        // Reversing the order here so they appear in the intended left-to-right order when aligned to the right
                        ui.checkbox(&mut list.display_name, "Name");
                        ui.checkbox(&mut list.display_name_title, "Name Title");
                        ui.checkbox(&mut list.display_id, "ID");
                        ui.checkbox(&mut list.display_id_title, "ID Title");

                        ui.label("Display Options:");
                    });
                });
            }

            // Task input section
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut list.new_task);

                if ui.button("+ Task").clicked() && !list.new_task.is_empty() {
                    list.add_task();
                }
            });

            ui.separator();

            // Collect indices of tasks to be deleted
            let mut tasks_to_remove = Vec::new();

            // Display existing tasks
            for (i, task) in list.tasks.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    // Use a checkbox instead of a toggle
                    ui.checkbox(&mut task.done_status, "");

                    // Display "Complete" in green if the task is done
                    if task.done_status {
                        ui.label(egui::RichText::new("Complete").color(egui::Color32::GREEN));
                    }

                    // Conditional display of fields
                    if list.display_id && list.display_id_title {
                        ui.label("ID:");
                    }
                    if list.display_id {
                        ui.label(format!("{}", task.id));
                    }
                    if list.display_name && list.display_name_title {
                        ui.label("Name:");
                    }
                    if list.display_name {
                        ui.label(&task.task);
                    }

                    ui.label(format!("Added on: {}", task.date_added.format("%Y-%m-%d %H:%M:%S")));

                    if let Some(deadline) = task.deadline {
                        ui.label(format!("Deadline: {}", deadline.format("%Y-%m-%d %H:%M:%S")));
                    } else {
                        ui.label("No deadline set");
                    }

                    // Toggle options visibility
                    if ui.button("Options").clicked() {
                        task.show_options = !task.show_options;
                    }
                });

                // Display options if toggled on
                if task.show_options {
                    ui.horizontal(|ui| {
                        ui.label("Rename:");
                        ui.text_edit_singleline(&mut task.task);

                        ui.label("Set Deadline (YYYY-MM-DD HH:MM):");
                        ui.text_edit_singleline(&mut task.deadline_input);

                        if ui.button("Set Deadline").clicked() {
                            if let Ok(naive_deadline) = chrono::NaiveDateTime::parse_from_str(&task.deadline_input, "%Y-%m-%d %H:%M") {
                                if let Some(parsed_deadline) = Local.from_local_datetime(&naive_deadline).single() {
                                    task.set_deadline(parsed_deadline);
                                } else {
                                    ui.label("Invalid date/time, cannot determine the timezone.");
                                }
                            } else {
                                ui.label("Invalid date format. Use YYYY-MM-DD HH:MM.");
                            }
                        }

                        if ui.button("Delete").clicked() {
                            tasks_to_remove.push(i); // Mark this task for removal
                        }
                    });
                }
                            }

            // Remove tasks after the loop
            for &index in tasks_to_remove.iter().rev() {
                list.tasks.remove(index);
            }
        });
        ui.separator();
    }


    // Remove lists after the loop
    for &index in lists_to_remove.iter().rev() {
        app.lists.remove(index);
    }

    // Save and Load buttons
    ui.horizontal(|ui| {
        if ui.button("Save Lists").clicked() {
            app.save_to_json("todo_lists.json").unwrap();
        }

        if ui.button("Load Lists").clicked() {
            app.load_from_json("todo_lists.json").unwrap();
        }
    });
}