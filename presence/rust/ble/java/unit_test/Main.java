import com.google.nearby.ble_test.TestBleScanner;

public class Main {
  public static void main(String[] args) {
    TestBleScanner test_scanner = new TestBleScanner();
    System.out.println("======== Test BLE Java.==========");
    test_scanner.hello();
    assert true;
  }
}