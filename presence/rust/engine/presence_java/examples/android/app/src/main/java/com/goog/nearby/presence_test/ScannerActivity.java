package com.goog.nearby.presence_test;



import android.annotation.SuppressLint;
import android.app.Activity;
import android.content.Context;

import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

import java.text.DateFormat;
import java.util.Date;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

/** ScannerActivity call MainlineAPIs to start discovery. */
@SuppressLint("RestrictedApi")
@SuppressWarnings("GmsCoreFirstPartyApiChecker")
public class ScannerActivity extends Activity {
  protected static final int PATH_LOSS = 300;
  protected static final int PRESENCE_ACTION = 124;
  private static final byte[] PUBLIC_KEY =
      new byte[] {
        48, 89, 48, 19, 6, 7, 42, -122, 72, -50, 61, 2, 1, 6, 8, 42, -122, 72, -50, 61, 3, 1, 7, 3,
        66, 0, 4, -56, -39, -92, 69, 0, 52, 23, 67, 83, -14, 75, 52, -14, -5, -41, 48, -35, -22,
        -83, 31, 42, -39, 102, -13, 22, -73, -73, 86, 30, -96, -84, -13, 4, 122, 104, 97, 32, -44,
        -65, 64, 91, -109, -45, -35, -56, 55, -79, 47, -85, 27, -96, -119, -82, -80, 100, 29, 94,
        123, 41, -119, -25, 1, -112, 112
      };
  private static final byte[] ENCRYPTED_METADATA_BYTES =
      new byte[] {
        -44, -25, -95, -124, -7, 90, 116, -8, 7, -120, -23, -22, -106, -44, -19, 61, -10, -81, 40,
        -18, 39, 29, 78, 108, -11, -39, 85, -30, 64, -99, 102, 65, 37, -42, 114, -37, 126, -93, -94,
        88, -112, 8, -75, -53, 23, -16, -104, 67, 49, 48, -53, 73, -109, 44, -23, -11, -99, -89,
        -118, -61, -37, -104, 60, 105, 115, 1, 56, -89, -107, -45, -116, -1, -25, 84, -127, -26,
        -19, -128, 81, 11, 92, 77, -58, 82, 122, 123, 31, -87, -57, 70, 23, -81, 7, 2, -50, 37,
        -114, -83, 74, 124, -68, -98, 47, 91, 9, 48, -67, 41, -7, -97, 78, 66, -65, 58, -4, 46, 36,
        -4, -46, -30, -85, -50, 100, 46, -66, -128, 7, 66, 9, 88, 95, 12, -13, 81, -91, -126
      };

  private static final byte[] METADATA_ENCRYPTION_KEY_TAG =
      new byte[] {-68, 116, 31, -83, 98, -67, 72, -56};

  private static final byte[] SECRETE_ID =
      new byte[] {
        87, 9, -17, 75, 119, -65, 112, -54, 37, -39, 75, 40, -122, 71, 108, -121, -70, -1, 61, -58,
        -20, -55, 119, 57, -90, -30, -34, 17, 26, 81, 19, -2
      };

  private static final byte[] AUTHENTICITY_KEY =
      new byte[] {89, -19, 36, 80, 57, -3, -22, -111, 55, -114, 124, -15, 0, -13, -1, -111};



  private Executor executor;
  private Context context;

  private Button scanButton;
  private boolean isScanning = false;
  private TextView textView;

  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    this.executor = Executors.newSingleThreadExecutor();
    this.context = getApplicationContext();

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

            isScanning = true;
          }
          showUi();
        });
  }

  private void showUi() {
    scanButton.setText(isScanning ? "Stop Scan" : "Start Scan");
  }







  protected Context getContext() {
    return context;
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
