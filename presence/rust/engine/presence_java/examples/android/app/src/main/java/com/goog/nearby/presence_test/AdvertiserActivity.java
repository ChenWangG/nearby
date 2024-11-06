package com.goog.nearby.presence_test;

import android.app.Activity;
import android.content.Context;

import android.os.Bundle;
import android.text.method.ScrollingMovementMethod;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;


import java.text.DateFormat;
import java.util.Date;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

public class AdvertiserActivity extends Activity {
  private Button advertiseButton;
  private boolean isAdvertising = false;
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
          if (isAdvertising) {
            isAdvertising = false;

          } else {

            isAdvertising = true;
          }
          showUi();
        });
  }

  private void showUi() {
    advertiseButton.setText(isAdvertising ? "Stop Broadcast" : "Start Broadcast");
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