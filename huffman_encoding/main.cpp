#include <cassert>
#include <iostream>
#include <vector>
#include <queue>
#include <unordered_map>
#include <exception>

#include "../common/common.hpp"

struct HuffmanCodec final {

    // typedef and struct declaration to organize a priority queue to build Huffman tree
    // size_t - index of the node in `nodes` vector
    // int - frequency of the node
    typedef std::pair<size_t, int> Code;
    struct Compare {
        bool operator()(const Code& op1, const Code& op2) {
            return op1.second > op2.second;
        }
    };

    struct Node {
        int freq;
        char value = ' ';
        Node* left = nullptr;
        Node* right = nullptr;
    };

    void buildTree(std::unordered_map<char, int> const& freq);
    void buildTable();
    void clear();
    size_t size() const;
    void printTable(std::ostream& stream) const;
    std::string encode(std::string const& msg) const;
    std::string const& encode(char const c) const;

private:
    size_t _size;
    Node* head = nullptr;
    std::vector<Node> nodes;
    std::unordered_map<char, std::string> _table;

    void _buildTable(Node* n, std::string code);
};

// build Huffman tree based on letters frequency
void HuffmanCodec::buildTree(std::unordered_map<char, int> const& freq) {
    clear();
    _size = freq.size();
    // priliminary initialize all nodes
    nodes.resize(2 * _size - 1);
    std::priority_queue<int, std::vector<Code>, Compare> queue;
    size_t i = 0;
    for(auto& v : freq) {
        nodes[i].freq = v.second;
        nodes[i].value = v.first;
        queue.push(std::make_pair(i, v.second));
        i++;
    }
    if(_size == 1) {
        // only one letter in freq. table
        head = &nodes[0];
    } else {
        for(size_t k = _size; k < 2 * _size - 1; k++) {
            // take two most frequent nodes and create new node on top of them,
            // push the new node to priority queue
            auto const v1 = queue.top();
            queue.pop();
            auto const v2 = queue.top();
            queue.pop();
            nodes[k].left = &nodes[v1.first];
            nodes[k].right = &nodes[v2.first];
            nodes[k].freq = v1.second + v2.second;
            head = &nodes[k];
            queue.push(std::make_pair(k, nodes[k].freq));
        }
    }
}

// build Huffman table for encoding
void HuffmanCodec::buildTable() {
    assert(size() > 0);
    assert(head != nullptr);
    _table.clear();
    _buildTable(head, "");
}

void HuffmanCodec::clear() {
    _table.clear();
    _size = 0;
    nodes.clear();
    head = nullptr;
}

size_t HuffmanCodec::size() const {
    return _size;
}

// internal function for recursive tree traversal
void HuffmanCodec::_buildTable(Node* n, std::string code) {
    if(n->left != nullptr) {
        // this is intermidiate node. Go deep
        assert(n->right != nullptr);
        _buildTable(n->left, code + "0");
        _buildTable(n->right, code + "1");
    } else {
        // this is leaf
        if(n == head) code = "0";
        assert(n->value != 0);
        _table[n->value] = code;
    }
}

void HuffmanCodec::printTable(std::ostream& stream) const {
    assert(_size > 0);
    for(size_t i = 0; i < size(); i++) {
        char c = nodes[i].value;
        stream << c << ": " << encode(c) << std::endl;
    }
}

// encode text message
std::string HuffmanCodec::encode(std::string const& msg) const {
    std::string ret = "";
    for(char const c : msg) {
        ret += encode(c);
    }
    return ret;
}

// encode letter
std::string const& HuffmanCodec::encode(char const c) const {
    assert(size() > 0);
    auto v = _table.find(c);
    if(v != _table.end()) {
        return v->second;
    } else {
        throw std::runtime_error("Can't find letter");
    }
}

int main() {
    // 1.Read message
    std::ios::sync_with_stdio(false);
    std::string str;
    std::cin >> str;

    HuffmanCodec codec;
    std::string code;
    measure_and_print([&codec, &str, &code] ()
        {
            // 2. Calculate frequancy map for letters
            std::unordered_map<char, int> freq;
            for(char c : str) {
                freq[c]++;
            }

            // 3. Build Huffman tree & table
            codec.buildTree(freq);
            codec.buildTable();

            // 4. Encode message
            code = codec.encode(str);
        });

    // 5. Output
    std::cout << codec.size() << " " << code.size() << std::endl;
    codec.printTable(std::cout);
    std::cout << code << std::endl;
    return 0;
}
