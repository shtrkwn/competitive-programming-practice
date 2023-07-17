import java.util.Scanner;

public class Main {

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    int N = sc.nextInt();
    int P = sc.nextInt();
    int Q = sc.nextInt();

    int payment = P;
    for (int i = 0; i < N; i++) {
      int Di = sc.nextInt();
      if (payment > Di + Q) {
        payment = Di + Q;
      }

    }
    sc.close();

    System.out.println(payment);
  }
}