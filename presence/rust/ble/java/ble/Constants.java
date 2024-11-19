package com.google.nearby.ble;

import android.os.ParcelUuid;
public class Constants {
  /** Presence advertisement service data uuid. */
  // TODO: change back from  34fc to 34fb.
  public static final ParcelUuid PRESENCE_SERVICE_DATA_UUID =
      ParcelUuid.fromString("0000fcf1-0000-1000-8000-00805f9b34fc");
}
