use super::base;
use super::super::enums::TaskStatus;

pub struct Pending{
    pub pending:base::Base
}

// impl Pending {
//     fn init()->Self{
//         let base=base::Base{
//             status:TaskStatus::DONE,
//             title:"my pending task".to_string()
//         };
//     return  Pending{pending:base};
//     }
// }

impl  Pending {
    fn new(title:String)->Self {
        let my_pending=base::Base{
            status:TaskStatus::DONE,
            title:title
        };
        return  Pending{pending:my_pending};
    }
}