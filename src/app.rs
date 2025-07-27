use super::infra;
use super::pomo;

pub fn start_pomodoro(container: &infra::container::Container) {
    let mut pomodoro = pomo::Pomodoro::new().with_minutes(30);
    println!(
        "{pomodoro:?} {} {}",
        pomodoro.start_time(),
        pomodoro.is_done()
    );

    pomodoro.finish("Testing Note");

    println!(
        "{pomodoro:?} {} {}",
        pomodoro.start_time(),
        pomodoro.is_done()
    );

    // save it to database
    let repository = infra::repository::PomodoroRepository::new(&container.database_connection);
    repository.save(&pomodoro);
}
