package com.google.nearby.presence_test;

import static com.google.nearby.presence_test.MainActivity.TAG;

import android.annotation.SuppressLint;
import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;

public class AdvertiserActivity extends BtActivity {

  private Button advertiseButton;
  private boolean isAdvertising = false;
  private TextView textView;

  @SuppressLint("SetTextI18n")
  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    setContentView(R.layout.activity_advertiser);
    advertiseButton = findViewById(R.id.broadcast);
    textView = findViewById(R.id.broadcast_display);
    textView.setVisibility(View.GONE);
    textView.setMovementMethod(new ScrollingMovementMethod());
    advertiseButton.setOnClickListener(
        v -> {
          textView.setVisibility(View.VISIBLE);
          if (isAdvertising) {
            isAdvertising = false;
            advertiseButton.setText("Start Broadcast");
            stop();
          } else {
            isAdvertising = true;
            advertiseButton.setText("Stop Broadcast");
            start();
          }
        });
  }

  public void start() {
    Log.i(TAG, "start advertise.");
  }

  public void stop() {
    Log.i(TAG, "stop advertise.");
  }
}