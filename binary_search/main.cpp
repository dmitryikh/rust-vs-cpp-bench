#include <cassert>
#include <iostream>
#include <vector>
#include <algorithm>

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
    auto a_vec = read_vec_w_num();
    auto const b_vec = read_vec_w_num();

    // 2. Sort the array
    std::sort(std::begin(a_vec), std::end(a_vec));

    // 3. Search values, write results
    for (auto const& val: b_vec)
        std::cout << binary_search(a_vec, val) << " ";

    return 0;
}

