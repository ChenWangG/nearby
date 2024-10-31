// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class

// Presence in Java Wrapping the Rust implementation.
public class Presence {

  static {
    System.loadLibrary("presence_java");
  }

   /* ========== Native methods implemented in Rust. ========== */
    private static native long newPresence();
    private static native void setRequest(long presence, long request);

  // Memory address of Rust Presence.
  // Opaque pointer to be passed back and forth between Rust and Java.
  private long rust_engine_ptr;
}
