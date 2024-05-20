package com.google.nearby.presence.engine;

// To generate JNI header, run
//    javac -h . PresenceEngine.java
public class PresenceEngine {

  static {
    System.loadLibrary("presence_java");
  }

  // No need to pass callbacks.
  private static native long presenceEngineNew();

  private static native long presenceEngineRun(long engine, PresenceEngine callbacks);

  private static native void presenceEngineDebug(long engine);

  private static native void presenceEngineFree(long engine);

  public void onDiscovery(int res) {
    System.out.println("onDiscovery: res = " + res);
  }

  public PresenceEngine() {
    rust_engine_ptr = presenceEngineNew();
  }

  public void run() {
    presenceEngineRun(rust_engine_ptr, this);
  }

  public void free() {
    presenceEngineFree(rust_engine_ptr);
  }

  public void debug() {
    presenceEngineDebug(rust_engine_ptr);

  }

  // Memory address of Rust Engine.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private final long rust_engine_ptr;
}