package com.google.nearby.presence_test;

import com.google.nearby.ble_test.TestBleScanner;

public class BleRustScannerActivity extends ScannerActivity {

  TestBleScanner test_scanner;
  public BleRustScannerActivity() {
    super();
    test_scanner = new TestBleScanner();
  }
  @Override
  protected void stopScan() {
    log("stop BLE Rust scan.");
  }

  @Override
  protected void startScan() {
    log("start BLE Rust scan.");
    test_scanner.start();
  }
}
