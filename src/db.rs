pub mod db {
    use crate::{comparible_task::comparible_task::ComparibleTask, task::task::Task};
    use std::fs;
    use std::path::PathBuf;
    use dirs::home_dir;

    const APP_DATA: &str = ".taskman/";
    const ARCHIVE: &str = "archive.csv";
    const ACTIVE_TASKS: &str = "tasks.csv";

    pub fn write_task(task: Task) {
        let tasks = vec![task];
        write_tasks(get_active_path(), tasks, true);
    }

    pub fn get_completed_tasks(
        total_to_get: usize
    ) -> Vec::<Task> {
        let path = get_archive_path();
        let mut tasks: Vec<Task> = get_raw_tasks(path);
        let return_size = usize::min(total_to_get, tasks.len());
        return tasks.drain(..return_size).collect();
    }

    pub fn get_tasks(
        total_to_get: usize
    ) -> Vec::<ComparibleTask> {
        let path = get_active_path();

        let tasks: Vec<Task> = get_raw_tasks(path);
        let mut comp_tasks: Vec<ComparibleTask> = Task::make_comparible(tasks);
        let return_size = usize::min(total_to_get, comp_tasks.len());
        return comp_tasks
            .drain(..return_size)
            .collect();
    }

    pub fn mark_complete(task_id: usize) {
        let mut tasks = get_raw_tasks(get_active_path());
        let mut completed: Task = tasks.remove(task_id);
        completed.complete();
        write_tasks(get_archive_path(), vec![completed], true);
        write_tasks(get_active_path(), tasks, false);
    }

    pub fn delete_task(task_id: usize) {
        let mut tasks = get_raw_tasks(get_active_path());
        
        // delete
        tasks.remove(task_id);
        write_tasks(get_active_path(), tasks, false);

    }

    pub fn get_unique_id(old_tasks: Vec<ComparibleTask>) -> i64 {
        // TODO: Race condition with getting ids, can end up with duplicate primary keys
        return match (&old_tasks).into_iter()
            .map(ComparibleTask::get_id)
            .reduce(std::cmp::max) {
            Some(val) => val + 1,
            None => 0
        };
    }

    fn get_active_path() -> PathBuf {
        return get_path(ACTIVE_TASKS);
    }

    fn get_archive_path() -> PathBuf {
        return get_path(ARCHIVE);
    }

    fn get_path(file: &str) -> PathBuf {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        path.push(file);
        return path;
    }

    fn get_raw_tasks(path: PathBuf) -> Vec<Task> {
        let file = fs::File::open(path).unwrap();
        return csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .map(|record| record.unwrap())
            .collect();
    }

    fn write_tasks(path: PathBuf, tasks: Vec<Task>, append: bool) {
        match fs::OpenOptions::new()
            .truncate(!append)
            .create(true)
            .write(true)
            .append(append)
            .open(path.as_path()) {
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