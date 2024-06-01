pub mod db {
    use crate::{comparible_task::comparible_task::ComparibleTask, task::task::Task};
    use std::fs;
    use dirs::home_dir;

    const APP_DATA: &str = ".taskman/tasks.csv";

    pub fn write_task(task: Task) {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        match fs::OpenOptions::new().write(true).append(true).open(path.as_path()) {
            Ok(file) => {
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(file);
                wtr.serialize(&task).unwrap();
                match wtr.flush() { 
                    Ok(_value) => println!("added new task {:?}", task),
                    Err(error) => eprintln!("failed to write new task, {error}")
                };
            }
            Err(error) => {
                let path_as_str = path.as_path().to_str().unwrap();
                eprintln!("file write error, {error}, {path_as_str}");
            }
        }
    }

    pub fn get_tasks(total_to_get: usize) -> Vec::<ComparibleTask> {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        let file = fs::File::open(path).unwrap();
        let tasks: Vec::<Task> = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file)
            .deserialize()
            .map(|record| record.unwrap())
            .collect();

        let return_size = usize::min(total_to_get, tasks.len());

        return Task::make_comparible(tasks)
            .drain(..return_size)
            .collect();
    }
}