Taskman is simple CLI task tracking and prioritization tool. After providing taskman with some tasks, it will use the following formula to suggest the most important tasks for you to do right now.

$$\frac{\textnormal{normalized priority}}{\textnormal{cost in minutes}} + \frac{\textnormal{cost in minutes}}{\textnormal{minutes until deadline}}$$

Taskman is intended to be used for small tasks that often come up during the day that you may not have the time to manage in the moment.

# Todos
## v0
- [x] Write new task
- [x] Show tasks
    - [x] tasks are always sorted on output, ranked by priority
- [ ] 'Write' mode, where you access the tasks directly with a text editor (this should just be a shortcut)
- [x] mark tasks complete

## v1
- [ ] All constants defined by config with default values, probably use yml here
- [ ] Include a way to adjust priority calculation by config 
    - [ ] maybe even let users use their own script for calculating priority?

## v2
- [ ] Dependencies between tasks, task A cannot be completed until task B is completed
    - [ ] Should affect how priority is calculated
