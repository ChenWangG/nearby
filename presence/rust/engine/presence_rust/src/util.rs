use std::future::Future;
use tokio::runtime::Builder;

pub fn async_block_on(future: impl Future<Output = ()>) {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(future);
}