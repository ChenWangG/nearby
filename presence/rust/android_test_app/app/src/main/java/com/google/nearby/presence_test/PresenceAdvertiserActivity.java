package com.google.nearby.presence_test;
public class PresenceAdvertiserActivity extends BleAdvertiserActivity {
  @Override
  protected byte[] getServiceData() {
    return new byte[] {
        0b00100000,
        0b00000000,
        0b00000010,
        0b00010101,
        0b00000110,
    };
  }
}
