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

  public void hello() {
     this.rustTestBleScanner = newTestBleScanner();
     System.out.println("TestBleScanner ptr: " + String.valueOf(this.rustTestBleScanner));
     start(this.rustTestBleScanner);
  }

  private long rustTestBleScanner;
}