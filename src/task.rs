

pub mod task {
    use std::fmt::Debug;
    use core::cmp::Ordering;
    use chrono::{DateTime, FixedOffset};

    #[derive(Debug)]
    pub enum Completed {
        YES,
        NO
    }

    impl Completed {
        fn from_string(string: &str) -> Completed {
            match string {
                "y" => Completed::YES,
                "n" => Completed::NO,
                _ => panic!("bad input")
            }
        }
    }

    impl std::fmt::Display for Completed {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let value: char = match self {
                Completed::YES => 'y',
                Completed::NO => 'n'
            };
            write!(f, "{}", value)
        }
    }

    #[derive(Debug)]
    pub struct Task {
        completed: Completed,
        name: String,
        desc: Option<String>,
        cost: u32,
        priority: u32,
        date_created: DateTime::<FixedOffset>,
        deadline: Option<DateTime::<FixedOffset>>,
    }

    impl Task {
        pub fn new(
            completed: Completed, 
            name: String, 
            desc: Option<String>,
            cost: u32, 
            priority: u32, 
            date_created: DateTime::<FixedOffset>,
            deadline: Option<DateTime::<FixedOffset>>
        ) -> Task {
            return Task {
                completed: completed,
                name: name,
                desc: desc,
                cost: cost,
                priority: priority,
                date_created: date_created,
                deadline: deadline
            };
        }

        pub fn as_string(&self) -> String {
            return format!("{0},{1},{2},{3},{4},{5},{6}", 
                self.completed,
                self.name, 
                match &self.desc {
                    Some(desc) => desc, 
                    None => "" 
                },
                self.cost, 
                self.priority, 
                self.date_created.to_rfc3339(), 
                match self.deadline {
                    Some(date) => date.to_rfc3339(),
                    None => String::from("")
                }
            );
        }

        fn from_string(string: String) -> Task {
            let data: Vec<&str> = string.split(",").collect();

            let comp = Completed::from_string(data[0]);
            let name = String::from(data[1]);
            let desc = Some(String::from(data[2]));
            let cost = data[3].parse::<u32>().unwrap();
            let priority = data[4].parse::<u32>().unwrap();
            let created = DateTime::parse_from_rfc3339(data[5]).unwrap();
            let deadline = match data[6].is_empty() {
                true => Some(DateTime::parse_from_rfc3339(data[5]).unwrap()),
                false => None
            };
        
            return Task::new(comp, name, desc, cost, priority, created, deadline);
        }
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> Ordering {
            return self.priority.cmp(&other.priority);
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Task) -> Option<Ordering> {
            return Some(self.cmp(other));
        }
    }

    impl Eq for Task {
        
    }

    impl PartialEq for Task {
        fn eq(&self, other: &Task) -> bool {
            return self == other;
        }
    }
}

