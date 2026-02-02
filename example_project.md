This manula describes example task and execution for it using Rea, it coveres main parts of Rea usage and most common R/E/As. (<-TODO: Rewrite this)

# Initial task
We have a company B, it has a set of coworkers. They all work on some projects that are presented as a list of tasks with approximate times required to execure those tasks. Some tasks must be done outside main office: in other office or somewhere else, so time taken to drive between points must be noted too.

REA must automate creation of timetables for coworkers.  
## Inputs
- Working hours and days in format TODO
- SQL database with tables containing:
    - Coworkers names
    - List of tasks in format: task name|required time for execution|list of people who can exexcute it|location of task executions
    - Map with marked locations of the executions
## Outputs
Each coworker recieves ones timetable in format one wants it.

# Execution
## Planning
So, rea will need to:
- export info from SQL
-  
- distribute those tasks in accordance to working days/hours
- send timetables to workers