package com.google.nearby.presence.engine;

import java.util.Arrays;
import java.util.Vector;

public class PresenceBleScanRequest {
  public static class Builder {

    private Builder(int priority) {
      System.out.println("PresenceBleScanRequest Builder with priority: " + priority);
      this.priority = priority;
      this.actions = new Vector<Integer>();
    }

    public void addAction(int action) {
      System.out.println("PresenceDiscoveryResult builder add action: " + action);
      this.actions.add(action);
    }

    @Override
    public String toString() {
      StringBuilder stringBuilder = new StringBuilder();
      stringBuilder.append("Presence BLE Scan Request Builder[").append("priority=").append(this.priority);
      stringBuilder.append(", actions=").append(this.actions.toString());
      stringBuilder.append("]");
      return stringBuilder.toString();
    }

    public void debug() {
      System.out.println("Debug PresenceDiscoveryResult Builder = " + this);
    }

    public PresenceBleScanRequest build() {
      System.out.println("PresenceBleScanRequest build.");
      return new PresenceBleScanRequest(this.priority, this.actions);
    }

    int priority;
    Vector<Integer> actions;
  }

  public static Builder toBuilder(int priority) {
    return new Builder(priority);
  }

  private PresenceBleScanRequest(int priority, Vector<Integer> actions) {
    this.priority = priority;
    this.actions = (Integer[]) actions.toArray(new Integer[0]);
  }

  public int getPriority() {
    return priority;
  }

  public Integer[] getActions() {
    return actions;
  }

  @Override
  public String toString() {
    StringBuilder stringBuilder = new StringBuilder();
    stringBuilder.append("PresenceBleScanRequest[").append("priority").append(this.priority);
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

  final int priority;
  final Integer[] actions;
}