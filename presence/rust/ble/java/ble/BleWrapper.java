package com.google.nearby.ble;

import android.annotation.SuppressLint;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.BluetoothLeScanner;

public class BleWrapper {
  public interface ScanCallback {
    public void onScanResult (int callbackType,
        ScanResult result);

    public void setInner(Object inner);
    public Object getInner();
  }

  public BleWrapper() {
    leScanner = BluetoothAdapter.getDefaultAdapter().getBluetoothLeScanner();
  }

  @SuppressLint({"MissingPermission"})
  public void startScan(ScanCallback callback) {
    android.bluetooth.le.ScanCallback leScanCallback = new android.bluetooth.le.ScanCallback() {
      @Override
      public void onScanResult(int callbackType, android.bluetooth.le.ScanResult result) {
        super.onScanResult(callbackType, result);
        callback.onScanResult(callbackType, new ScanResult());
      }
    };
    callback.setInner(leScanCallback);
    leScanner.startScan(leScanCallback);
  }

  @SuppressLint({"MissingPermission"})
  public void stopScan(ScanCallback callback) {
    leScanner.stopScan((android.bluetooth.le.ScanCallback)callback.getInner());
  }

  private final BluetoothLeScanner leScanner;
}
