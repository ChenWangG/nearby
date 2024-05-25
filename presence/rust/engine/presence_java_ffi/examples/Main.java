import com.google.nearby.presence.engine.Engine;
import com.google.nearby.presence.engine.PresenceDiscoveryResult;
import com.google.nearby.presence.engine.PresenceBleScanRequest;
import com.google.nearby.presence.engine.PresenceDiscoveryRequestBuilder;
import com.google.nearby.presence.engine.PresenceScanResultBuilder;
import java.util.concurrent.Executors;

class Constants {
  public static final int PRIORITY = 199;
  public static final int ACTION_ZERO = 100;
  public static final int ACTION_ONE = 101;
  public static final int MEDIUM = 1;
}
class PresenceCallbacks implements Engine.Callbacks {
   @Override
   public void onDiscovery(PresenceDiscoveryResult result) {
     System.out.println("PresenceCallbacks onDiscovery result: " + result);
     assert result.getMedium() == Constants.MEDIUM;
     assert result.getActions().length == 2;
     assert result.getActions()[0] == Constants.ACTION_ZERO;
     assert result.getActions()[1] == Constants.ACTION_ONE;
   }
}

class PresenceSystemApis implements Engine.SystemApis {
  @Override
  synchronized public void startBleScan(PresenceBleScanRequest request) {
    System.out.println("PresenceSystemApis startBleScan: " + request);
    assert request.getPriority() == Constants.PRIORITY;

    long builderInRust = PresenceScanResultBuilder.create(Constants.MEDIUM);
    for (int action : request.getActions()) {
      PresenceScanResultBuilder.addAction(builderInRust, action);
    }
    PresenceScanResultBuilder.debug(builderInRust);
    result = Long.valueOf(PresenceScanResultBuilder.build(builderInRust));
    notify();
  }

  synchronized public long waitForResult() {
    try {
      while(this.result == null) {
        this.wait();
      }
    } catch (InterruptedException e) {
      e.printStackTrace();
    }
    return this.result;
  }

  private Long result = null;

}

public class Main {
  public static void main(String[] args) {
    System.out.println("Example to demo Presence Rust Engine Java API.");
    PresenceSystemApis apis = new PresenceSystemApis();
    Engine engine = new Engine(new PresenceCallbacks(), apis);
    engine.start(Executors.newSingleThreadExecutor());
    long builderInRust = PresenceDiscoveryRequestBuilder.create(Constants.PRIORITY);
    PresenceDiscoveryRequestBuilder.addCondition(builderInRust, Constants.ACTION_ZERO, 1, 1);
    PresenceDiscoveryRequestBuilder.addCondition(builderInRust, Constants.ACTION_ONE, 1, 1);
    long request = PresenceDiscoveryRequestBuilder.build(builderInRust);
    engine.setDiscoveryRequest(request);
    long result = apis.waitForResult();
    engine.onScanResult(result);
  }
}