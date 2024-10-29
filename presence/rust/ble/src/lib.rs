pub struct BleScanRequest;
pub struct BleScanResult;

pub trait ScanCallback {
  fn on_update(&self, result: BleScanResult);
}
pub trait Scanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback);
}

pub struct BleScanner;

impl Scanner for BleScanner {
    fn start(&self, request: BleScanRequest, callback: impl ScanCallback) {
        println!("BleScanner start.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
