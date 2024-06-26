package com.google.nearby.api;

// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class

public class TestApi {

  static {
    System.loadLibrary("test_api");
  }

  /* ========== Native methods implemented in Rust. ========== */
  public static native void startScan();

  /* ================= Called from Rust. ==================== */
  public void onScanResult() {
    System.out.println("onStart.");
  }
}