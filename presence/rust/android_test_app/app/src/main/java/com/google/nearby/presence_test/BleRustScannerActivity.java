package com.google.nearby.presence_test;

import com.google.nearby.presence.Presence;
import java.util.concurrent.Executors;

public class BleRustScannerActivity extends ScannerActivity {
  @Override
  protected void stopScan() {
    log("stop BLE Rust scan.");
  }

  @Override
  protected void startScan() {
    log("start BLE Rust scan.");
  }
}
