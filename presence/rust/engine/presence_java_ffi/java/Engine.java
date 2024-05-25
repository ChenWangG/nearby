package com.google.nearby.presence.engine;

// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class

import java.util.concurrent.ExecutorService;
import java.util.concurrent.TimeUnit;

// Presence Engine in Java Wrapping the Rust implementation.
public class Engine {
  public interface Callbacks {
    public void onDiscovery(PresenceDiscoveryResult result);
  }

  public interface SystemApis {
    public void startBleScan(PresenceBleScanRequest request);
  }

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private native long start();
  private static native long setDiscoveryRequest(long engine, long request);
  private static native long onScanResult(long engine, long result);
  private static native void debug(long rust_engine_ptr);
  private static native void free(long rust_engine_ptr);

  /* ========== Callbacks called from Rust. ========== */
  synchronized public void onStart(long rust_engine_ptr) {
    System.out.println("onStart.");
    this.rust_engine_ptr = rust_engine_ptr;
    isStarted = true;
    notify();
  }
  synchronized public void onDiscovery(PresenceDiscoveryResult result) {
    this.callbacks.onDiscovery(result);
  }

  synchronized public void startBleScan(PresenceBleScanRequest request) {
    System.out.println("Java Engine startBleScan." + request);
    this.systemApis.startBleScan(request);
  }

  /* ========== Standard Java APIs wrapping the native methods. ========== */
  public Engine(Callbacks callbacks, SystemApis systemApis) {
    this.callbacks = callbacks;
    this.systemApis = systemApis;
  }

  static void testBuildDiscoveryRequest() {
    long builderInRust = PresenceDiscoveryRequestBuilder.create(131);
    PresenceDiscoveryRequestBuilder.addCondition(builderInRust, 1, 1, 1);
    PresenceDiscoveryRequestBuilder.debug(builderInRust);
    long result = PresenceDiscoveryRequestBuilder.build(builderInRust);
    PresenceDiscoveryRequestBuilder.debugResult(result);
  }

  static long testBuildScanResult() {
    long builderInRust = PresenceScanResultBuilder.create(1);
    PresenceScanResultBuilder.addAction(builderInRust, 101);
    PresenceScanResultBuilder.debug(builderInRust);
    long result = PresenceScanResultBuilder.build(builderInRust);
    PresenceScanResultBuilder.debugResult(result);
    return result;
  }

  synchronized public void start(ExecutorService executor) {
    System.out.println("Start Engine.");
    executor.execute(() -> { start(); });
    try {
      while (!isStarted) {
        wait();
      }
    } catch (InterruptedException e) {
      e.printStackTrace();
    }
  }

  synchronized public void setDiscoveryRequest(long request) {
    System.out.println("setDiscoveryRequest");
    setDiscoveryRequest(this.rust_engine_ptr, request);
  }

  synchronized public void onScanResult(long result) {
    System.out.println("Java onScanResult");
    onScanResult(this.rust_engine_ptr, result);
  }
  synchronized public void free() {
    free(this.rust_engine_ptr);
  }

  synchronized public void debug() {
    System.out.println("debug");
    debug(this.rust_engine_ptr);
  }

  // Memory address of Rust Engine.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private long rust_engine_ptr;
  private final Callbacks callbacks;
  private final SystemApis systemApis;
  boolean isStarted = false;
}