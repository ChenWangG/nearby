package com.google.nearby.presence.engine;

public class PresenceScanResultBuilder {
  public static native long create(int medium);
  public static native void addAction(long builderInRust, int action);
  // Returns the *PresenceDiscoveryRequest, the object is opaque to Java.
  public static native long build(long builderInRust);
  public static native void debug(long builderInRust);
  public static native void debugResult(long resultInRust);
}