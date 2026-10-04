#include <iostream>
#include <vector>
#include <span>

int main() {
    std::vector<int> numbers = {10, 20, 30, 40, 50};

    std::span<int> part(numbers.data() + 1, 3);

    for (int x : part) {
        std::cout << x << " ";
    }

    return 0;
}
