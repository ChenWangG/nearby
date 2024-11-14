package com.google.nearby.ble;

public class BleWrapper {
  public interface ScanCallback {
    public void onScanResult (int callbackType,
        ScanResult result);

    public void setInner(Object inner);
    public Object getInner();
  }

  public BleWrapper() {}

  public void startScan(ScanCallback callback) {
    System.out.println("[Java][BleWrapper] startScan");
  }

  public void stopScan(ScanCallback callback) {
    System.out.println("[Java][BleWrapper] stopScan");
  }
}