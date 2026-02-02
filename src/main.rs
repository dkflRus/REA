// Replonsible for Apps and Utils proper piping. Anything that exists OUTSIDE A&E.
// Technically main.rs is not required for core.rs. It is a default layout for A&E use. ??????????????

use eframe::{App, egui};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Utc};
use rusqlite::{Connection, Result as SqlResult};

use std::any::TypeId;

use crate::core::EventTable;

mod core;
mod gui;

// Definitions


// End Definitionss


// const APPS_PIPELINE: Vec<Box<dyn App>> = vec![ //TODO: make changable through setting in file (.csv or smth)
//         Box::new(App1),
//         Box::new(App2),
//     ]; 

fn main(){
    gui::main();
}



struct REAAppBackend{
    
}
impl REAAppBackend {
    fn REA_update(starting_index:Option<u32>){

        // for app in APPS_PIPELINE[starting_index.unwrap_or(0)..]{}
    }
}




// struct PomodoroApp(){

// }
// impl App for PomodoroApp{
//     fn get_name(){"Pomodoro"}
//     fn get_inputs()->{
//         EventTable,
//         Vec<core::UUIDType> //Events to apply pomodoro (TODO:checking if piped by E events exist in ET recieved by A) //????????????????????
//     };
//     fn process(&self,app_input:InputList)->Option<EventTable>;

// }
