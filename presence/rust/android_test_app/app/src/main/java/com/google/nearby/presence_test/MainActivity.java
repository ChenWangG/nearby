package com.google.nearby.presence_test;

import static android.Manifest.permission.BLUETOOTH_ADVERTISE;
import static android.Manifest.permission.BLUETOOTH_SCAN;
import static android.Manifest.permission.BLUETOOTH_CONNECT;
import static android.Manifest.permission.BLUETOOTH;

import android.Manifest.permission;
import android.app.Activity;
import android.content.Intent;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.RadioButton;
import androidx.annotation.NonNull;

public class MainActivity extends Activity {

  static final String TAG = "PresenceTest";
  private static final String POST_NOTIFICATION = "android.permission.POST_NOTIFICATIONS";
  private static final int REQUEST_CODE = 123;

  private Button broadcastButton;
  private Button discoverButton;

  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_main);
    broadcastButton = findViewById(R.id.broadcast_role);
    discoverButton = findViewById(R.id.discovery_role);

    // Default to test Presence V1.
    ((RadioButton) findViewById(R.id.radio_presence_v1)).setChecked(true);
    setBroadCastScanActivities(PresenceAdvertiserActivity.class, PresenceScannerActivity.class);

    requestPermissions(
        new String[]{ BLUETOOTH_ADVERTISE, BLUETOOTH_SCAN,  BLUETOOTH_CONNECT, BLUETOOTH },
        REQUEST_CODE);
  }

  // Listener for radio buttons.
  public void onRadioButtonClicked(View view) {
    Log.d(TAG, "onRadioButtonClicked: ");
    if (!((RadioButton) view).isChecked()) {
      return;
    }
    if (view.getId() == R.id.radio_presence_v1) {
      setBroadCastScanActivities(
          PresenceAdvertiserActivity.class, PresenceScannerActivity.class);
    } else if (view.getId() == R.id.radio_ble_api) {
        setBroadCastScanActivities(
            BleAdvertiserActivity.class, ScannerActivity.class);
    } else if (view.getId() == R.id.radio_ble_rust_api) {
      setBroadCastScanActivities(
          BleAdvertiserActivity.class, BleRustScannerActivity.class);
    }
  }
  @Override
  public void onRequestPermissionsResult(
      int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
    Log.i(TAG, "request permission results:");
    if (requestCode == REQUEST_CODE) {
      for (int i = 0; i < permissions.length; i++) {
        if (grantResults[i] == PackageManager.PERMISSION_GRANTED) {
          Log.i(TAG, "Permission granted for: " + permissions[i]);
        } else {
          Log.e(TAG, "Permission denied for: " + permissions[i]);
        }
      }
    }
  }

  private void setBroadCastScanActivities(Class<?> advertiser, Class<?> scanner) {
    Log.d(TAG, "Test Presence V1");
    broadcastButton.setVisibility(View.VISIBLE);
    broadcastButton.setOnClickListener(v -> startActivity(
        new Intent(MainActivity.this, advertiser)));
    discoverButton.setOnClickListener(v -> startActivity(
        new Intent(MainActivity.this, scanner)));
  }
}
