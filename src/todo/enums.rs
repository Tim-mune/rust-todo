use std::fmt;
pub enum TaskStatus {
    DONE,
    PENDING
    }
    // impl TaskStatus {
    //     pub fn stringify(&self) -> String {
    //     match &self {
    //     &Self::DONE => {"DONE".to_string()},
    //     &Self::PENDING => {"PENDING".to_string()}
    //     }
    //     }
    //     }
    impl fmt::Display for TaskStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                &Self::DONE=>{write!(f,"Done with the task")},
                &Self::PENDING=>{write!(f,"Task is pending")}
            }
        }
    }
