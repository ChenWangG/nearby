import com.google.nearby.test.Ble;

public class TestBle {
  public static void main(String[] args) {
    Ble ble = new Ble();
    System.out.println("======== Test BLE Java.==========");
    ble.hello();
    assert true;
  }
}