

pub mod task {
    use std::fmt::Debug;
    use chrono::{DateTime, FixedOffset};
    use chrono::Local;
    use serde::{Deserialize, Serialize};

    use crate::comparible_task::comparible_task::ComparibleTask;
    use crate::show_rule::show_rule::ShowRule;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Task {
        id: usize,
        name: String,
        desc: Option<String>,
        cost: u32,
        priority: u32,
        date_created: DateTime::<FixedOffset>,
        date_completed: Option<DateTime::<FixedOffset>>,
        deadline: Option<DateTime::<FixedOffset>>,
    }

    impl Task {
        pub fn new(
            id: usize,
            name: String, 
            desc: Option<String>,
            cost: u32, 
            priority: u32, 
            date_created: DateTime::<FixedOffset>,
            date_completed: Option<DateTime::<FixedOffset>>,
            deadline: Option<DateTime::<FixedOffset>>
        ) -> Task {
            return Task {
                id, name, desc, cost, priority, date_created, date_completed, deadline
            };
        }

        fn optional_time_as_string(time: &Option<DateTime::<FixedOffset>>) -> String {
            return match time {
                Some(date) => date.to_rfc2822(),
                None => String::from("")
            };
        }

        pub fn complete(mut self) -> Task {
            self.date_completed = Some(DateTime::from(Local::now()));
            return self;
        }

        pub fn as_row(&self, show_rule: &ShowRule) -> Vec::<String> {
            let id = self.id.to_string();
            let name = self.name.to_string();
            let desc : String = match &self.desc {
                Some(v) => v.to_string(), 
                None => "".to_string() 
            };
            let cost = self.cost.to_string();
            let priority = self.priority.to_string();
            let date = self.date_created.to_rfc2822();
            let deadline = Task::optional_time_as_string(&self.deadline);
            let completed = Task::optional_time_as_string(&self.date_completed);

            let cols = Task::get_cols(show_rule);
            let mut res: Vec::<String> = vec![];

            for col in cols.into_iter() {
                match col {
                    "id" => res.push(id.to_string()),
                    "task" => res.push(name.to_string()),
                    "desc" => res.push(desc.to_string()),
                    "cost" => res.push(cost.to_string()),
                    "priority" => res.push(priority.to_string()),
                    "deadline" => res.push(deadline.to_string()),
                    "completed"  => res.push(completed.to_string()),
                    "created" => res.push(date.to_string()),
                    _ => unreachable!("unrecognized col")
                }
            }

            return res;
        }

        pub fn get_cols(show_rule: &ShowRule) -> Vec::<&'static str> {
            let col_to_print: Vec<(&str, ShowRule)> = Vec::from([
                ("id", ShowRule::Required),
                ("task", ShowRule::Required), 
                ("desc", ShowRule::Verbose),
                ("cost", ShowRule::Required),
                ("priority", ShowRule::Required), 
                ("completed", ShowRule::Complete), 
                ("deadline", ShowRule::Required), 
                ("created", ShowRule::Verbose) 
            ]);

            return col_to_print
                .into_iter()
                .filter(|(_key, val)| val >= show_rule)
                .map(|(key, _val)| key)
                .collect();
        }

        pub fn make_comparible(tasks: Vec::<Task>) -> Vec::<ComparibleTask> {
            let tasks_ref = &tasks;
            let total_prio_squared: u32 = tasks_ref 
                .into_iter()
                .map(|task: &Task| task.priority)
                .fold(0, |acc, e| acc + e.pow(2));

            let mut comp_tasks: Vec::<ComparibleTask> = tasks
                .into_iter()
                .map(|task| task.as_comparible_task(total_prio_squared))
                .collect();

            comp_tasks.sort();
            comp_tasks.reverse();
            return comp_tasks;
        }

        pub fn get_id(&self) -> &usize {
            return &self.id;
        }

        fn as_comparible_task(self, total_prio_squared: u32) -> ComparibleTask {
            let normalized: f64 = f64::from(self.priority.pow(2)) / f64::from(total_prio_squared).sqrt();
            let roi : f64 = normalized / f64::from(self.cost);
            if self.deadline.is_none() {
                return ComparibleTask::new(self, roi);
            } else {
                let now = Local::now();
                let calc_time = self.deadline.unwrap().signed_duration_since(now).num_minutes();
                let minutes: i64 = Ord::max(calc_time, 1);
                let comparitor = roi + (f64::from(self.cost) / minutes as f64);
                return ComparibleTask::new(self, comparitor);
            }
        }
    }
}