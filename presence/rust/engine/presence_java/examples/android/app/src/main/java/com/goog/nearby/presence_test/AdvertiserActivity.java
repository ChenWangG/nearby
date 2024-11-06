package com.goog.nearby.presence_test;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.content.Context;

import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;


import java.text.DateFormat;
import java.util.Collections;
import java.util.Date;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

/** AdertiserActivity call MainlineAPIs to start broadcast. */
@SuppressLint("RestrictedApi")
@SuppressWarnings("GmsCoreFirstPartyApiChecker")
public class AdvertiserActivity extends Activity {
  private static final byte[] SALT = new byte[] {102, 22};
  private static final byte[] SECRETE_ID =
      new byte[] {
        87, 9, -17, 75, 119, -65, 112, -54, 37, -39, 75, 40, -122, 71, 108, -121, -70, -1, 61, -58,
        -20, -55, 119, 57, -90, -30, -34, 17, 26, 81, 19, -2
      };
  private static final byte[] META_DATA_ENCRYPTION_KEY =
      new byte[] {-39, -55, 115, 78, -57, 40, 115, 0, -112, 86, -86, 7, -42, 68, 11, 12};
  private static final byte[] AUTHENTICITY_KEY =
      new byte[] {89, -19, 36, 80, 57, -3, -22, -111, 55, -114, 124, -15, 0, -13, -1, -111};
  private static final String DEVICE_NAME = "Pixel 6 Pro";
  private static final int BLE_MEDIUM = 1;
  private static final int PRESENCE_ACTION = 124;

  private Button advertiseButton;
  private boolean isAdertising = false;
  private TextView textView;
  private Executor executor;
  private Context context;



  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    this.executor = Executors.newSingleThreadExecutor();
    this.context = getApplicationContext();


    setContentView(R.layout.activity_advertiser);
    advertiseButton = findViewById(R.id.broadcast);
    textView = findViewById(R.id.broadcast_display);
    textView.setVisibility(View.GONE);
    textView.setMovementMethod(new ScrollingMovementMethod());
    advertiseButton.setOnClickListener(
        v -> {
          textView.setVisibility(View.VISIBLE);
          if (isAdertising) {
            isAdertising = false;

          } else {

            isAdertising = true;
          }
          showUi();
        });
  }

  private void showUi() {
    advertiseButton.setText(isAdertising ? "Stop Broadcast" : "Start Broadcast");
  }











  private void log(String log) {
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
