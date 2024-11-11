package com.goog.nearby.presence_test;

public class TestNdk {

  static {
    System.loadLibrary("test-ndk");
  }

  private static native long newNdk();

  public TestNdk() {
    this.ndkValue = newNdk();
  }

  public long getNdkValue() {
    return this.ndkValue;
  }

  private long ndkValue = 0;
}
