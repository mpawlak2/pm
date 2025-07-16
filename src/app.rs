use super::pomo;


pub fn start_pomodoro() {
    let pomodoro = pomo::Pomodoro::new();
    println!("{pomodoro:?} {}", pomodoro.start_time());
}