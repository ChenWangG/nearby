package com.google.nearby.presence_test;

import android.Manifest.permission;
import android.annotation.SuppressLint;
import android.content.pm.PackageManager;
import com.google.nearby.ble.Constants;

import static com.google.nearby.presence_test.MainActivity.TAG;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.le.AdvertiseCallback;
import android.bluetooth.le.AdvertiseData;
import android.bluetooth.le.AdvertiseSettings;
import android.bluetooth.le.BluetoothLeAdvertiser;
import android.os.ParcelUuid;
import android.util.Log;
import androidx.annotation.Nullable;

public class BleAdvertiserActivity extends AdvertiserActivity {

  @Nullable
  private final BluetoothLeAdvertiser leAdvertiser;

  @SuppressLint({"MissingPermission"})
  public BleAdvertiserActivity() {
    super();
    BluetoothAdapter bluetoothAdapter = BluetoothAdapter.getDefaultAdapter();
    if (bluetoothAdapter == null) {
      String errorMsg = "Failed to get Bluetooth Adapter.";
      Log.e(TAG, errorMsg);
      log(errorMsg);
      leAdvertiser = null;
      return;
    }
    bluetoothAdapter.setName("PresenceTest");
    leAdvertiser = bluetoothAdapter.getBluetoothLeAdvertiser();
  }

  @Override
  public void start() {
    Log.i(TAG, "start BLE advertise.");
    if (leAdvertiser != null) {
      AdvertiseSettings settings =
          new AdvertiseSettings.Builder()
              .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
              .setConnectable(false)
              .build();

      AdvertiseData advertiseData =  new AdvertiseData.Builder()
          .addServiceData(Constants.PRESENCE_SERVICE_DATA_UUID, getServiceData())
          .build();

      AdvertiseData scanResponse = new AdvertiseData.Builder().build();
      try {
        leAdvertiser.startAdvertising(
            settings, advertiseData, scanResponse, new AdvertiseCallback() {
              @Override
              public void onStartSuccess(AdvertiseSettings settingsInEffect) {
                super.onStartSuccess(settingsInEffect);
                Log.i(TAG, "Succeeded to start advertising.");
                // TODO: hook log to AdvertiserActivity.
                // log("Succeeded to start advertising.");
              }
            });
      } catch (NullPointerException | IllegalStateException | SecurityException e) {
        Log.e(TAG, "Failed to start broadcast with Exception: " + e.toString());
      }
    }
  }

  private byte[] getServiceData() {
    return new byte[] {
        0b00000001,
        0b00000010,
        0b00000011,
    };
  }
}
