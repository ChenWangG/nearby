package com.goog.nearby.presence_test;

import android.app.Activity;
import android.widget.TextView;
import java.text.DateFormat;
import java.util.Date;

public class BtActivity extends Activity {
  protected TextView textView;
  protected void log(String log) {
    getMainExecutor()
        .execute(
            () -> {
              String stringBuilder = DateFormat.getDateTimeInstance().format(new Date())
                  + ": "
                  + log
                  + "\n"
                  + "\n";
              textView.append(stringBuilder);
            });
  }
}

