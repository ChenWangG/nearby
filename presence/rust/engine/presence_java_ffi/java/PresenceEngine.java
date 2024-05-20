package com.google.nearby.presence.engine;

// To generate JNI header, run
//    javac -h . PresenceEngine.java

// Presence Engine in Java Wrapping the Rust implementation.
public class PresenceEngine {

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long presenceEngineNew();

  // TODO: move cllabacks to New.
  private static native long presenceEngineRun(long engine,  PresenceEngine object);

  private static native void presenceEngineDebug(long engine);

  private static native void presenceEngineFree(long engine);

  /* ========== Callbacks called from Rust. ========== */
  public static void getDiscoveryResultBuilder(int medium) {
    System.out.println("getDiscoveryResultBuilder with medium: " + medium);
    PresenceDiscoveryResult.build(medium);
  }
  public void onDiscovery(int res) {
    System.out.println("onDiscovery: res = " + res);
  }


  /* ========== Standard Java APIs wrapping the native methods. ========== */
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