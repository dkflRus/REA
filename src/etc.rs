use chrono::Local;

struct CalendarView {
    displayed_month: UUID_TYPE,
    displayed_year: i32,
}

impl CalendarView {
    fn new() -> Self {
        let now = Local::now();
        Self {
            displayed_month: now.month(),
            displayed_year: now.year(),
        }
    }
}

impl View for CalendarView {
    fn name(&self) -> &str {
        "Calendar"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Calendar");
        ui.label(format!(
            "Month: {}, Year: {}",
            self.displayed_month, self.displayed_year
        ));
        // Add your existing calendar UI logic here, e.g., buttons to change months,
        // a grid for days, etc.
        if ui.button("Next Month").clicked() {
            self.displayed_month = (self.displayed_month % 12) + 1;
            if self.displayed_month == 1 {
                self.displayed_year += 1;
            }
        }
    }
}




use eframe::egui;
use chrono::{Datelike, Duration, Local, NaiveDate};
use rusqlite::{Connection, Result as SqlResult};

// Define the CalendarApp struct to hold the app's state
struct CalendarApp {
    displayed_month: UUID_TYPE,
    displayed_year: i32,
    db_conn: Connection,
    selected_date: Option<NaiveDate>,
}

impl CalendarApp {
    // Constructor to initialize with the current month and year, and set up the database
    fn new() -> Self {
        // Open or create the SQLite database file
        let conn = Connection::open("calendar.db").expect("Failed to open database");

        // Create the events table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                start_datetime TEXT NOT NULL,
                end_datetime TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create events table");

        // Insert a sample event if the table is empty
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .expect("Failed to count events");
        if count == 0 {
            conn.execute(
                "INSERT INTO events (name, start_datetime, end_datetime) VALUES (?, ?, ?)",
                &["Sample Event", "2023-01-01 10:00:00", "2023-01-01 11:00:00"],
            ).expect("Failed to insert sample event");
        }

        // Initialize with the current month and year
        let now = Local::now();
        Self {
            displayed_month: now.month(),
            displayed_year: now.year(),
            db_conn: conn,
            selected_date: None,
        }
    }

    // Helper function to get the month name
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

// Implement the eframe::App trait for GUI rendering
impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Calculate calendar layout
            let first_day = NaiveDate::from_ymd_opt(self.displayed_year, self.displayed_month, 1)
                .unwrap();
            let offset = first_day.weekday().num_days_from_monday();
            let next_month = if self.displayed_month == 12 {
                NaiveDate::from_ymd_opt(self.displayed_year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(self.displayed_year, self.displayed_month + 1, 1).unwrap()
            };
            let last_day = next_month - Duration::days(1);
            let days_in_month = last_day.day();
            let total_cells = offset + days_in_month;
            let number_of_rows = (total_cells + 6) / 7;

            // Navigation controls
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    if self.displayed_month == 1 {
                        self.displayed_month = 12;
                        self.displayed_year -= 1;
                    } else {
                        self.displayed_month -= 1;
                    }
                }
                ui.add_space(10.0);
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

            ui.add_space(10.0);

            // Weekday headers
            ui.horizontal(|ui| {
                for day in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].iter() {
                    ui.label(*day);
                    ui.add_space(10.0);
                }
            });

            ui.add_space(10.0);

            // Calendar grid with clickable days
            egui::Grid::new("calendar_grid")
                .min_col_width(30.0)
                .show(ui, |ui| {
                    for row in 0..number_of_rows {
                        for col in 0..7 {
                            let cell_index = row * 7 + col;
                            if cell_index < offset || cell_index >= offset + days_in_month {
                                ui.label("");
                            } else {
                                let day = (cell_index - offset + 1) as u32;
                                let date = NaiveDate::from_ymd_opt(
                                    self.displayed_year,
                                    self.displayed_month,
                                    day
                                ).unwrap();
                                let is_selected = self.selected_date == Some(date);
                                if ui.add(egui::Button::new(format!("{}", day))
                                    .selected(is_selected)).clicked() {
                                    if is_selected {
                                        self.selected_date = None;
                                    } else {
                                        self.selected_date = Some(date);
                                    }
                                }
                            }
                        }
                        ui.end_row();
                    }
                });

            // Display events for the selected date
            if let Some(selected_date) = self.selected_date {
                let date_str = selected_date.format("%Y-%m-%d").to_string();
                let start_of_day = format!("{} 00:00:00", date_str);
                let end_of_day = format!("{} 23:59:59", date_str);

                ui.add_space(10.0);
                ui.label(format!("Events on {}", date_str));

                // Query events overlapping the selected date
                let mut stmt = self.db_conn.prepare(
                    "SELECT name, start_datetime, end_datetime FROM events 
                     WHERE start_datetime < ? AND end_datetime > ?"
                ).expect("Failed to prepare statement");
                let events = stmt.query_map(&[&end_of_day, &start_of_day], |row| {
                    let name: String = row.get(0)?;
                    let start: String = row.get(1)?;
                    let end: String = row.get(2)?;
                    Ok((name, start, end))
                })
                .expect("Failed to query events")
                .collect::<SqlResult<Vec<_>>>()
                .expect("Failed to collect events");

                if events.is_empty() {
                    ui.label("No events on this day");
                } else {
                    for event in events {
                        ui.label(format!("{}: {} to {}", event.0, event.1, event.2));
                    }
                }
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calendar App",
        native_options,
        Box::new(|_cc| Box::new(CalendarApp::new())),
    ).expect("Failed to run eframe application");
}