#include<cassert>
#include<iostream>
#include<queue>
#include<algorithm>

#include "../common/common.hpp"

template<typename T>
long _merge(std::vector<T>& vec, size_t const left, size_t const mid, size_t const right) {
    long swap_count = 0;
    size_t size = right - left + 1;
    size_t curl = left, curr = mid+1;
    T* tmp = new T[size];
    for(int i = 0; i < size; i++) {
        if((curl <= mid) && (curr <= right)) {
            if(vec[curl] <= vec[curr]) {
                tmp[i] = vec[curl++];
            } else {
                tmp[i] = vec[curr++];
                swap_count += mid - curl + 1;
            }
        } else if(curl <= mid) {
            tmp[i] = vec[curl++];
        } else if(curr <= right) {
            tmp[i] = vec[curr++];
        } else {
            std::cerr << "What i'm doing here?" << std::endl;
        }
    }
    std::copy(std::make_move_iterator(tmp),
                                        std::make_move_iterator(&tmp[size]),
                                        vec.begin() + left);
    delete[] tmp;
    return swap_count;
}

template<typename T>
long merge_sort(std::vector<T>& vec) {
    long swap_count = 0;
    size_t curr_size, left_start, n;
    n = vec.size();

    // Merge subarrays in bottom up manner.  First merge subarrays of
    // size 1 to create sorted subarrays of size 2, then merge subarrays
    // of size 2 to create sorted subarrays of size 4, and so on.
    for (curr_size=1; curr_size<=n-1; curr_size = 2*curr_size) {
        // Pick starting point of different subarrays of current size
        for (left_start=0; left_start<n-1; left_start += 2*curr_size) {
            // Find ending point of left subarray. mid+1 is starting 
            // point of right
            int mid = left_start + curr_size - 1;
            int right_end = std::min(left_start + 2*curr_size - 1, n-1);

            // Merge Subarrays arr[left_start...mid] & arr[mid+1...right_end]
            swap_count += _merge(vec, left_start, mid, right_end);
        }
    }
    return swap_count;
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

int main() {
    std::ios::sync_with_stdio(false);

    // 1. Read the array
    auto vec = read_vec_w_num();

    long nswap = 0;
    measure_and_print([&vec, &nswap] ()
        {
            // 2. Mergesort the array, calculate number of swaps
            nswap = merge_sort(vec);
        });

    // 3. Write result
    std::cout << nswap << std::endl;

    return 0;
}
