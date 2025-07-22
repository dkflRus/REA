use eframe::egui;
use chrono::{Datelike, Duration, Local, NaiveDate};

// Define the CalendarApp struct to hold the app's state
struct CalendarApp {
    displayed_month: u32,
    displayed_year: i32,
}

impl CalendarApp {
    // Constructor to initialize with the current month and year
    fn new() -> Self {
        let now = Local::now();
        Self {
            displayed_month: now.month(),
            displayed_year: now.year(),
        }
    }

    // Helper function to get the month name from a number
    fn month_name(&self) -> &'static str {
        match self.displayed_month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Invalid",
        }
    }
}

// Implement the eframe::App trait for the GUI
impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Calculate calendar data
            let first_day = NaiveDate::from_ymd_opt(
                self.displayed_year,
                self.displayed_month,
                1
            ).unwrap();
            let offset = first_day.weekday().num_days_from_monday();
            let next_month_first = if self.displayed_month == 12 {
                NaiveDate::from_ymd_opt(self.displayed_year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(self.displayed_year, self.displayed_month + 1, 1).unwrap()
            };
            let last_day = next_month_first - Duration::days(1);
            let days_in_month = last_day.day();
            let total_cells = offset + days_in_month;
            let number_of_rows = (total_cells + 6) / 7;

            // Navigation and month/year display
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    if self.displayed_month == 1 {
                        self.displayed_month = 12;
                        self.displayed_year -= 1;
                    } else {
                        self.displayed_month -= 1;
                    }
                }
                ui.add_space(10.0); // Add some spacing
                ui.heading(format!("{} {}", self.month_name(), self.displayed_year));
                ui.add_space(10.0);
                if ui.button(">").clicked() {
                    if self.displayed_month == 12 {
                        self.displayed_month = 1;
                        self.displayed_year += 1;
                    } else {
                        self.displayed_month += 1;
                    }
                }
            });

            ui.add_space(10.0); // Vertical spacing

            // Days of the week header
            ui.horizontal(|ui| {
                for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].iter() {
                    ui.label(*day);
                    ui.add_space(10.0); // Space between day names
                }
            });

            ui.add_space(10.0); // Vertical spacing

            // Calendar grid
            egui::Grid::new("calendar_grid")
                .min_col_width(30.0) // Ensure columns are wide enough
                .show(ui, |ui| {
                    for row in 0..number_of_rows {
                        for col in 0..7 {
                            let cell_index = row * 7 + col;
                            if cell_index < offset || cell_index >= offset + days_in_month {
                                ui.label(""); // Empty cell
                            } else {
                                let day = cell_index - offset + 1;
                                ui.label(format!("{}", day));
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}


fn main() {
    // Set up native options for the window
    let native_options = eframe::NativeOptions::default();
    // Run the app
    eframe::run_native(
        "Calendar App",
        native_options,
        Box::new(|_cc| Box::new(CalendarApp::new())),
    ).expect("Failed to run eframe application");
}