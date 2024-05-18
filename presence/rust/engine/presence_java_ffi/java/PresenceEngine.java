package com.google.nearby.presence.engine;
// To generate JNI header, run
//    javac -h . PresenceEngine.java
public class PresenceEngine {

    static {
        System.loadLibrary("presence_java");
    }

   // No need to pass callbacks.
    public static native long PresenceEngineNew();
    public static native void PresenceEngineDebug(long engine);

  public PresenceEngine() {
    rust_engine_ptr = PresenceEngineNew();
  }

  public void Debug() {
    PresenceEngineDebug(rust_engine_ptr);
  }

  // Memory address of Rust Engine.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private final long rust_engine_ptr;
}