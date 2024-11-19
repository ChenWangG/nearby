package com.google.nearby.ble;

public class ScanResult {
  public ScanResult(byte[] serviceData) {
    this.serviceData = serviceData;
  }

  public byte[] getServiceData() {
    return serviceData;
  }
  private final byte[] serviceData;

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
   public static native long toRustScanResult(byte[] serviceData);
}
