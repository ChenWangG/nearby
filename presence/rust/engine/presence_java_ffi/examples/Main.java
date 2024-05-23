import com.google.nearby.presence.engine.Engine;
import com.google.nearby.presence.engine.PresenceDiscoveryResult;
import java.util.concurrent.Executors;

class PresenceCallbacks implements Engine.Callbacks {
   public void onStart() {
     System.out.println("PresenceCallbacks onStart.");
   }
   public void onDiscovery(PresenceDiscoveryResult result) {
     System.out.println("PresenceCallbacks onDiscovery.");
     System.out.println("onDiscovery: result = " + result);
   }

}

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello World");
    Engine engine = new Engine(new PresenceCallbacks());

    engine.start(Executors.newSingleThreadExecutor());
    engine.debug();
    // Never do this.
    // engine.free();
  }
}