#include <cassert>
#include <iostream>
#include <vector>
#include <algorithm>

#include "../common/common.hpp"

// return position of the element if found (indexing from 1),
// return -1 otherwise
template<typename T>
int binary_search(std::vector<T> const& vec, T const& value) {
    int l = 0;
    int r = static_cast<int>(vec.size() - 1);
    int i;
    while(l <= r) {
        i = int((l + r) / 2);
        if(vec[i] == value) {
            return i + 1;
        } else if(vec[i] > value) {
            r = i - 1;
        } else if(vec[i] < value) {
            l = i + 1;
        }
    }
    return -1;
}

// read number of elements, read elements into vector
std::vector<int> read_vec_w_num() {
    size_t n;
    std::cin >> n;
    std::vector<int> vec(n, 0);
    for (auto& val: vec)
        std::cin >> val;
    return vec;
}

int main(int argc, char** argv) {
    std::ios::sync_with_stdio(false);

    // 1. Read the array, and values for search
    auto const a_vec = read_vec_w_num();
    auto const b_vec = read_vec_w_num();

    // 2. Search values, write results
    std::vector<int> res(b_vec.size());
    measure_and_print([&a_vec, &b_vec, &res] ()
        {
            for (size_t i = 0; i < b_vec.size(); i++)
                res[i] = binary_search(a_vec, b_vec[i]);
        });

    // 3. Write results
    for (auto const& r: res)
        std::cout << r << " ";

    return 0;
}

