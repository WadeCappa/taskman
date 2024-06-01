TaskMan is an implementation of a task sorting strategy that I've already implemented in excel (sans task-dependency support). Implementing it for linux for portability and easy of use.

# Todos
## v0
- [x] Write new task
- [x] Show tasks
    - [x] tasks are always sorted on output, ranked by priority
- [ ] 'Write' mode, where you access the tasks directly with a text editor (this should just be a shortcut)
- [ ] mark tasks complete

## v1
- [ ] All constants defined by config with default values, probably use yml here
- [ ] Include a way to adjust priority calculation by config 

## v2
- [ ] Dependencies between tasks, task A cannot be completed until task B is completed
    - [ ] Should affect how priority is calculated
