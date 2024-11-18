package com.google.nearby.ble;

import java.lang.Thread;

public class BleWrapper implements BleWrapperInterface {
  public BleWrapper() {}

  public void startScan(ScanCallback callback) {
    System.out.println("[Java][BleWrapper] startScan");
    new Thread() {
      public void run() {
        System.out.println("[Java][BleWrapper] startScan callback.");
        callback.onScanResult(1, new ScanResult());
      }
    }.start();
  }

  public void stopScan(ScanCallback callback) {
    System.out.println("[Java][BleWrapper] stopScan");
  }
}