pub mod db {
    use crate::{comparible_task::comparible_task::ComparibleTask, task::task::Task};
    extern crate fs2;

    use fs2::FileExt;
    use std::fs;
    use std::path::PathBuf;
    use dirs::home_dir;
    use std::io::{Read, Write, Seek, SeekFrom};

    const APP_DATA: &str = ".taskman/";
    const ARCHIVE: &str = "archive.csv";
    const ACTIVE_TASKS: &str = "tasks.csv";
    const ID_SEQUENCE: &str = "id_sequence.txt";

    pub fn write_task(task: Task) {
        let tasks = vec![task];
        write_tasks(get_active_tasks_path(), tasks, true);
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
        let path = get_active_tasks_path();

        let tasks: Vec<Task> = get_raw_tasks(path);
        let mut comp_tasks: Vec<ComparibleTask> = Task::make_comparible(tasks);
        let return_size = usize::min(total_to_get, comp_tasks.len());
        return comp_tasks
            .drain(..return_size)
            .collect();
    }

    pub fn mark_complete(task_ids: Vec::<usize>) {
        let mut todo = Vec::<Task>::new();
        let mut completed = Vec::<Task>::new();

        for t in get_raw_tasks(get_active_tasks_path()) {
            match task_ids.contains(t.get_id()) {
                true => completed.push(t.complete()),
                false => todo.push(t)
            }
        }

        write_tasks(get_archive_path(), completed, true);
        write_tasks(get_active_tasks_path(), todo, false);
    }

    pub fn delete_tasks(task_ids: Vec::<usize>) {
        let tasks = get_raw_tasks(get_active_tasks_path())
            .into_iter()
            .filter(|t| !task_ids.contains(t.get_id()))
            .collect();
        
        write_tasks(get_active_tasks_path(), tasks, false);
    }

    pub fn get_unique_id() -> usize {
        let mut sequence_file = std::fs::OpenOptions::new()
            .create(false)
            .write(true)
            .read(true)
            .open(get_path(ID_SEQUENCE))
            .unwrap();
        sequence_file.lock_exclusive().unwrap();

        let mut buffer = Vec::<u8>::new();
        sequence_file.read_to_end(&mut buffer).unwrap();
        
        let mut arr = [0; 8]; 
        arr.copy_from_slice(&buffer[0..buffer.len()]);
        let new_id = usize::from_ne_bytes(arr); 
        let next_id = new_id + 1;

        sequence_file.seek(SeekFrom::Start(0)).unwrap();
        sequence_file.write_all(&next_id.to_ne_bytes()).unwrap();
        sequence_file.unlock().unwrap();
        return new_id;
    }

    fn get_active_tasks_path() -> PathBuf {
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