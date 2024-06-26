import com.google.nearby.api.TestApi;

public class Main {
  public static void main(String[] args) {
    TestApi test_api = new TestApi();
    test_api.onScanResult();
    System.out.println("Example to demo Nearby Android System API in Rust.");
  }
}