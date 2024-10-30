pub mod ble_scan_provider;

pub trait ScanProvider<R> {
    async fn set_request(&self, request: ScanRequest);
    // The result type is decided by the underlying media such as BLE o mDNS.
    async fn on_result(&self, result: R);
    async fn stop(&mut self);
}

pub struct ScanRequest {
    pub(crate) priority: i32,
}

pub struct ScanResult {
    service_data: Vec<u8>,
}