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

#[derive(Clone, Debug)]
pub struct ScanResult {
    service_data: Vec<u8>,
}

impl ScanResult {
    pub fn new(service_data: Vec<u8>) -> Self {
        Self{ service_data }
    }

    pub fn service_data(&self) -> &Vec<u8> {
        &self.service_data
    }
}