import com.google.nearby.presence.engine.Engine;
import com.google.nearby.presence.engine.PresenceDiscoveryResult;
import java.util.concurrent.Executors;

class PresenceCallbacks implements Engine.Callbacks {
   @Override
   synchronized public void onDiscovery(PresenceDiscoveryResult result) {
     System.out.println("PresenceCallbacks onDiscovery result: " + result);
   }
}

public class Main {
  public static void main(String[] args) {
    System.out.println("Example to demo Presence Rust Engine Java API.");
    Engine engine = new Engine(new PresenceCallbacks());
    engine.start(Executors.newSingleThreadExecutor());
    engine.setDiscoveryRequest();
    engine.debug();
    // Never do this.
    // engine.free();
  }
}