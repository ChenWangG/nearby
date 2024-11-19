package com.google.nearby.ble;

import static com.google.nearby.ble.BleScanner.TAG;

import android.annotation.SuppressLint;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.le.ScanRecord;
import android.util.Log;

public class BleWrapper implements BleWrapperInterface {

  public BleWrapper() {
    leScanner = BluetoothAdapter.getDefaultAdapter().getBluetoothLeScanner();
  }

  @SuppressLint({"MissingPermission"})
  public void startScan(ScanCallback callback) {
    android.bluetooth.le.ScanCallback leScanCallback = new android.bluetooth.le.ScanCallback() {
      @Override
      public void onScanResult(int callbackType, android.bluetooth.le.ScanResult result) {
        super.onScanResult(callbackType, result);
        ScanRecord record = result.getScanRecord();
        byte[] serviceData = null;
        if (record != null) {
          serviceData = record.getServiceData(Constants.PRESENCE_SERVICE_DATA_UUID);
          if (serviceData != null ) {
            Log.d(TAG, "Received Presence BLE service data from: " + result.getDevice().getAddress());
            callback.onScanResult(callbackType, new ScanResult(serviceData));
          }
        }
      }
    };
    leScanner.startScan(leScanCallback);
  }

  // TODO map callbackPtr to LeCallback instance.
  // Note: include callbackPtr into ScanCallback to setup the map.
  @SuppressLint({"MissingPermission"})
  public void stopScan(ScanCallback callback) {
  }

  private final BluetoothLeScanner leScanner;
}
