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
import java.nio.charset.StandardCharsets;
import java.util.Arrays;

public class BleAdvertiserActivity extends AdvertiserActivity {

  @Nullable
  private final BluetoothLeAdvertiser leAdvertiser;
  private final AdvertiseCallback callback;

  @SuppressLint({"MissingPermission"})
  public BleAdvertiserActivity() {
    super();
    BluetoothAdapter bluetoothAdapter = BluetoothAdapter.getDefaultAdapter();
    if (bluetoothAdapter == null) {
      String errorMsg = "Failed to get Bluetooth Adapter.";
      Log.e(TAG, errorMsg);
      log(errorMsg);
      leAdvertiser = null;
      callback = null;
      return;
    }
    bluetoothAdapter.setName("PresenceTest");
    leAdvertiser = bluetoothAdapter.getBluetoothLeAdvertiser();
    callback = new AdvertiseCallback() {
      @Override
      public void onStartSuccess(AdvertiseSettings settingsInEffect) {
        super.onStartSuccess(settingsInEffect);
        Log.i(TAG, "Succeeded to start advertising.");
      }
      @Override
      public void onStartFailure(int errorCode) {
        super.onStartFailure(errorCode);
        Log.e(TAG, "Failed to start advertising.");
      }
    };
  }

  @Override
  public void start() {
    Log.i(TAG, "start BLE advertise.");
    AdvertiseSettings settings =
        new AdvertiseSettings.Builder()
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
            .setConnectable(false)
            .build();

    // TODO: switch back to getServiceData().
    AdvertiseData advertiseData =  new AdvertiseData.Builder()
        .addServiceData(Constants.PRESENCE_SERVICE_DATA_UUID, getTestData())
        .build();

    AdvertiseData scanResponse = new AdvertiseData.Builder().build();
    try {
      assert leAdvertiser != null;
      leAdvertiser.startAdvertising(settings, advertiseData, scanResponse, callback);
      log("Succeeded to start BLE advertisement with service data: " +
          advertiseData.getServiceData().toString());
    } catch (NullPointerException | IllegalStateException | SecurityException e) {
      log("Failed to start BLE advertisement.");
      Log.e(TAG, "Failed to start broadcast with Exception: " + e.toString());
    }
  }


  @Override
  @SuppressLint({"MissingPermission"})
  public void stop() {
    Log.i(TAG, "stop BLE advertisement.");
    assert leAdvertiser != null;
    leAdvertiser.stopAdvertising(callback);
    log("Stopped BLE advertisement.");
  }
  private byte[] getTestData() {
    // return "abcefghijklmnopqrst".getBytes(StandardCharsets.UTF_8);
    // Max bytes to be received by iOS is 24.
    byte[] bytes = new byte[24];
    Arrays.fill( bytes, (byte) 3 );
    return bytes;
  }
  private byte[] getServiceData() {
    return new byte[] {
        0b00100000,
        0b00000000,
        0b00000010,
        0b00010101,
        0b00000110,
    };
  }
}
