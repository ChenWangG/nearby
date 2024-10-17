use async_std::task;
use futures::executor::block_on;

fn start() -> async_std::task::JoinHandle<()> {
    task::spawn(async move { assert_eq!(1, 1); })
}
async fn run() {
    // task::spawn(async move { assert_eq!(1, 1); }).await;
    start().await
}
#[test]
fn test_event_poller() {
    assert_eq!(1, 1);
    block_on(run());
}