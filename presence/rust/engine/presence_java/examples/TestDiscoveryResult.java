public class TestDiscoveryResult {
  static {
    System.loadLibrary("test_discovery_result");
  }
  /* ========== Native methods implemented in Rust. ========== */
  private static native long testDiscoveryResult();

  public static void main(String[] args) {
    System.out.println("======== TestJavaDiscoveryResult. ==========");
    testDiscoveryResult();
  }
}