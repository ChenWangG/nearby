// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble;


// Counterpart of Rust BleScanner for FFI.
public class BleScanner {
  // Static method called from Rust to return a Ble instance.
  public static BleScanner build() {
    return new BleScanner();
  }

  public void start(long callbackPtr) {
    System.out.println("[Java][BleScanner] start.");
  }

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
  private static native void onScanResult(long callbackPtr, long result);

  private final BleWrapper bleWrapper = new BleWrapper();
}