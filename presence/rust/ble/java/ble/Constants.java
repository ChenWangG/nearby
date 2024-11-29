package com.google.nearby.ble;

import android.os.ParcelUuid;
public class Constants {
  /** Presence advertisement service data uuid. */
  // TODO: The first two bytes are changeable. change ff01 when conflict.
  public static final ParcelUuid PRESENCE_SERVICE_DATA_UUID =
      ParcelUuid.fromString("0000ff01-0000-1000-8000-00805f9b34fb");
}
