package com.google.nearby.presence.engine;

// To generate JNI header, run
//    javac -h . PresenceEngine.java

// Presence Engine in Java Wrapping the Rust implementation.
public class Engine {

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long build();

  // TODO: move cllabacks to New.
  private static native long run(long engine, Engine object);

  private static native void debug(long engine);

  private static native void free(long engine);

  /* ========== Callbacks called from Rust. ========== */
  public void onStart(long rust_engine_ptr) {
  }
  public void onDiscovery(PresenceDiscoveryResult res) {
    System.out.println("onDiscovery: res = " + res);
  }

  /* ========== Standard Java APIs wrapping the native methods. ========== */
  public Engine() {
    rust_engine_ptr = build();
  }

  public void run() {
    run(rust_engine_ptr, this);
  }

  public void free() {
    free(rust_engine_ptr);
  }

  public void debug() {
    debug(rust_engine_ptr);

  }

  // Memory address of Rust Engine.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private final long rust_engine_ptr;
}