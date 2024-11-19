package com.google.nearby.ble;

public class ScanResult {
  public ScanResult(byte[] serviceData) {
    this.serviceData = serviceData;
  }

  public byte[] getServiceData() {
    return serviceData;
  }

  public long toRustScanResult() {
    return toRustScanResult(serviceData);
  }

  private final byte[] serviceData;

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
   private static native long toRustScanResult(byte[] serviceData);
}
