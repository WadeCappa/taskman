pub mod db {
    use crate::task::task::Task;
    use std::fs;
    use dirs::home_dir;
    use std::io::prelude::*;
    use std::collections::BinaryHeap;
    use std::io;

    const APP_DATA: &str = ".taskman/tasks.csv";

    pub fn write_task(task: Task) {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        match fs::OpenOptions::new().write(true).append(true).open(path.as_path()) {
            Ok(mut file) => {
                match writeln!(file, "{0}", task.as_string()) { 
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

    pub fn get_tasks(quantity: u32) -> BinaryHeap::<Task> {
        let lines = get_lines();
        // convert all to intermediary datastructure, 
        // calculate prioirty
        // return priority queue
        return BinaryHeap::new();
    }

    pub fn get_lines() -> io::Lines<io::BufReader<fs::File>> {
        let mut path = home_dir().unwrap();
        path.push(APP_DATA);
        return match fs::File::open(path) {
            Ok(file) => std::io::BufReader::new(file).lines(),
            Err(error) => panic!("{error}")
        };
    }
}