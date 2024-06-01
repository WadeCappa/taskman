

pub mod comparible_task {
    use core::cmp::Ordering;
    use colored::Colorize;
    use serde::{Deserialize, Serialize};

    use crate::task::task::Task;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComparibleTask {
        task: Task,
        index: usize,
        comparitor: f64
    }

    impl ComparibleTask {
        pub fn new(task: Task, comparitor: f64, index: usize) -> ComparibleTask {
            return ComparibleTask {
                task, index, comparitor
            };
        }

        pub fn as_table(tasks: Vec::<ComparibleTask>, verbose: bool) -> Vec::<String>{
            return tasks.into_iter().map(|task| task.as_row(verbose)).collect();
        }

        fn as_row(&self, verbose: bool) -> String {
            let index_string = self.index.to_string().red();
            let task_string = match verbose {
                true => format!("task {0} with score {1} | ", index_string, &self.comparitor),
                false => format!("task {0} | ", index_string)
            };

            return format!("{0: <10} {1}", 
                task_string,
                self.task.as_row(verbose)
            );
        }
    }

    impl Ord for ComparibleTask {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.comparitor < other.comparitor {
                return Ordering::Less;
            } else if self.comparitor > other.comparitor {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
    }

    impl PartialOrd for ComparibleTask {
        fn partial_cmp(&self, other: &ComparibleTask) -> Option<Ordering> {
            return Some(self.cmp(other));
        }
    }

    impl Eq for ComparibleTask {
        
    }

    impl PartialEq for ComparibleTask {
        fn eq(&self, other: &ComparibleTask) -> bool {
            return self == other;
        }
    }
}