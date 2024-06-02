pub mod db {
    use crate::{comparible_task::comparible_task::ComparibleTask, task::task::Task};
    use std::fs;
    use dirs::home_dir;

    const APP_DATA: &str = ".taskman/tasks.csv";

    pub fn write_task(task: Task) {
        let tasks = vec![task];
        write_tasks(tasks, true);
    }

    pub fn get_tasks(
        total_to_get: usize, 
        show_completed: bool
    ) -> Vec::<ComparibleTask> {
        let tasks: Vec<Task> = get_raw_tasks();
        let mut comp_tasks: Vec<ComparibleTask> = Task::make_comparible(tasks, show_completed);
        let return_size = usize::min(total_to_get, comp_tasks.len());
        return comp_tasks
            .drain(..return_size)
            .collect();
    }

    pub fn mark_complete(task_id: usize) {
        let mut tasks = get_raw_tasks();
        tasks[task_id].complete();
        write_tasks(tasks, false);
    }

    fn get_raw_tasks() -> Vec<Task> {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        let file = fs::File::open(path).unwrap();
        return csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .map(|record| record.unwrap())
            .collect();
    }

    fn write_tasks(tasks: Vec<Task>, append: bool) {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        match fs::OpenOptions::new().create(true).write(true).append(append).open(path.as_path()) {
            Ok(file) => {
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(file);

                for task in &tasks {
                    wtr.serialize(&task).unwrap();
                }

                match wtr.flush() { 
                    Ok(_value) => if append {println!("added new task {tasks:?}")},
                    Err(error) => eprintln!("failed to write new task, {error}")
                };
            }
            Err(error) => {
                let path_as_str = path.as_path().to_str().unwrap();
                eprintln!("file write error, {error}, {path_as_str}");
            }
        }
    }
}