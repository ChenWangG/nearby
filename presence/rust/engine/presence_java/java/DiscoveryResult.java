package com.google.nearby.presence;

import java.util.List;
import java.util.ArrayList;
import java.util.Arrays;

public class DiscoveryResult {
  public static class Builder {
    public Builder() {
      dataElements = new ArrayList<DataElement>();
    }

    public void addDataElement(int type, byte[] content) {
      System.out.println("Add DE type: " + type + "content " + Arrays.toString(content));
      this.dataElements.add(new DataElement(type, content));
    }

    public DiscoveryResult build() {
      System.out.println("Build");
      for (DataElement de : dataElements) {
      System.out.println("DE type: " + de.type + "content " + Arrays.toString(de.content));
      }
      return new DiscoveryResult(dataElements);
    }

    private ArrayList<DataElement> dataElements;
  }

  public static class DataElement {
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