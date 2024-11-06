package com.goog.nearby.presence_test;

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

public class ScannerActivity extends Activity {
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
