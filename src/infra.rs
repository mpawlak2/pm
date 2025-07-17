use super::pomo;

pub struct PomodoroRepository;

impl PomodoroRepository {
    pub fn new() -> PomodoroRepository {
        PomodoroRepository {}
    }

    pub fn save(&self, pomodoro: &pomo::Pomodoro) {
        println!("Saving pomodoro with id {}", pomodoro.id());
    }
}
