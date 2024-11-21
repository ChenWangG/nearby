package com.google.nearby.presence;

import java.util.List;
import java.util.ArrayList;

public class DiscoveryResult {
  public static class Builder {
    public Builder() {
      dataElements = new ArrayList<DataElement>();
    }

    private ArrayList<DataElement> dataElements;
  }

  public class DataElement {
    public DataElement(int type, byte[] content) {
      this.type = type;
      this.content = content;
    }

    private int type;
    private byte[] content;
  }

  public DiscoveryResult(ArrayList<DataElement> dataElements) {
    this.dataElements = dataElements;
  }

  public List<DataElement> dataElements() {
    return dataElements;
  }

  // Static method called from Rust to return a Ble instance.
  public static Builder newBuilder() {
   System.out.println("Java DiscoveryResult new Builder.");
    return new Builder();
  }

  private final ArrayList<DataElement> dataElements;
}