import com.google.nearby.presence.DiscoveryResult;
import java.util.Arrays;

public class TestDiscoveryResult {
  static {
    System.loadLibrary("test_discovery_result");
  }
  /* ========== Native methods implemented in Rust. ========== */
  private static native void testDiscoveryResult();

  // TODO: return result from testDiscoveryResult() directly.
  public static void onResult(DiscoveryResult result) {
    System.out.println("======== onResult. ==========");
    assert result.dataElements().size() == 1;
    assert result.dataElements().get(0).type == 5;
    assert Arrays.equals(result.dataElements().get(0).content, new byte[]{1, 2, 3, 4});
  }

  public static void main(String[] args) {
    System.out.println("======== TestJavaDiscoveryResult. ==========");
    testDiscoveryResult();
  }
}