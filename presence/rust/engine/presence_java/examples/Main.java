import com.google.nearby.presence.Presence;

public class Main {
  public static void main(String[] args) {
    System.out.println("======== Example to demo Presence Rust Java API.==========");
    System.out.println("==========================================================");
    Presence presence = new Presence();
    presence.hello();
    System.out.println("==========================================================");
    System.out.println("========== End of demo Presence Rust Java API.============");
  }
}