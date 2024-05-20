import com.google.nearby.presence.engine.PresenceEngine;
import com.google.nearby.presence.engine.PresenceDiscoveryResult;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello World");
    PresenceDiscoveryResult.build(10);
    PresenceEngine engine = new PresenceEngine();
    engine.run();
    engine.debug();
    engine.free();
  }
}