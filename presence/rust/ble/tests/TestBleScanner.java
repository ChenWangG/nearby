// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.test;

public class TestBleScanner {
  /* ========== Native methods implemented in Rust. ========== */
  private static native long newTestBleScanner();

  public void hello() {
     System.out.println("Hello Ble.");
  }
}