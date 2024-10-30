pub mod ble_scan_provider;

pub trait ScanProvider {
    async fn set_request(&self, request: ScanRequest);
    async fn on_result(&self, result: ScanResult);
    async fn stop(&mut self);
}

pub struct ScanRequest {
    pub(crate) priority: i32,
}

pub struct ScanResult {
    service_data: Vec<u8>,
}