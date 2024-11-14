package com.google.nearby.ble;

public class BleWrapper {
  public interface ScanCallback {
    public void onScanResult (int callbackType,
        ScanResult result);
  }

  public BleWrapper() {}

  public void startScan(ScanCallback callback) {
    System.out.println("[Java][BleWrapper] startScan");
  }

  public void stopScan(long callbackPtr) {
    System.out.println("[Java][BleWrapper] stopScan");
  }
}