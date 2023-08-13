// to do app
mod todo;
use todo::enums;
fn main() {
    let done_task=enums::TaskStatus::DONE;
    print!("{}",done_task)
}
