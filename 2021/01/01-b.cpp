#include <iostream>

class ring_buffer {
    static const int window_size = 3;
    int data[window_size] = { 0 };
    int idx = 0;

public:
    bool full = 0;
    int running_sum = 0;

    void append(int value) {
        running_sum += value - data[idx];

        data[idx] = value;
        idx = (idx + 1) % window_size;
        if (idx == 0 && !full) {
            full = true;
        }
    }
};

int main()
{
    int window_increases = 0;
    ring_buffer rb;

    int prev_sum = 0;
    int n;
    while (std::cin >> n) {
        rb.append(n);

        if (rb.full) {
            if (rb.running_sum > prev_sum) {
                window_increases++;
            }
            prev_sum = rb.running_sum;
        }

    }

    std::cout << window_increases - 1 << std::endl;
    return 0;
}
