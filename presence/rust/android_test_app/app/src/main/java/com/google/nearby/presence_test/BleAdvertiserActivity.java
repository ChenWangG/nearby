package com.google.nearby.presence_test;

import static com.google.nearby.presence_test.MainActivity.TAG;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.AdvertiseCallback;
import android.bluetooth.le.AdvertiseData;
import android.bluetooth.le.AdvertiseSettings;
import android.bluetooth.le.BluetoothLeAdvertiser;
import android.util.Log;
import androidx.annotation.Nullable;

public class BleAdvertiserActivity extends AdvertiserActivity {
  @Nullable
  private final BluetoothLeAdvertiser leAdvertiser;

  public BleAdvertiserActivity() {
    super();
    BluetoothAdapter  bluetoothAdapter =  BluetoothAdapter.getDefaultAdapter();
    if (bluetoothAdapter == null) {
      String errorMsg = "Failed to get Bluetooth Adapter.";
      Log.e(TAG, errorMsg);
      log(errorMsg);
      leAdvertiser = null;
      return;
    }
    leAdvertiser = bluetoothAdapter.getBluetoothLeAdvertiser();
  }

  @Override
  public void start() {
    Log.i(TAG, "start BLE advertise.");
    if (leAdvertiser != null) {
      AdvertiseSettings settings =
          new AdvertiseSettings.Builder()
              .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
              .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_MEDIUM)
              .setConnectable(true)
              .build();

      AdvertiseData advertiseData = null;

      AdvertiseData scanResponse = null;
      /*
      try {
        leAdvertiser.startAdvertising(
            settings, advertiseData, scanResponse, new AdvertiseCallback() {
              @Override
              public void onStartSuccess(AdvertiseSettings settingsInEffect) {
                super.onStartSuccess(settingsInEffect);
                log("Succeeded to start advertising.");
              }
            });
      } catch (NullPointerException | IllegalStateException | SecurityException e) {
        Log.e(TAG, "Failed to start broadcast with Exception: " + e.toString());
      }
       */
    }
  }
}
