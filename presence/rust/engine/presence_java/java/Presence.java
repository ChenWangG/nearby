// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.presence;

import java.util.concurrent.ExecutorService;

// Presence in Java Wrapping the Rust implementation.
public class Presence {
  public interface Callbacks {
    public void onDiscovery(long result);
  }

  public int testNdk() {
      return 7;
  }

  static {
    System.loadLibrary("presence_java");
  }

  /* ========== Native methods implemented in Rust. ========== */
  private static native long newPresence();
  private static native void setRequest(long presence, long request);
  private native void start(long presence);
  private native void stop(long presence);

  /* ========== Callbacks called from Rust. ========== */
  synchronized public void onDiscovery(long result) {
      System.out.println("Java onDiscovery.");
      this.callbacks.onDiscovery(result);

  }


  /* ========== Presence Java API. ========== */
  public Presence(Callbacks callbacks) {
    this.callbacks = callbacks;
    presence_rust_ptr = newPresence();
  }

  synchronized public void start(ExecutorService executor) {
     System.out.println("Start Engine.");
     executor.execute(() -> { start(this.presence_rust_ptr); });
   }

  synchronized public void setRequest() {
    setRequest(this.presence_rust_ptr, 1);
  }

  synchronized public void stop() {
     System.out.println("Stop Engine.");
     stop(this.presence_rust_ptr);
  }

  public void hello() {
      System.out.println("Hello from Presence Java Lib.");
      presence_rust_ptr = newPresence();
      System.out.println("Presence Rust ptr: " + presence_rust_ptr);
      setRequest(1, 1);
      start(presence_rust_ptr);
  }

  final Callbacks callbacks;
  // Memory address of Rust Presence.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private long presence_rust_ptr;
}
