// Test Helper to pass JVM into Rust BleScanner.
// The Rust counterpart TestBleScanner is provided in examples/test_ble_scanner.rs.
//
// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble_test;

public class TestBleScanner {
  static {
    System.loadLibrary("test_ble_scanner");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long newTestBleScanner();
  private static native void start(long rustTestBleScanner);

  public TestBleScanner() {
    this.rustTestBleScanner = newTestBleScanner();
  }

  public void start() {
     System.out.println("TestBleScanner ptr: " + String.valueOf(this.rustTestBleScanner));
     start(this.rustTestBleScanner);
  }

  private final long rustTestBleScanner;
}