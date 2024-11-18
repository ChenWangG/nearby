package com.google.nearby.ble;

public class ScanResult {
  public ScanResult(byte[] serviceData) {
    this.serviceData = serviceData;
  }

  public byte[] getServiceData() {
    return serviceData;
  }
  private final byte[] serviceData;
}
