import java.util.Scanner;

public class Main {

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    int t = sc.nextInt();
    sc.close();
    System.out.println(f(f(f(t) + t) + f(f(t))));

  }

  public static int f(int t) {
    return t * t + 2 * t + 3;
  }
}