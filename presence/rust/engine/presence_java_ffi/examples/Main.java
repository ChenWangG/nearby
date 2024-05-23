import com.google.nearby.presence.engine.Engine;
import java.util.concurrent.Executors;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello World");
    Engine engine = new Engine();

    engine.start(Executors.newSingleThreadExecutor());
    engine.debug();
    engine.free();
  }
}