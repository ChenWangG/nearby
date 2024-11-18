package com.google.nearby.ble;

interface BleWrapperInterface {
  public interface ScanCallback {
    public void onScanResult (int callbackType,
        ScanResult result);
  }

  public void startScan(ScanCallback callback);
  public void stopScan(ScanCallback callback);
}