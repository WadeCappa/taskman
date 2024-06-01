

pub mod task {
    use std::fmt::Debug;
    use chrono::{DateTime, FixedOffset};
    use chrono::Local;
    use serde::{Deserialize, Serialize};

    use crate::comparible_task::comparible_task::ComparibleTask;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Status {
        InProgress,
        Completed, 
        Queued 
    }

    impl std::fmt::Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        status: Status,
        name: String,
        desc: Option<String>,
        cost: u32,
        priority: u32,
        date_created: DateTime::<FixedOffset>,
        deadline: Option<DateTime::<FixedOffset>>,
    }

    impl Task {
        pub fn new(
            status: Status, 
            name: String, 
            desc: Option<String>,
            cost: u32, 
            priority: u32, 
            date_created: DateTime::<FixedOffset>,
            deadline: Option<DateTime::<FixedOffset>>
        ) -> Task {
            return Task {
                status: status,
                name: name,
                desc: desc,
                cost: cost,
                priority: priority,
                date_created: date_created,
                deadline: deadline
            };
        }

        pub fn as_row(&self, verbose: bool) -> String {
            let name = &self.name;
            let desc = match &self.desc {
                Some(desc) => desc, 
                None => "" 
            };
            let status = &self.status;
            let cost = &self.cost;
            let priority = &self.priority;
            let date = &self.date_created;
            let deadline = match &self.deadline {
                Some(date) => date.to_rfc2822(),
                None => String::from("")
            };

            if verbose {
                return format!("{0: <10}, {1: <10}, {2: <10}, {3: <10}, {4: <10}, {5: <10}, {6: <10}", 
                    name,
                    desc,
                    status,
                    cost,
                    priority,
                    date,
                    deadline,
                );
            } else {
                return format!("{0: <10}, {1: <10}, {2: <10}, {3: <10}, {4: <10}", 
                    name,
                    status,
                    cost,
                    priority,
                    deadline,
                );
            }
        }

        pub fn make_comparible(tasks: Vec::<Task>) -> Vec::<ComparibleTask> {
            let tasks_ref = &tasks;
            let total_prio_squared: u32 = tasks_ref 
                .into_iter()
                .map(|task: &Task| task.priority)
                .fold(0, |acc, e| acc + e.pow(2));

            let mut comp_tasks: Vec::<ComparibleTask> = tasks
                .into_iter()
                .enumerate()
                .map(|(i, task)| task.as_comparible_task(total_prio_squared, i))
                .collect();

            comp_tasks.sort();
            return comp_tasks;
        }

        fn as_comparible_task(self, total_prio_squared: u32, index: usize) -> ComparibleTask {
            let normalized: f64 = f64::from(self.priority.pow(2)) / f64::from(total_prio_squared).sqrt();
            let roi : f64 = normalized / f64::from(self.cost);
            if self.deadline.is_none() {
                return ComparibleTask::new(self, roi, index);
            } else {
                let now = Local::now();
                let calc_time = self.deadline.unwrap().signed_duration_since(now).num_minutes();
                let minutes: i64 = Ord::max(calc_time, 1);
                return ComparibleTask::new(self, roi + (60.0 / minutes as f64), index);
            }
        }
    }
}