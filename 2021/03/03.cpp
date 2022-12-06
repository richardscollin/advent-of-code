#include <algorithm>
#include <iostream>
#include <vector>
#include <string>

class bitbucket {
    static const int bitwidth = 12;
    std::vector<int> zeros;
    std::vector<int> ones;
    std::vector<int> values;

public:
    bitbucket() : zeros(bitwidth, 0), ones(bitwidth, 0) {}

    void insert(std::string line)
    {
        int value = std::stoi(line, 0, 2);
        values.push_back(value);

        for (int i = 0; i < bitwidth; i++)
        {
            if (line[i] == '0') {
                zeros[i]++;
            } else if (line[i] == '1') {
                ones[i]++;
            } else {
                std::abort();
            }
        }
    }

    int match_criteria(int criteria) {
      std::vector<int> result(values.size());
      std::cout <<"values size " << values.size() << std::endl;

      int i = 0;
      while (result.size() > 1 && i < bitwidth) {
        auto it = std::copy_if(values.begin(),values.end(), result.begin(),
                               [&](int e) { return (~(e ^ criteria)) >> (bitwidth - 1 - i); });

        result.resize(std::distance(result.begin(), it));
        std::cout <<"new size " << result.size() << std::endl;
        i++;
      }

      if (result.size() == 1) {
          return criteria >> (bitwidth - i);
      }

      return criteria;
    }

    int getGamma() {
        int result = 0;
        for (int i = 0; i < bitwidth; i++, result <<= 1) {
            if (ones[i] >= zeros[i]) {
                result |= 1;
            }
        }
        return result >> 1;
    }
    int getEpsilon() { return getGamma() ^ ((1 << bitwidth) - 1); }
};

int main()
{
    bitbucket bb;

    std::string line;
    while (std::getline(std::cin, line))
    {
        bb.insert(line);
    }

    int o2Criteria = bb.getGamma();
    int co2Criteria = bb.getEpsilon();
    int o2Rating = bb.match_criteria(o2Criteria);
    int co2Rating = bb.match_criteria(co2Criteria);

    std::cout << o2Rating * co2Rating << std::endl;
    return 0;
}
