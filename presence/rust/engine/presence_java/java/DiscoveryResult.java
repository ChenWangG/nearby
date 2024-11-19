package com.google.nearby.presence;

public class DiscoveryResult {
  public DiscoveryResult(byte[] serviceData) {
    this.serviceData = serviceData;
  }

  public byte[] serviceData() {
    return serviceData;
  }

  // Static method called from Rust to return a Ble instance.
  public static DiscoveryResult build(byte[] serviceData) {
    return new DiscoveryResult(serviceData);
  }

  private byte[] serviceData;
}