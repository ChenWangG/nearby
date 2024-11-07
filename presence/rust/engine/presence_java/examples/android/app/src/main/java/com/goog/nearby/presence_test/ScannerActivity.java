package com.goog.nearby.presence_test;

import android.annotation.SuppressLint;
import android.bluetooth.le.ScanResult;

import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.view.View;
import android.widget.Button;

import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.ScanCallback;

import com.google.nearby.presence.Presence;

class TestCallbacks implements Presence.Callbacks {
  @Override
  public void onDiscovery(long l) {

  }
}
public class ScannerActivity extends BtActivity {

  private BluetoothLeScanner mBtLeScanner;
  private ScanCallback bleCallback;
  private Button scanButton;
  private boolean isScanning = false;

  // Permission already required in MainActivity.
  @SuppressLint({"MissingPermission", "SetTextI18n"})
  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    BluetoothAdapter mBtAdapter = BluetoothAdapter.getDefaultAdapter();
    mBtLeScanner = mBtAdapter.getBluetoothLeScanner();

    setContentView(R.layout.activity_scanner);
    scanButton = findViewById(R.id.discovery);
    textView = findViewById(R.id.discovery_display);
    textView.setVisibility(View.GONE);
    textView.setMovementMethod(new ScrollingMovementMethod());

    bleCallback = new ScanCallback() {
      @Override
      public void onScanResult(int callbackType, ScanResult result) {
        super.onScanResult(callbackType, result);
        log("BLE scan callback.");
      }
    };

    scanButton.setOnClickListener(
        v -> {
          textView.setVisibility(View.VISIBLE);
          if (isScanning) {
            isScanning = false;
            scanButton.setText("Start Scan");
            mBtLeScanner.stopScan(bleCallback);
            textView.setText("BLE Scan stopped");
            Presence presence = new Presence(new TestCallbacks());
            log(String.valueOf(presence.testNdk()));

          } else {
            isScanning = true;
            scanButton.setText("Stop Scan");
            try {
              mBtLeScanner.startScan(bleCallback);
              log("Succeeded to start BLE scan.");
            } catch (Exception e) {
              log("Failed to start BLE scan.");
            }
          }
        });
  }
}
