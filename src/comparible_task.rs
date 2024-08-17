

pub mod comparible_task {
    use core::cmp::Ordering;
    use serde::{Deserialize, Serialize};
    use tabled::builder::Builder;

    use crate::task::task::Task;
    use crate::show_rule::show_rule::ShowRule;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ComparibleTask {
        task: Task,
        comparitor: f64
    }

    impl ComparibleTask {
        pub fn new(task: Task, comparitor: f64) -> ComparibleTask {
            return ComparibleTask {
                task, comparitor
            };
        }

        pub fn add_tasks_to_table(
            tasks: Vec::<ComparibleTask>,
            builder: & mut Builder, 
            show_rule: &ShowRule
        ) {
            builder.push_record(ComparibleTask::get_cols(show_rule));
            for task in &tasks {
                builder.push_record(task.to_row(show_rule));
            }
        }

        pub fn get_id(&self) -> i64 {
            return self.task.get_id();
        }

        fn get_cols(show_rule: &ShowRule) -> Vec::<&'static str> {
            let mut res: Vec::<&str> = vec![];
            res.append(&mut Task::get_cols(show_rule));
            res.push("score");

            return res;
        }

        fn to_row(&self, show_rule: &ShowRule) -> Vec::<String> {
            let mut task_vals: Vec::<String> = self.task.as_row(show_rule);
            task_vals.push(self.comparitor.to_string());
            return task_vals;
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