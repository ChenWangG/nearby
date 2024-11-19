// Main class to unit test BLE scanner
// by invoking TestBleScanner::start().
// This test uses system BLE API mocked in BleWrapper.java.

import com.google.nearby.ble_test.TestBleScanner;
import com.google.nearby.ble.ScanResult;

public class Main {
  public static void main(String[] args) {
    TestBleScanner test_scanner = new TestBleScanner();
    System.out.println("======== Test ScanResult FFI.==========");
    byte[] serviceData = new byte[]{1, 2, 3};
    ScanResult result = new ScanResult(serviceData);
    result.toRustScanResult(serviceData);
    System.out.println("======== Test E2E from mocked BleWrapper to TestBleScanner.==========");
    test_scanner.start();
    assert true;
  }
}