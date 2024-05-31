

pub mod comparible_task {
    use core::cmp::Ordering;

    use crate::task::task::Task;

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

        pub fn as_string(&self) -> String {
            return format!("{0}) {1} with score of {2}", self.index, self.task.as_string(), self.comparitor);
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