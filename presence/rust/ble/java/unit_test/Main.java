// Main class to unit test BLE scanner
// by invoking TestBleScanner::start().
// This test uses system BLE API mocked in BleWrapper.java.

import com.google.nearby.ble_test.TestBleScanner;

public class Main {
  public static void main(String[] args) {
    TestBleScanner test_scanner = new TestBleScanner();
    System.out.println("======== Test BLE Java.==========");
    test_scanner.start();
    assert true;
  }
}