**REA** is a simple extensible calendar-planner framework. Partly written with use of Grok.

CAPSLOCK written verbs MUST be used in accordace to [Key words for use in RFCs](https://www.rfc-editor.org/rfc/rfc2119)

# Installation
Install Rust,
clone this repo with `git clone` and
run with `cargo run`

Also ensure all time periods are between DateTime::MIN_UTC and DateTime::MIN_UTC (January 1, 262144 BCE - December 31, 262142 CE), especially when switching from UTC.

# Project structure

## Philosophy

### Abstract
Time planning is a complex task. Making program for time planning is even more complex - *each person has his/her own time schedule requirements and techniques*. So all time schedule constructing instructions can be chosen by user himself/herself with use of elements which are combined with each other in user-defined order.
If element that fullfills users requirements does not exist, user can write this element himself/herself.
REA system posltulates that time schedule is a list of **Events**, each of which has AT LEAST [start and stop] times.


To ensure that any and every user can be satisfied with REA program, REA has a set of design requirements:

- Extensibility: REA timetable generation process can be
    - infinitely extensible and
    - arbitrarily configured. 

    User has complete freedom over 
    - the choice of elements and
    - REA output further handling.

- Minimality: REA framework itself contains no [reading or modification] of timetable generation process.
> Informal: Imagine kids build the tower. The teacher doesn't touch the tower but makes sure the kids behave. Teacher is like REA(ensures order), B is time table that is constructed and kids are elements.

- Protectiveness: REA framework ensures that user can freely define execution order of the elements.
> Informal: REA design ensures that elements *(later Apps+used Extensions)* can be stacked in any order and program will not crush/panic. In other words if all elements and their inputs are configured then user can not place them in such an order that code crashes.

Those requirements define whole design and functionality of the REA framework and guide [ [input provision/output further handling] methods and elements] of REA framework creation.

### Structure blocks (R/E/A definition)
To fullfill those, EventTable, Apps(elements), Extensions(input gathering methods) and Renders(REA output further handling) are defined.

Render(s), Extension(s) and App(s) can be shortened as `R/E/A(s)`. Not to be confused with `REA` - calendar-planner framework.


/Those definitions are very formal, so dont worry if you can not understand those fully, there are examples below./
- **EventTable** (abbreviated as **ET**) - time table format. Since main REA calendar app purpose is time planning, of cource it needs some time table where events are stored. ET is defined in `core.rs` and includes EventName, EventStart, EventFinish and EventUUID *(≈EventName, more on it below)*.
Btw Events should be **unable to delete**, but they could be split or moved. It is based on logic that there all actions are SOMEHOW important, so there are no actions that can be automatically cancelled, but they can be moved at the end of ET.
Event theoretically can overlap, 
> TODO: ET structure isn't set in stone, although it ensures lack of connections between Apps (there is much lower probability that new user-written App would be designed to follow other already existing UNIQUE app = against Protectiveness requirement) it is too strict and includes only vital(that is kinda too low) information.
> Probably other fields must be included. Also maybe include users ability to add fields to ET in his REA piping.


- **App** - time schedule constructing instruction, or more precise ET modifying peice of code. Apps have arbitiral input set by App creator, more on it below. Apps only connection with each other is a single ET pipe - the only universal undeniable information that must be transmitted from one app to another is a time table itself!
App must have no external inputs, only those that are pipied in by REA. This rule can not defined with Rust :c
- **Extension** - responsible for R/E/A input provision (yes, Extensions themselves can use other Extensions for input). Can be designed 
    - for usage only with specific R/E/A or 
    - be completely universal or
    - somewhere inbetween,
alghough hypotheitcally if some output type of Extension is same with some input type of R/E/A then they can be connected.
Extension can freely use external inputs, it may have no inputs in REA system at all and still gather information from externals like internet or filesystem.

Single Extension may dublicate one output to multiple Apps inputs or to multiple Apps. All Extensions recieve (last generated) ET from previous App (or empty ET if this Extension is required by first App in pipeline).
Important to mention that if Extension0 gives input to AppI and AppJ (where I and J are indexes of AppI and AppJ, I < J), E0 recieves A(I-1) ET. But AJ may require A(J-1) as input, so E0 should recieve A(J-1) ET, what creates dependence paradox (E0 requres A(J-1) work first to recieve its ET, A(J-1) requres AI work first /as well as all inbetween AI and A(J-2)/ to recieve its ET, AI requres E0 work first to recieve proper input). In this case REA will give error by generate_order_of_processing()

> TODO: Instead of Apps dev may try to use Extensions, since thay are equal up to the point of outputs: App can have only 1 ET as putput while Extensions outputs are unregulated. It is hard to distinguish direct user-provided and ET inputs by design, so it will stay flaw that allows creation of very useful Extension that will dictate design of other R/E/As and stand against Protectiveness.
> Partly corrected with core::Pipeline.check_classes


- **Renders** - output further handling parts of code. Generally they are piping ET to GUI(so general rendering) or to file or to URL or to online interface or somewhere else. Renders are last element of ET pipeline and can have no further piping.


A list of reccomendations for R/E/A creation:
- Clarity: REA framework R/E/As SHOULD be open source
- Atomicity: REA framework Apps SHOULD be so small that its separation in multiple elements must be
        - impossible due to lack of information transmitted between R/E/As or
        - involve decomposition of the original idea so that it has no sense.
- GUI compatibility: All R/E/As SHOULD have a default Extension(s) [for all App provided by direct user input input(s)] that is compatibile with **eframe**. One Extension can be responsible for multiple App inputs.
Those default Extensions can be replaced ith user-defined ones. If at least one R/E/A input is not provided by user-provided Extensions, default Extension is executed. If default Extension outputs and user-provided Extension output(s) are for same R/E/A input, user-provided Extension has priority and those default Extension outputs are ommited.


### Piping

Here's how piping works:
- Zeroth App recieves empty ("empty" is an initial state) EventTable, adds something to it with Users instructions, passes it to first App
- First App recieves EventTable from App0, modifies or adds something to it with Users instructions, passes it to second App

and so on.


So, User sets App0, App1 and App2 in a row and gives input to them (with GUI by default). But sometimes using GUI and/or writing ALL info for App by hand is inefficient or impossible, so **Extensions** are introduced.
**Extensions** are special pieces of code that replaces one or many of App defined inputs. Each Extension is designed for one app, in rare cases one Extension can be used for different Apps with simmilar inputs.

### GUI
Technically GUI is not a part of REA system, nevertheless it is required for user-friendliness. For that `gui.rs`  exists.

## Overview
Calendar is 100% extensible, all functionality is based on R/E/A, relations and data piping between them are controlled by REA Core.

REA itself is **solid Business Logic / Domain Layer**.
This project also contains
    defualt Presentation Layer (GUI) and
    (maybe) default Data Access Layer (sqlite connection).

1. **Core** [in `core.rs`, mounted in GUI with `main.rs`]: responsible for REA structure and organisation.
    
    **Core is unable to edit EventTable.** Core is a part of code that:
    - Pipes...
        - EventTable in Apps->Apps/Exts link and 
        - data in EventTable in Exts=>Apps link
    - Renders tabs for Apps and fiels for Exts.

... and 3 types of user-defined parts:

2. **Apps** [in other `.rs` files user will install yourself]: responsible for Events addition and modification (and the only element of R/E/A that can modify ET).
    Input:
        - 1 EventTable (to prevent any transmissions between REA elements **Names of events CAN NOT be read**. They can be modified although through appending something to them. Events can be handles with thier UUIDs) and 
        - its own inputs(each input can be transmitted via GUI or Extension)
    Output:1 EventTable
3. **Extensions** [in other .rs files near Apps ]: Modify and complement work of Apps by changing App input
    Input: Any
    Output: Any (piped to R/E/A)
4. **Render**: Output and ending of piping.
    Input: EventTable(full access)
    Output: Any (outside pipeline)


## Piping order

All R/E/As have 2 types of inputs:
1. GUI as input (technically this GUI can be empty if no arguments are requred. In this case App/Ext. tab will be blank).
2. Output of previous Apps/Extensions (App0 has empty EventTable)

If Project structure is:
```
/No Extensions/  [Ext1.0][Ext1.1] [Ext2.0] 
[     App0     ] [     App1     ] [     App2     ][   Render0   ]
```

Means that:
- **App0** has only **empty EventTable + GUI** as input
- **App1** has only **EventTable after App0 + Data from Ext1.0 + Data from Ext1.1 + GUI(except elements that are controlled by Ext1.0 and Ext1.1)** as input
- **App2** has only **EventTable after App1 + Data from Ext2.0 + GUI(except elements that are controlled by Ext2.0)** as input
/also Ext1.# recieves RT from A0 and Ext2.# recieves RT from A1/

Then data piping is:
```
[App0(GUI user input)]-(only EventTable)->

    ->[Ext1.0(+GUI)]=(input data for App1)=>
    ->[Ext1.1(+GUI)]=(input data for App1)=>
=(Ext1.0+Ext1.1+EventTable)=>[App1(+GUI)]-(only EventTable)->

    ->[Ext2.0(+GUI)]=(input data for App2)=>
=(Ext2.0+EventTable)=>[App2(+GUI)]
=>[Render0]
```
,where thin arrow (`->`) is only EventTable and fat arrow (`=>`) is other input data.
Data differentates between Apps

### Storage of piping order

Pipeline file has json format, it contains:
1. Aliases of R/E/As' names
2. Baseline Apps with Render(s)
3. All other connections between R/E/As

#### "Alias"
section allows giving aliases to R/E/A names. It is helpful if
- R/E/A can be switched to another R/E/A sponaniousley or
- if name is too long.
In next sections names of R/E/A can be declared with use of alias or as their direct name.

#### "Baseline"
is an array of Apps, ending with Extension. Any App of Baseline can have amid Extenion(s), to which amid ET will be piped.
Syntax:
```
"Pipeline":[
        "App0", // App with no renders, ET will be piped to App1 only
        {"App1":[]}, // Also App with no renders, ET will be piped to App2 only
        {"App2":"Render2.0"}, // App with 1 render, ET will be piped to App3 and Render2.0
        {"App3":["Render3.0","Render3.1"]}, // App with 3 renders, ET will be piped next app and and Render2.0
        ...
        "AppN",
        "Render" //Specified endine Render. "Pipeline" can also end with app, then 
    ],
```

#### "Connections"
section specifies other connections between R/E/As. This is the only section where piping from/to Extensions can be declared.






## GUI
Since almost all R/E/A elements require some input, all of those inputs must be allowed to be provide by GUI. Since R/E/A elements have different inout types to accept those with GUI helping default Extensions


## std
List of R/E/As have a vast usage potential and are preprogrammed by developers:

Apps:

|App name|Purpose                          |Inputs|
|--------|---------------------------------|------|
|Adder   |adds defined Events to ET        |      |
|Filter  |lets only selected Events through|      |

Currently REA supports only one pipe per one running Core.
> TODO: allow pipes branching

> TODO:
> (Parts of code where those TODOs must be applied are marked with their names)
> [ ] Allow more complex piping with:
>> multiple EventTables as input for or
>> ability to loop parts of pipeline with Apps
> [ ] Async?
