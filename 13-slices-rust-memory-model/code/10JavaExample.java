import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] numbers = {10, 20, 30, 40, 50};

        int[] part = Arrays.copyOfRange(numbers, 1, 4);

        System.out.println(Arrays.toString(part));
    }
}
