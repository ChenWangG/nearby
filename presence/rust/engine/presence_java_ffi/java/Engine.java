package com.google.nearby.presence.engine;

// To generate JNI header, run
//    javac -h . PresenceEngine.java
// To get method signatures:
//    javac *.java && javap -s *.class

import java.util.concurrent.ExecutorService;
import java.util.concurrent.TimeUnit;

// Presence Engine in Java Wrapping the Rust implementation.
public class Engine {

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long build();

  // TODO: move cllabacks to New.
  private static native long start(long engine, Engine object);

  private static native void debug(long engine);

  private static native void free(long engine);

  /* ========== Callbacks called from Rust. ========== */
  public void onStart(long rust_engine_ptr) {
    System.out.println("onStart.");
    this.rust_engine_ptr = rust_engine_ptr;
  }
  public void onDiscovery(PresenceDiscoveryResult res) {
    System.out.println("onDiscovery: res = " + res);
  }

  /* ========== Standard Java APIs wrapping the native methods. ========== */
  public Engine() {
    rust_engine_ptr = build();
  }

  public void start(ExecutorService executor) {
    executor.execute(() -> { start(this.rust_engine_ptr, this); });
    try {
      TimeUnit.MILLISECONDS.sleep(300);
    } catch (InterruptedException e) {
      e.printStackTrace();
    }
  }

  public void free() {
    free(this.rust_engine_ptr);
  }

  public void debug() {
    System.out.println("debug");
    debug(this.rust_engine_ptr);

  }

  // Memory address of Rust Engine.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private long rust_engine_ptr;
}