public class Ble {
    static {
       System.loadLibrary("ble_rust_java_ffi");
    }

    synchronized public void startScan(ScanRequest request) {
        System.out.println("Java system API start ble scan." + request);
    }
}