package com.google.nearby.presence_test;

import static android.Manifest.permission.ACCESS_FINE_LOCATION;
import static android.Manifest.permission.BLUETOOTH_ADVERTISE;
import static android.Manifest.permission.BLUETOOTH_SCAN;
import static android.Manifest.permission.UWB_RANGING;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.RadioButton;
import android.widget.RadioGroup;
import android.widget.RadioGroup.OnCheckedChangeListener;
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

    RadioButton presenceV1 = ((RadioButton) findViewById(R.id.radio_presence_v1));
    presenceV1.setOnClickListener(view -> setBroadCastScanActivities(
        AdvertiserActivity.class, PresenceScannerActivity.class
    ));
    RadioButton fastPair = ((RadioButton) findViewById(R.id.radio_presence_fast_pair));
    fastPair.setOnClickListener(view -> setBroadCastScanActivities(
        AdvertiserActivity.class, ScannerActivity.class
    ));
    // Default to test Presence V1.
    presenceV1.setChecked(true);
    setBroadCastScanActivities(AdvertiserActivity.class, PresenceScannerActivity.class);

    requestPermissions(
        new String[]{
            BLUETOOTH_ADVERTISE, BLUETOOTH_SCAN, UWB_RANGING, ACCESS_FINE_LOCATION,
            POST_NOTIFICATION
        },
        REQUEST_CODE);
  }

  @Override
  public void onRequestPermissionsResult(
      int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults) {
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
