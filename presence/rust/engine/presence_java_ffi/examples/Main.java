import com.google.nearby.presence.engine.PresenceEngine;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello World");
    PresenceEngine engine = new PresenceEngine();
    engine.run();
    engine.debug();
    engine.free();
  }
}