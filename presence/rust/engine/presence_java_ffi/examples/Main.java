import com.google.nearby.presence.engine.Engine;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello World");
    Engine engine = new Engine();
    engine.run();
    engine.debug();
    engine.free();
  }
}