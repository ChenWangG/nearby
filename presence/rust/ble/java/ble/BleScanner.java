// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble;


// Nearby BLE in Java Wrapping the system API.
public class BleScanner {
  // Static method called from Rust to return a Ble instance.
  public static BleScanner build() {
    return new BleScanner();
  }

  private BleScanner() {
  }

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
  private static native long onScanResult(long rustBleScanner, long result);

  public void start() {
  }

}