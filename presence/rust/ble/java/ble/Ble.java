// To generate JNI header, run
//    javac -h . *.java
// To get method signatures:
//    javac *.java && javap -s *.class
package com.google.nearby.ble;


// Nearby BLE in Java Wrapping the system API.
public class Ble {
  public static Ble build() {
    return new Ble();
  }
}