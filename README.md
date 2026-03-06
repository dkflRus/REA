**REA** is a simple extensible calendar-planner framework. 
 
A modular framework for building custom time‑planning applications in Rust.
 
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)
![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)
![(stars badge)](https://img.shields.io/github/stars/dkflRus/REA?style=social)
 
---
 
📌 Overview
 
REA is a framework that lets users build their own time‑planning applications by assembling independent, reusable components. Unlike traditional calendar apps with fixed features, REA gives complete freedom to define how schedules are created, modified, and visualised. It is designed with three core principles in mind:
 
- Extensibility – anyone can add new components (Apps, Extensions, Renders) to adapt the system to any scheduling need.
- Minimalism – the core framework does not impose any scheduling logic; it only orchestrates the flow of data between components.
- Protectiveness – if components are correctly configured, they can be arranged in any order without crashing the system.
 
> Informal analogy: Imagine children building a tower. The teacher (REA) does not touch the tower but ensures the children (components) behave and the structure stays safe.
 
REA is implemented in Rust and serves as a solid domain layer, completely separated from presentation and data sources. It includes a default graphical interface built with egui, but the core can be used with any frontend.
 
---
 
✨ Key Concepts
 
**EventTable** is the fundamental data structure: a list of Events, each having at least a start time and an end time. Events are identified by UUIDs and can be extended with additional fields if needed (the framework allows future customisation).
 
Three Types of Components:
1. Apps – modify the EventTable. They receive an input EventTable (plus any custom inputs provided by Extensions) and output a new EventTable. Apps are the only components that can change the schedule.
2. Extensions – provide external data to Apps or other Extensions. They can fetch information from files, sensors, databases, machine learning models, or user input via GUI. Extensions are reusable and can supply input to multiple Apps.
3. Renders – consume the final EventTable and output it in some form: a GUI window, a file, a web page, etc. Renders are the end point of a pipeline.
 
**Pipeline Configuration**
 
Users define the order of execution in a JSON file, specifying which Apps run, which Extensions feed into them, and which Renders produce output. The core validates the pipeline to ensure that all required inputs are satisfied and that there are no circular dependencies.
 
Safety & Flexibility
 
- Apps cannot read each other's internal state – they only communicate via EventTable.
- Extensions are isolated and can be reused across different pipelines.
- The core guarantees that any valid configuration will not panic (provided the components themselves are correctly implemented).
 
---
 
🛠️ Technologies Used
 
- Rust – for performance, memory safety, and fearless concurrency.
- egui – an immediate mode GUI library for the default interface.
- JSON – for pipeline configuration files.
- Git – version control.
 
---
 
🚀 Getting Started
 
Prerequisites
 
- Install Rust and Cargo: rustup.rs
 
Installation
 
1. Clone the repository:
 
   ```
   git clone https://github.com/dkflRus/REA.git
   cd REA
   ```
 
2. Run the example pipeline:
 
   ```cargo run```
 
   This will launch the default GUI with a sample configuration.
 
Creating Your Own Pipeline
 
1. Define your components (or use existing ones from the std collection).
2. Write a JSON configuration file (see example_project.md for syntax).
3. Load it via the GUI or the CLI.
 
---
 
🧠 What This Project Demonstrates
 
- Software architecture design – separating core logic from presentation and data sources, creating extensible interfaces.
 
- Rust proficiency – ownership, traits, generics, error handling, and safe concurrency.
- Modularity – designing a system where users can plug in their own components without modifying the core.
- Safety‑by‑design – enforcing rules at compile‑time and runtime to prevent crashes.
- Practical integration – linking with external ML models (e.g., CarML) via Extensions to build intelligent scheduling systems.
 
---
 
🔗 Integration with CarML
 
One of the most exciting developments is using REA together with my predictive maintenance project, CarML. In this setup:
 
- CarML acts as an Extension, providing predicted repair dates and descriptions.
- A custom App receives these predictions and inserts them into the EventTable as maintenance events.
- A Render visualises the resulting maintenance schedule in an interactive dashboard.
 
This integration demonstrates how REA can be used to turn abstract AI predictions into actionable plans – a perfect example of its real‑world applicability.
 
---
```
📂 Repository Structure
 
REA/
├── src/
│   ├── core.rs          # Core traits and pipeline orchestration
│   ├── gui.rs           # Default egui interface
│   ├── std/             # Standard library of pre‑built components
│   │   ├── adder.rs     # App that adds events
│   │   ├── filter.rs    # App that filters events
│   │   └── ...
│   └── lib.rs           # Library entry point
├── examples/            # Example pipeline configurations
├── Cargo.toml           # Rust dependencies and metadata
├── README.md            # This file
└── example_project.md   # Detailed pipeline guide
```
---
 
📄 License
 
This project is open source under the MIT License – feel free to use, extend, and contribute.
 
---
 
🙋‍♂️ About the Author
 
REA is created and maintained by Vladislav Sheremet, a first‑year Electrical Engineering and Information Technology student at Hochschule München. I developed REA to explore advanced Rust concepts and to build a tool that I personally wanted – a completely customisable calendar planner. It continues to evolve as I integrate it with other projects like CarML.
 
I am now applying to TUM Asia to deepen my knowledge in engineering and software systems, and I believe REA is a strong demonstration of my technical abilities and passion for clean, extensible design.
