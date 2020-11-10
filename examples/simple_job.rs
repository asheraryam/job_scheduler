// #[feature(async_closure)]
extern crate job_scheduler;

// use futures::future;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let mut sched = JobScheduler::new();

    // let test_func =
    sched.add(Job::new(
        "1/5 * * * * *".parse().unwrap(),
        Box::new(|| Box::pin(async { println!("I get executed every 5 seconds!") })),
    ));

    loop {
        sched.tick().await;

        std::thread::sleep(Duration::from_millis(500));
    }
}
