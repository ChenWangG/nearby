package com.goog.nearby.presence_test;

import static android.Manifest.permission.ACCESS_FINE_LOCATION;
import static android.Manifest.permission.BLUETOOTH_ADVERTISE;
import static android.Manifest.permission.BLUETOOTH_SCAN;
import static android.Manifest.permission.UWB_RANGING;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.content.Intent;
import android.content.pm.PackageInfo;
import android.content.pm.PackageManager;
import android.content.pm.PackageManager.NameNotFoundException;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.RadioButton;

/** Shows options for role selection, and allows to activate broadcast or discovery. */
@SuppressLint("RestrictedApi")
@SuppressWarnings("GmsCoreFirstPartyApiChecker")
public class MainActivity extends Activity {

  static final String TAG = "MainlineTest";
  private static final String POST_NOTIFICATION = "android.permission.POST_NOTIFICATIONS";
  private static final int REQUEST_CODE = 123;

  private Button broadcastButton;
  private Button discoverButton;

  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    Log.d(TAG, "Check Mainline Tethering module version code.");
    PackageInfo packageInfo = null;
    try {
      packageInfo =
          getApplicationContext()
              .getPackageManager()
              .getPackageInfo("com.google.android.tethering", PackageManager.MATCH_APEX);
    } catch (NameNotFoundException e1) {
      try {
        packageInfo =
            getApplicationContext()
                .getPackageManager()
                .getPackageInfo("com.android.tethering", PackageManager.MATCH_APEX);
      } catch (NameNotFoundException e2) {
        Log.d(TAG, "Tethering package not found.");
      }
    }
    if (packageInfo != null) {
      Log.d(TAG, "Tethering package version code: " + packageInfo.getLongVersionCode());
    }

    setContentView(R.layout.activity_main);
    broadcastButton = findViewById(R.id.broadcast_role);
    discoverButton = findViewById(R.id.discovery_role);

    ((RadioButton) findViewById(R.id.radio_presence_v1)).setChecked(true);
    testPresenceV1();

    requestPermissions(
        new String[] {
          BLUETOOTH_ADVERTISE, BLUETOOTH_SCAN, UWB_RANGING, ACCESS_FINE_LOCATION, POST_NOTIFICATION
        },
        REQUEST_CODE);
  }

  public void onRadioButtonClicked(View view) {
    if (!((RadioButton) view).isChecked()) {
      return;
    }

    
  }

  @Override
  public void onRequestPermissionsResult(
      int requestCode, String[] permissions, int[] grantResults) {}

  private void testPresenceV1() {
    Log.d(TAG, "Test Presence V1");
    broadcastButton.setVisibility(View.VISIBLE);
    broadcastButton.setOnClickListener(
        v -> startActivity(new Intent(MainActivity.this, AdvertiserActivity.class)));
    discoverButton.setOnClickListener(
        v -> startActivity(new Intent(MainActivity.this, ScannerActivity.class)));
  }


}
