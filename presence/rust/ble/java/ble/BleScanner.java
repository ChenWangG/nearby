// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble;


// Nearby BLE in Java Wrapping the system API.
public class BleScanner {
  // Static method called from Rust to return a Ble instance.
  public static BleScanner build() {
    return new BleScanner(1);
  }

  private BleScanner(long rust_scanner) {
    this.rustBleScanner = rust_scanner;

  }

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
  private static native long onScanResult(long rustBleScanner, long result);
  public void onScanResult() {
    
  }

  private long rustBleScanner;
}