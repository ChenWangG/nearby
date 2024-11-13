import com.google.nearby.test.TestBleScanner;

public class TestBle {
  public static void main(String[] args) {
    TestBleScanner test_scanner = new TestBleScanner();
    System.out.println("======== Test BLE Java.==========");
    test_scanner.hello();
    assert true;
  }
}