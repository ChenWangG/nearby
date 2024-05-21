package com.google.nearby.presence.engine;

import java.util.Vector;
public class PresenceDiscoveryResult {
  public static class Builder {
    private Builder(int medium) {
      System.out.println("PresenceDiscoveryResult Builder with medium: " + medium);
      this.medium = medium;
      this.actions = new Vector<Integer>();
    }
    public void addAction(int action) {
      System.out.println("PresenceDiscoveryResult builder add action: " + action);
      this.actions.add(action);
    }

    public void debug(int noUse) {
      System.out.println("Debug PresenceDiscoveryResult medium: " + this.medium);
      for (int action : this.actions) {
        System.out.println("action: " + action);
      }
    }

    public PresenceDiscoveryResult build() {
      System.out.println("PresenceDiscoveryResult build.");
      return new PresenceDiscoveryResult();
    }
    int medium;
    Vector<Integer> actions;
  }

  public static Builder toBuilder(int medium) {
    return new Builder(medium);
  }
}



