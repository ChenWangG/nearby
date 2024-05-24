package com.google.nearby.presence.engine;

public class PresenceDiscoveryRequestBuilder {
  public static native long create(int priority);
  public static native void addCondition(long builderInRust, int action, int identityType, int measurementAccuracy);
  // Returns the *PresenceDiscoveryRequest, the object is opaque to Java.
  public static native long build(long builderInRust);
  public static native void debug(long builderInRust);
  public static native void debugResult(long resultInRust);
}

