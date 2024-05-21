package com.google.nearby.presence.engine;
public class PresenceDiscoveryResult {
  public static class Builder {
    private Builder(int medium) {
      System.out.println("PresenceDiscoveryResult Builder.");
      this.medium = medium;
    }
    public PresenceDiscoveryResult build() {
      System.out.println("PresenceDiscoveryResult build.");
      return new PresenceDiscoveryResult();
    }
    int medium;
  }

  public static Builder toBuilder(int medium) {
    return new Builder(medium);
  }

}

