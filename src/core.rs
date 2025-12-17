//Responsible for whole App structure and regulation.


use uuid::Uuid;

use std::collections::HashSet;
use std::ops::Index;
// ==========EVENTTABLE==========
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Utc};
use rand::Rng;
use rusqlite::types::Type;


type DateTimeType = DateTime<Utc>;

///Reponsible for single Event information handling. Generally must be static (mush have no impl), but can be interactive too
#[derive(Clone, Debug)]
pub struct Event {
    pub uuid: Uuid,
    name: String,            //TODO?: Apps MUST NOT read this field directly → use append_name()
    pub start: DateTimeType,
    pub end:   DateTimeType,
}

///Responsible for Events storage and control over their correctness.
#[derive(Clone, Debug, Default)]
pub struct EventTable {
    events: Vec<Event>,
    ids:    HashSet<Uuid>,
}

impl EventTable {

    // ====ET MODIFICATIONS====

    pub fn new()->Self {
        Self::default()
    }

    /// Validation of new event
    pub fn check_event(event: &Event
    )->Result<(), String> 
    {
        if event.start > event.end {
            return Err(format!("Event {} has end < start", event.uuid));
        }
        Ok(())
    }
    /// Whole check_event + specific for ET checks that single Event does not have
    pub fn check_self(&self
    )->Result<(), String> 
    {
        if self.events.len() != self.ids.len() {
            return Err("EventTable corrupted: length mismatch between events and ids".into());
        }

        for event in &self.events {
            if !self.ids.contains(&event.uuid) {
                return Err(format!("Missing UUID {} in id set", event.uuid));
            }
            Self::check_event(event)?;
        }
        Ok(())
    }

    /// Add a new event. Random UUID is automatically generated
    pub fn add(&mut self,
        name: String,
        start: DateTimeType,
        end: DateTimeType
    )->Result<(), String> {
        let uuid = Uuid::new_v4();

        let event=Event { uuid, name, start, end };
        Self::check_event(&event)?;

        self.events.push(event);
        self.ids.insert(uuid);

        // Check must be done after any modification;
        self.check_self()?;

        Ok(())
    }

    /// Splits one Event in a set of events. All of child-Events have [parent-Events name]+suffix
    /// input list of child-Events info: Vec<(StartTime,EndTime,Suffix)>
    pub fn split(&mut self,
        data:Vec<(
            DateTimeType,
            DateTimeType,
            String)>
    ){
        //TODO
        todo!();

    }



    // ====PRECISE EVENTS MODIFICATION====

    /// Safe mutable access by UUID
    fn get_mut(&mut self,
        uuid: Uuid
    )->Option<&mut Event> {
        if self.ids.contains(&uuid) {
            self.events.iter_mut().find(|e| e.uuid == uuid)
        } else {
            None
        }
    }

    /// Apps may only append to name, never read it → no information leak between Apps
    pub fn append_name(&mut self,
        uuid: Uuid,
        suffix: &str
    )->Result<(), String> {
        if let Some(ev) = self.get_mut(uuid) {
            ev.name.push_str(suffix);
        } else {
            return Err(format!("Missing UUID {}", uuid));
        }

        self.check_self()?;
        Ok(())
    }

    /// Change times safely
    pub fn set_times(&mut self,
        uuid: Uuid,
        start: DateTimeType,
        end: DateTimeType
    ){
        if let Some(ev) = self.get_mut(uuid) {
            ev.start = start;
            ev.end = end;
        }

        self.check_self();
    }

    /// Read-only iterator (useful for Extensions)
    pub fn get_events(&self
    )->std::slice::Iter<'_, Event> {
        self.events.iter()
    }
}



// ==========R/E/A==========
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// All IO of R/E/As except ET in Baseline
type IOType = HashMap<
    String,                     // Name
    Box<dyn Any+Send+Sync>      // Input/output itself
>;

enum R_E_AClass {
    Render,
    Extension,
    App
}

// DRY traits
// (check ALL classes run() inputs after redacting any of them)

pub trait R_E_AGeneric{
    fn get_name(&self)->&'static str;
    fn get_class(&self)->&'static R_E_AClass;

    /// Inputs of this R/E/A (except ET for R/A), can be empty.
    /// Inputs names, types and amount can be static(declared in code before compiling) or dynamic(procedurally generated)
    fn get_inputs(&self)
    ->&'static Vec<(
        String,
        TypeId
    )>;
}
pub trait Render: R_E_AGeneric{
    fn run(&self,
        et: EventTable,
        inputs: IOType
    )->Result<(), String>;
}
pub trait Extension: R_E_AGeneric{
    fn get_outputs(&self)
    ->&'static Vec<(
        String,
        TypeId
    )>;
    
    fn run(&self,
        inputs: IOType
    )->Result<(),
        Vec<(
            String,
            Box<dyn Any>
        )>
    >;
}
pub trait App: R_E_AGeneric{
    fn run(&self,
        et: EventTable,
        inputs: IOType
    )->Result<EventTable, String>;
}


