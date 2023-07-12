import java.util.Scanner;

public class Main {

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    int N = sc.nextInt();

    char[][] A = new char[N][N];
    char[][] B = new char[N][N];
    for (int i = 0; i < N; i++) {
      A[i] = sc.next().toCharArray();
    }
    for (int i = 0; i < N; i++) {
      for (int j = 0; j < N; j++) {
        B[i][j] = A[i][j];
      }
    }

    sc.close();

    // 最上列の処理
    B[0][0] = A[1][0];
    for (int j = 1; j < N; j++) {
      B[0][j] = A[0][j - 1];
    }
    // 中間列の処理
    for (int i = 1; i < N - 1; i++) {
      B[i][0] = A[i + 1][0];
      B[i][N - 1] = A[i - 1][N - 1];
      for (int j = 1; j < N - 1; j++) {
        B[i][j] = A[i][j];
      }
    }
    // 最下列の処理
    B[N - 1][N - 1] = A[N - 2][N - 1];
    for (int j = 0; j < N - 1; j++) {
      B[N - 1][j] = A[N - 1][j + 1];
    }
    for (int i = 0; i < N; i++) {
      System.out.println(new String(B[i]));
    }
  }
}