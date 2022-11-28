use application::run;

mod application;
mod appstate;
mod grid;
mod snake;
mod point;
mod directions;

fn main() {
    match run() {
        Ok(()) => {
            println!("{}", "Program exited successfully.");
        }
        Err(message) => {
            println!("Program failure: {}", message);
        }
    }
}
