

pub mod show_rule{
    #[derive(PartialEq, PartialOrd)]
    pub enum ShowRule{
        Verbose,
        Complete,
        Required,
    }

    impl ShowRule {
        pub fn from(verbose: bool, complete: bool) -> ShowRule {
            if verbose {
                return ShowRule::Verbose;
            } else if complete {
                return ShowRule::Complete;
            } else {
                return ShowRule::Required;
            }
        }
    }
}