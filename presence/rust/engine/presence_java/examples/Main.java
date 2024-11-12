import com.google.nearby.presence.Presence;
import java.util.concurrent.Executors;
import java.util.concurrent.ExecutorService;

class TestCallbacks implements Presence.Callbacks {
  synchronized public void onDiscovery(long result)  {
    System.out.println("TestCallbacks: onDiscovery");
    this.result = result;
    notify();
  }

 synchronized public long waitForResult() {
     try {
       while(this.result == null) {
         this.wait();
       }
       System.out.println("TestCallbacks: waitForResult has the result.");
     } catch (InterruptedException e) {
       e.printStackTrace();
     }
     return this.result;
   }

   private Long result = null;
}

public class Main {
  public static void main(String[] args) {
    System.out.println("======== Example to demo Presence Rust Java API.==========");
    System.out.println("==========================================================");

    TestCallbacks callbacks = new TestCallbacks();
    Presence presence = new Presence(callbacks);
    ExecutorService executor = Executors.newSingleThreadExecutor();
    presence.start(executor);
    presence.setRequest();
    callbacks.waitForResult();
    presence.stop();
    executor.shutdown();
    System.out.println("Service shutdown.");

    // New Presence instance required to restart the service.
    // The previous instance has been consumed by the executor.
    presence = new Presence(callbacks);
    executor = Executors.newSingleThreadExecutor();
    presence.start(executor);
    System.out.println("Service Restarted.");
    presence.stop();
    executor.shutdown();
    System.out.println("Service shutdown again.");

    System.out.println("==========================================================");
    System.out.println("========== End of demo Presence Rust Java API.============");
  }
}