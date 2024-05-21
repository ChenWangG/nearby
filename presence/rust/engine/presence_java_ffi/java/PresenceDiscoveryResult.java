package com.google.nearby.presence.engine;

import java.util.Vector;
public class PresenceDiscoveryResult {
  public static class Builder {
    private Builder(int medium) {
      System.out.println("PresenceDiscoveryResult Builder.");
      this.medium = medium;
      this.device = new Device();
    }
    public void addAction(int action) {
      System.out.println("PresenceDiscoveryResult builder add action.");
    }
    public PresenceDiscoveryResult build() {
      System.out.println("PresenceDiscoveryResult build.");
      return new PresenceDiscoveryResult();
    }
    int medium;
    Device device;
  }

  public static Builder toBuilder(int medium) {
    return new Builder(medium);
  }

  public static class Device {
    public Device() {
      Vector<Integer> actions = new Vector<Integer>();
    }
  public void addAction(int action) {
    this.actions.add(action);
  }
  Vector<Integer> actions;
  }

}



