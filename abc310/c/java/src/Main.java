import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class Main {

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    int N = sc.nextInt();

    Set<String> acd = new HashSet<String>();

    for (int i = 0; i < N; i++) {
      String Si = sc.next();
      String Sir = new StringBuilder(Si).reverse().toString();
      if (!acd.contains(Si) && !acd.contains(Sir)) {
        acd.add(Si);
      }
    }
    sc.close();
    
    System.out.println(acd.size());
  }
}
