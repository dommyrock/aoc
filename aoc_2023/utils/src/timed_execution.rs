pub trait TimedExecution {
    fn timed<T, F>(func: F) -> T
    where
        F: FnOnce() -> T;
}

impl TimedExecution for std::time::Instant {
    fn timed<T, F>(func: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start_time = std::time::Instant::now();
        let result = func(); //if std::process::exit(0); bellow code = unreachable
        let elapsed = start_time.elapsed();
        println!("Elapsed: {:?}", elapsed);
        result
    }
}
