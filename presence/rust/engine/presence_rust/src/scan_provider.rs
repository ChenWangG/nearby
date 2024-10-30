pub mod ble_scan_provider;

pub trait ScanProvider<R> {
    async fn set_request(&self, request: ScanRequest);
    // on_result() is called by the underlying media (BLE or mDNS) with different result type R.
    async fn on_result(&self, result: R);
    async fn stop(&mut self);
}

#[derive(Clone)]
pub struct ScanRequest {
    pub(crate) priority: i32,
}

pub struct ScanResult {
    service_data: Vec<u8>,
}