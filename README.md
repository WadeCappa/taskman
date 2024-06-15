Taskman is a simple CLI task tracking and prioritization tool. 

Taskman uses the following formula to order tasks from most important to least important:

$$\frac{\textnormal{normalized priority}}{\textnormal{cost in minutes}} + \frac{\textnormal{cost in minutes}}{\textnormal{minutes until deadline}}$$

## v0
- [x] Write new task
- [x] Show tasks
    - [x] tasks are always sorted on output, ranked by priority
- [x] delete tasks
- [x] mark tasks complete
- [ ] install script 

## v1
- [ ] Reoccuring tasks
- [ ] Dependencies between tasks, task A cannot be completed until task B is completed
    - [ ] Should affect how priority is calculated

## v2
- [ ] All constants defined by config with default values, probably use yml here
- [ ] Include a way to adjust priority calculation by config 
    - [ ] maybe even let users use their own script for calculating priority?