pub enum R_E_A {
    Render(Box<dyn Render>),
    Extension(Box<dyn Extension>),
    App(Box<dyn App>),
}

impl R_E_AGeneric for R_E_A {
    fn get_name(&self) -> &'static str {
        self.get_name()
    }

    fn get_class(&self) -> &'static R_E_AClass { // TODO: can be optimized?
        match self {
            R_E_A::App(_) => &R_E_AClass::App,
            R_E_A::Extension(_) => &R_E_AClass::Extension,
            R_E_A::Render(_) => &R_E_AClass::Render,
        }
    }

    fn get_inputs(&self) -> &'static Vec<(String, TypeId)> {
        self.get_inputs()
    }
}



// ==========PIPELINE==========
// Each R/E/A in full run of Pipeline can be executed only once. If one program will be used repeatedly, they will be stored in alias multiple times with different Uuids

struct Pipeline{
    /// If 'true': checkup [if all categories have correct class type] will be executed after any modification or import. Some checkups (for example check of "connections" if selected Element has this iutput/input) will always be done.
    pub check_classes:bool,

    pub R_E_AList:HashMap<
        Uuid,
        Box<R_E_A>
    >,
    pub baseline:Vec<(
        Uuid,             // App
        Vec<Uuid>         // List of Renders
    )>,
    pub connections:Vec<(
        Uuid,String,      // Element uuid + output name
        Uuid,String       // Element uuid + input name
    )>,

    /// Each element of Vec is attached to Baseline App by index
    /// Full execution order: [All R/E/As from Vec[0]<Uuid> in order thay are in Vec<Uuid>] [App0] [All R/E/As from Vec[1]<Uuid>] [App1] ...
    execution_order:Vec<
        Vec<Uuid>
    >,
    /// Buffer where E/A outputs will be stored until [R/E/A to which they are piped] are executed.
    memory_buffer:HashMap<Uuid, // Uuid of element that gives output
        HashMap<String, // Name of output
            Box<dyn Any> // Output itself
        >
    >,
    current_et:EventTable
}
impl Pipeline {
    // === Checks

    // === runs + required functions

    /// Assembles input for R/E/A from connections and memory_buffer
    fn pull_inputs(
        &self,
        elementID: &Uuid,
    )->Result<
        IOType,
    String>{
        if let required_inputs = self.R_E_AList.get(elementID).unwrap()
        .get_inputs()
        {
            let mut inputs:IOType;
            let connections=self.connections;

            for (name, type_id) in required_inputs {
                if let Some(val) = inputs.get(name){
                    if val.type_id() != *type_id{
                        return Err(format!("Type mismatch for {} in {}", name, element.get_name()));
                    }else{
                        inputs.insert(
                            name.clone(),
                            { // searching and getting this input
                                for q in connections{
                                    if q.3==name
                                }
                            }
                        );
                    }
                }else{
                    return Err(format!("Missing input {} for {}", name, element.get_name()));
                }
            }

            Ok(inputs)
        }else{
            Err(format!("{} not found in Alias",elementID))
        }
    }

    /// Define order of R/E/A execution with Kahn's Algorithm
    fn generate_order_of_processing(){

    }



    pub fn execute_element(&mut self, 
        r_e_a_uuid: Uuid, 
        chunk_inputs: &IOType
    ) -> Result<(), String> 
    {
        let variant = self.R_E_AList.get(&r_e_a_uuid)
            .ok_or(format!("R/E/A element {} not found", r_e_a_uuid))?;

        match variant.as_mut() {
            R_E_A::App(app) => {
                let inputs = self.pull_inputs(app, chunk_inputs)?;  // Merge/compute specific inputs
                let new_et = app.run(self.current_et.clone(), inputs)?;
                self.current_et = new_et;
                self.current_et.check_self()?;
            }
            R_E_A::Extension(ext) => {
                let inputs = self.pull_inputs(ext, chunk_inputs)?;
                let outputs = ext.run(inputs)?; //TODO:Allow provision of current ET
                // Handle outputs: e.g., merge into next chunk's inputs or store.
                // For now, assume it updates a temp map or something; adjust per needs.
            }
            R_E_A::Render(render) => {
                let inputs = self.pull_inputs(render, chunk_inputs)?;
                render.run(self.current_et.clone(), inputs)?;
            }
        }
        Ok(())
    }

    /// Execute part of baseline until (including) nth.
    /// All previous parts also must be executed since it is.
    fn run_baseline_until(&mut self, index:usize){
        for chunk in 0..index{
            for R_E_AUuid in self.execution_order.index(chunk){
                self.R_E_AList.index(R_E_AUuid).;
            }
        }
    }

    fn run_full(&self){
        fn self.run_baseline_until(self.baseline.len())
    }



    // Work with files

    /// Pull pipeline from file
    pub fn import(
    ){

    }
    /// Push pipeline from file
    /// Currently obsolete, but can be useful after changing pipeline with GUI
    pub fn export(){
        // std::fs::

        //use std::path::{Path, PathBuf};
    }
}