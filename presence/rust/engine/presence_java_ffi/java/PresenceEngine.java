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

  public static void main(String[] args) {
    System.out.println("Hello World");
    long engine = PresenceEngineNew();
    PresenceEngineDebug(engine);
  }
}