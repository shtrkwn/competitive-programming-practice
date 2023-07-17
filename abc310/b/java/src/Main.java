import java.util.Arrays;
import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class Main {

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    int N = sc.nextInt();
    int M = sc.nextInt();

    Product[] products = new Product[N];

    for (int i = 0; i < N; i++) {
      int Pi = sc.nextInt();
      int Ci = sc.nextInt();
      products[i] = new Product(Pi);
      for (int j = 0; j < Ci; j++) {
        products[i].functions.add(sc.nextInt());
      }
    }
    sc.close();

    Arrays.sort(products);

    for (int i = 0; i < N - 1; i++) {
      for (int j = i + 1; j < N; j++) {
        if (products[j].functions.containsAll(products[i].functions)) {
          if (products[i].price > products[j].price || products[j].functions.size() > products[i].functions.size()) {
            System.out.println("Yes");
            return;
          }
        }
      }
    }

    System.out.println("No");
  }
}

class Product implements Comparable<Product> {

  int price;
  Set<Integer> functions;

  public Product(int price) {
    this.price = price;
    this.functions = new HashSet<Integer>();
  }

  @Override
  public int compareTo(Product product) {
    return product.price - this.price;
  }

}