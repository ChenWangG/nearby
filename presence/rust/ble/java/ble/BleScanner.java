// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble;


import com.google.nearby.ble.BleWrapper.ScanCallback;

// Counterpart of Rust BleScanner for FFI.
public class BleScanner {
  // Static method called from Rust to return a Ble instance.
  public static BleScanner build() {
    return new BleScanner();
  }

  public void start(long callbackPtr) {
    System.out.println("[Java][BleScanner] start.");

    ScanCallback callback = new ScanCallback() {
      @Override
      public void onScanResult(int callbackType, ScanResult result) {
        System.out.println("[Java][BleScanner] onScanResult called.");
        BleScanner.onScanResult(rustCallbackPtr, 1);

      }
      private final long rustCallbackPtr = callbackPtr;
    };
    bleWrapper.startScan(callback);
  }

  public void stop(long callbackPtr) {
    bleWrapper.stopScan(callbackPtr);
  }

  /* ========== Native methods implemented in Rust. ========== */
  // Callback to Rus to deliver a scan result.
  private static native void onScanResult(long callbackPtr, long result);

  private final BleWrapper bleWrapper = new BleWrapper();
}