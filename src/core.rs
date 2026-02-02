//Responsible for whole App structure and regulation.
// For devs: during compairing entities like R/E/As or Uuids look carefully after if it is object==object or value==value


use egui::epaint::ahash::random_state::RandomSource;
use egui::output;
use uuid::Uuid;

use std::arch::x86_64;
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
    name: String,            //TODO?: E/As MUST NOT read/write this field directly → use append_name()
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

/// All [IO of R/E/As] except [ET in Baseline]. Don't be confused, it is NOT TYPE of TYPE of IO; it is type of IO; it already contains IO info that is piped to make R/E/As communicate.
type IOType<'a> = HashMap<
    String,                     // Name
    &'a Box<dyn Any+Send+Sync>      // Input/output itself
>;

enum R_E_AClass {
    Render,
    Extension,
    App
}

// DRY traits
// (check ALL classes run() inputs after redacting any of them)

/// Datatype for listing inputs/outputs of R/E/As
type IOListOfTypes=HashMap<
    String,
    TypeId
>;

pub trait R_E_AGeneric{
    fn get_name(&self)->&'static str;
    fn get_class(&self)->&'static R_E_AClass;

    /// Inputs of this R/E/A (except ET for R/A), can be empty.
    /// Inputs names, types and amount can be static(declared in code before compiling) or dynamic(procedurally generated)
    fn get_inputs(&self)
    ->&'static IOListOfTypes;
}
pub trait Render: R_E_AGeneric{
    fn run(&self,
        et: EventTable,
        inputs: IOType
    )->Result<(), String>;
}
pub trait Extension: R_E_AGeneric{
    fn get_outputs(&self)
    ->&'static IOListOfTypes;
    
    fn run(&self,
        inputs: IOType
    )->Result<
        IOType, // outputs of Extension
        String //Err
    >;
}
pub trait App: R_E_AGeneric{
    fn run(&self,
        et: EventTable,
        inputs: IOType
    )->Result<
        EventTable, // for further baseline R/A
        String //Err
    >;
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

    fn get_inputs(&self) ->&'static IOListOfTypes{
        match self {
            R_E_A::Render(R) => R.get_inputs(),
            R_E_A::Extension(E) => E.get_inputs(),
            R_E_A::App(A) => A.get_inputs(),
        }
    }
}



// ==========PIPELINE==========
// Each R/E/A in full run of Pipeline can be executed only once. If one program will be used repeatedly, they will be stored in alias multiple times with different Uuids

struct Pipeline<'a>{
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
        IOType<'a> // Name of output+Output itself
    >,
    current_et:EventTable
}
impl Pipeline<'_> {
    // === Checks

    // === runs + required functions

    /// Assembles input for R/E/A from [connections and memory_buffer] for [element with UUID]
    //TODO: optimize
    fn pull_inputs(
        &self,
        elementID: &Uuid,
    )->Result<
        IOType,
    String>{
        if let element = self.R_E_AList.get(elementID).unwrap() // checking out R/E/A for which we assemble inputs
        {
            let mut inputs:IOType=IOType::new(); //final output of pull_inputs
            let required_inputs=element.get_inputs();


            let mut cropped_connections:Vec<&(Uuid, String, Uuid, String)>=Vec::new(); //=all connections of this R/E/A
            for q in &self.connections{
                if q.2==*elementID{cropped_connections.push(q)} // checking if target UUID R/E/A is our elementID; TODO: check if it does value==value, not obj==obj
            }
            for (curr_input_name, curr_input_type_id) in required_inputs {
                let (source_uuid,source_name)={//getting [uuid of output element] and output name
                    let index_try=cropped_connections.iter()
                    .find(|x| x.3==*curr_input_name);

                    if let Some(index)=index_try{
                        (index.0,index.1.clone())
                    }else{ //if None
                        return Err(format!("pull_inputs(): connection for R/E/A {} with input name {} not found",
                        elementID,curr_input_name));
                    }
                };

                let output_type=*(self.R_E_AList.get(&source_uuid).unwrap()).get_inputs().get(&source_name).unwrap();

                let debug_string=format!("pull_inputs(): output of Extension {} named {} has type {:?}\n input of R/E/A {} with input name {} has type {:?}\n",source_uuid,source_name,output_type,
                elementID,curr_input_name,curr_input_type_id);

                if  output_type!=
                    *curr_input_type_id{
                    return Err(debug_string+"Different types");
                }


                // Finally getting input from self.memory_buffer yay!!
                let wrapped_input_itself:Option<&&Box<dyn Any + Send + Sync + 'static>>=self.memory_buffer.get(&source_uuid).unwrap().get(&source_name);
                if let Some(input_itself)=wrapped_input_itself{
                    inputs.insert(
                        curr_input_name.to_string(),
                        input_itself
                    );
                }else{// if None
                    return Err(debug_string+"This output is not found in memory_buffer, either it is not loaded YET or\n it does not exist (unusual since all names are checked in this function)")
                }
                        

                    


                // if let Some(val) = inputs.get(curr_input_name){ //checking out
                    // if val.type_id() != *curr_type_id{
                    //     return Err(format!("Type mismatch for {} in {}", curr_input_name, element.get_name()));
                    // }else{
                    //     inputs.insert(
                    //         curr_input_name.clone(),
                            
                    //         { // searching and getting this input
                    //             for connection_list in self.connections{
                    //                 if connection_list.3==curr_input_name{

                    //                     return 
                    //                 }
                    //             }

                    //         }
                    //     );
                    // }
                // }else{
                //     return Err(format!("Missing input {} for {}", curr_input_name, element.get_name()));
                // }
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
    ) -> Result<(), String> {
        let mut outputs_box= None; // in case if R/E/A produces output

{        let inputs = self.pull_inputs(&r_e_a_uuid)?;  // Assume owned; borrow ends
    
        let et_clone = self.current_et.clone();  // Borrow ends
    
        // Get variant mut ref
        let variant_ref = self.R_E_AList.get(&r_e_a_uuid)
            .ok_or(format!("R/E/A element {} not found", r_e_a_uuid))?;
        
        // Match on mutable ref to avoid holding borrow across arms
        match &**variant_ref {
            R_E_A::Render(render) => {
                render.run(et_clone, inputs)?;
            }
            R_E_A::Extension(ext) => {
                outputs_box = Option::Some(ext.run(inputs)?); 
    
                // Drop variant_ref borrow by scoping
            }  // End match, borrow drops
    
            R_E_A::App(app) => {
                let new_et = app.run(et_clone, inputs)?;
                self.current_et = new_et;
                self.current_et.check_self()?;
            }
        }
}
        // Now safe to borrow memory_buffer
        if let Some(outputs)=outputs_box{ //TODO
            // let memory_cell = self.memory_buffer.get_mut(&r_e_a_uuid)
            // .ok_or(format!("Memory cell for {} not found", r_e_a_uuid))?;
            // memory_cell.extend(outputs);
            todo!();
        }

        Ok(())
    }

    /// Execute part of baseline until (including) nth.
    /// All previous parts also must be executed since it is.
    fn run_baseline_until(&mut self, index:usize){
        for chunk in 0..index{
            for R_E_AUuid in self.execution_order.index(chunk){
                // self.R_E_AList.index(R_E_AUuid).;
                todo!();
            }
        }
    }

    fn run_full(&mut self){
        self.run_baseline_until(self.baseline.len())
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