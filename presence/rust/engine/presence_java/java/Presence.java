// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.presence;

// Presence in Java Wrapping the Rust implementation.
public class Presence {

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long newPresence();
  private static native void setRequest(long presence, long request);
  private native void start(long presence);

  /* ========== Callbacks called from Rust. ========== */
  synchronized public void onDiscovery(long result) {
      System.out.println("onDiscovery.");
  }

  public void hello() {
      System.out.println("Hello from Presence Java Lib.");
      presence_rust_ptr = newPresence();
      System.out.println("Presence Rust ptr: " + presence_rust_ptr);
      setRequest(1, 1);
      start(presence_rust_ptr);
  }

  // Memory address of Rust Presence.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private long presence_rust_ptr;
}
