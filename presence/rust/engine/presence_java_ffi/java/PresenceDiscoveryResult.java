package com.google.nearby.presence.engine;

import java.util.Arrays;
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

    @Override
    public String toString() {
      StringBuilder stringBuilder = new StringBuilder();
      stringBuilder.append("Builder[").append("medium=").append(this.medium);
      stringBuilder.append(", actions=").append(this.actions.toString());
      stringBuilder.append("]");
      return stringBuilder.toString();
    }

    public void debug() {
      System.out.println("Debug PresenceDiscoveryResult Builder = " + this);
    }

    public PresenceDiscoveryResult build() {
      System.out.println("PresenceDiscoveryResult build.");
      return new PresenceDiscoveryResult(this.medium, this.actions);
    }

    int medium;
    Vector<Integer> actions;
  }

  public static Builder toBuilder(int medium) {
    return new Builder(medium);
  }

  private PresenceDiscoveryResult(int medium, Vector<Integer> actions) {
    this.medium = medium;
    this.actions = (Integer[]) actions.toArray(new Integer[0]);
  }

  @Override
  public String toString() {
    StringBuilder stringBuilder = new StringBuilder();
    stringBuilder.append("PresenceDiscoveryResult[").append("medium=").append(this.medium);
    stringBuilder.append(", actions=").append(Arrays.toString(this.actions));
    stringBuilder.append("]");
    return stringBuilder.toString();
  }

  public void debug() {
    System.out.println("Debug " + this);
    //System.out.println(
    //    "Debug PresenceDiscoveryResult. medium: " + this.medium + ", actions:" + Arrays.toString(
    //        this.actions));
  }

  final int medium;
  final Integer[] actions;
}



