#include <iostream>
#include <string>
#include <stdexcept>

int main()
{
    int distance = 0;
    int depth = 0;
    int aim = 0;

    std::string s;
    while (std::getline(std::cin, s)) {
        size_t split_idx = s.find(" ");
        int amount = std::stoi(s.substr(split_idx));

        if (s[0] == 'f') { // forward
            distance += amount;
            depth += aim * amount;
        } else if (s[0] == 'u') { // up
            aim -= amount;
        } else if (s[0] == 'd') { // down
            aim += amount;
        } else {
            throw std::invalid_argument("invalid command input");
        }
    }

    std::cout << distance * depth << std::endl;
    return 0;
}
