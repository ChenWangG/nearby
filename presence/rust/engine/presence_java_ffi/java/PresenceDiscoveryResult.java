package com.google.nearby.presence.engine;
public class PresenceDiscoveryResult {
  public static class Builder {
    public PresenceDiscoveryResult build() {
      System.out.println("PresenceDiscoveryResult build.");
      return new PresenceDiscoveryResult();
    }
  }

  public static Builder toBuilder() {
    return new Builder();
  }
}

