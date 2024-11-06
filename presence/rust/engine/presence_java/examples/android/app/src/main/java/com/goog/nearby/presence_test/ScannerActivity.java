package com.goog.nearby.presence_test;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.bluetooth.le.ScanResult;
import android.content.Context;

import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

import android.bluetooth.le.BluetoothLeScanner;
import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.ScanCallback;

import java.text.DateFormat;
import java.util.Date;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

public class ScannerActivity extends Activity {

  private BluetoothLeScanner mBtLeScanner;
  private Button scanButton;
  private boolean isScanning = false;
  private TextView textView;

  // Permission already required in MainActivity.
  @SuppressLint("MissingPermission")
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
    scanButton.setOnClickListener(
        v -> {
          textView.setVisibility(View.VISIBLE);
          if (isScanning) {
            isScanning = false;

          } else {
            try {
              mBtLeScanner.startScan(new ScanCallback() {
                @Override
                public void onScanResult(int callbackType, ScanResult result) {
                  log("BLE onScanResult.");
                }
              });
              log("Succeeded to start BLE scan.");
            } catch (Exception e) {
              log("Failed to start BLE scan.");
            }
            isScanning = true;
          }
          showUi();
        });
  }

  private void showUi() {
    scanButton.setText(isScanning ? "Stop Scan" : "Start Scan");
  }

  protected void log(String log) {
    getMainExecutor()
        .execute(
            () -> {
              StringBuilder stringBuilder =
                  new StringBuilder()
                      .append(DateFormat.getDateTimeInstance().format(new Date()))
                      .append(": ")
                      .append(log)
                      .append("\n")
                      .append("\n");
              textView.append(stringBuilder.toString());
            });
  }
}
