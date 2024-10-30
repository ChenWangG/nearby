pub mod ble_scan_provider;

pub trait ScanProvider {
    async fn set_scan_request(&self);
    async fn on_scan_result(&self);
    async fn stop(&mut self);
}