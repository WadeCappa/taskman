Taskman is a simple CLI task tracking and prioritization tool. 

Taskman uses the following formula to order tasks from most important to least important:

$$\frac{\textnormal{normalized priority}}{\textnormal{cost in minutes}} + \frac{\textnormal{cost in minutes}}{\textnormal{minutes until deadline}}$$

## v0
- [x] Write new task
- [x] Show tasks
    - [x] tasks are always sorted on output, ranked by priority
- [x] delete tasks
- [x] mark tasks complete
- [x] install script (simple with bash)

## v1
- [ ] Can triage tasks
- [ ] The task priority is affected by how old a task is
- [ ] Can edit a task, only change parameters added to the cmd
    - [ ] Need semantics for empty values

## v2
- [ ] Reoccuring tasks
- [ ] Dependencies between tasks, task A cannot be completed until task B is completed
    - [ ] Should affect how priority is calculated

## v3
- [ ] All constants defined by config with default values, probably use yml here
- [ ] Include a way to adjust priority calculation by config 
    - [ ] maybe even let users use their own script for calculating priority?

