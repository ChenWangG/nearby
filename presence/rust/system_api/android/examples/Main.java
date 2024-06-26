import com.google.nearby.api.TestApi;

public class Main {
  public static void main(String[] args) {
    System.out.println("Example to demo Nearby Android System API in Rust.");
    TestApi test_api = new TestApi();
    test_api.startScan();
  }
}