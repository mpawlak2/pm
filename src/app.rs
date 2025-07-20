use super::infra;
use super::pomo;

pub fn start_pomodoro(container: &infra::container::Container) {
    let mut pomodoro = pomo::Pomodoro::new().with_minutes(30);
    println!(
        "{pomodoro:?} {} {}",
        pomodoro.start_time(),
        pomodoro.is_time_up()
    );

    pomodoro.finish("Testing Note");

    println!(
        "{pomodoro:?} {} {}",
        pomodoro.start_time(),
        pomodoro.is_time_up()
    );

    // save it to database
    let repository = infra::PomodoroRepository::new(&container.database_connection);
    repository.save(&pomodoro);
}
