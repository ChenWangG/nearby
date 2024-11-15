package com.google.nearby.presence_test;

import android.annotation.SuppressLint;
import com.google.nearby.presence.Presence;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class PresenceScannerActivity extends ScannerActivity {
  private ExecutorService executor;
  private Presence presence;

  @SuppressLint({"SetTextI18n"})
  @Override
  protected void stopScan() {
    // mBtLeScanner.stopScan(bleCallback);
    textView.setText("BLE Scan stopped");
    presence.stop();
    executor.shutdown();
    log(String.valueOf(presence.testNdk()));
  }

  @Override
  protected void startScan() {
    Presence.Callbacks callbacks = result -> log("onDiscovery");
    presence = new Presence(callbacks);
    executor = Executors.newSingleThreadExecutor();
    presence.start(executor);
    presence.setRequest();
    try {
      // mBtLeScanner.startScan(bleCallback);
      log("Succeeded to start BLE scan.");
    } catch (Exception e) {
      log("Failed to start BLE scan.");
    }
  }
}
