

pub mod comparible_task {
    use core::cmp::Ordering;
    use colored::Colorize;
    use serde::{Deserialize, Serialize};
    use tabled::builder::Builder;

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

        pub fn add_tasks_to_table(
            tasks: Vec::<ComparibleTask>,
            builder: & mut Builder, 
            verbose: bool
        ) {
            builder.push_record(ComparibleTask::get_cols(verbose));
            for task in tasks {
                builder.push_record(task.to_row(verbose));
            }
        }

        fn get_cols(verbose: bool) -> Vec::<String> {
            let mut res: Vec::<String> = vec!["index".to_string()];
            let mut task_cols: Vec::<String> = Task::get_cols(verbose);

            res.append(&mut task_cols);

            if verbose {
                res.push("score".to_string());
            }

            return res;
        }

        fn to_row(&self, verbose: bool) -> Vec::<String> {
            let mut res: Vec::<String> = vec![self.index.to_string()];

            let mut task_vals: Vec::<String> = self.task.as_row(verbose);

            res.append(&mut task_vals);

            if verbose {
                res.push(self.comparitor.to_string());
            }

            return res;
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