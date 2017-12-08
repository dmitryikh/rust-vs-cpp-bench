#include <cassert>
#include <iostream>
#include <vector>
#include <unordered_map>

struct HuffmanCodec final {

    struct Node {
        char value = ' ';
        Node* left = nullptr;
        Node* right = nullptr;
    };

    void buildTree(std::unordered_map<char, std::string> const& letter_codes);
    void clear();
    size_t size() const;
    std::string decode(std::string const& code) const;

private:
    size_t _size;
    Node* head = nullptr;
    std::vector<Node> nodes;
};

// restore Huffman tree based on letters' codes
void HuffmanCodec::buildTree(std::unordered_map<char, std::string> const& letter_codes) {
    assert(letter_codes.size() > 0);
    clear();
    _size = letter_codes.size();
    if (_size == 0) {
        return;
    } else if (_size == 1) {
        // only one letter in the table
        nodes.resize(1);
        for (auto const& v : letter_codes) {
            // assert(v.second == "0");
            head = &nodes[0];
            head->left = nullptr;
            head->right = nullptr;
            head->value = v.first;
        }
    } else {
        nodes.resize(_size * 2 - 1);
        size_t next = 1;
        head = &nodes[0];
        // for every letters' code traverse the tree inserting missing nodes
        for (auto const& v : letter_codes) {
            Node* n = head;
            for (char c : v.second) {
                if (c == '0') {
                    if(n->left == nullptr) {
                        n->left = &nodes[next++];
                    }
                    n = n->left;
                } else if (c == '1') {
                    if(n->right == nullptr) {
                        n->right = &nodes[next++];
                    }
                    n = n->right;
                }
            }
            assert(n->right == nullptr && n->left == nullptr);
            n->value = v.first;
        }
        assert(next == nodes.size());
    }
}

void HuffmanCodec::clear() {
    _size = 0;
    nodes.clear();
    head = nullptr;
}

size_t HuffmanCodec::size() const {
    return _size;
}

// decode `code` into original message
std::string HuffmanCodec::decode(std::string const& code) const {
    if (size() == 0)
        throw std::runtime_error("Huffman tree is empty!");
    assert(head != nullptr);

    std::string ret;
    size_t i = 0;
    if(size() == 1) {
        // special case, when size() == 1
        for(; i < code.size(); i++) {
            assert(code[i] == '0');
            ret += head->value;
        }
    } else {
        Node* n = head;
        for (char const c : code) {
            assert((c == '0' || c == '1'));
            assert((n->left != nullptr) || (n->right != nullptr));
            if(c == '0')
                n = n->left;
            else if(c == '1')
                n = n->right;
            else
                throw std::runtime_error("Uknown code");

            if (n->left == nullptr) {
                // we are in leaf
                assert(n->right == nullptr);
                ret += n->value;
                n = head;
            }
        }
    }
    return ret;
}

int main() {
    // 1. Read input data: number of letters, code length, letter->code table, code
    std::ios::sync_with_stdio(false);
    int nlet;
    int code_len;
    std::unordered_map<char, std::string> letters;
    std::string _code;
    std::string c, co;
    std::cin >> nlet >> code_len;
    for(int i = 0; i < nlet; i++) {
        std::cin >> c >> co;
        letters.insert(std::make_pair(c[0], co));
    }
    std::cin >> _code;

    // 2. Build Huffman tree
    HuffmanCodec codec;
    codec.buildTree(letters);

    // 3. Decode message
    std::string const res = codec.decode(_code);

    // 4. Output
    std::cout << codec.decode(_code) << std::endl;
    return 0;
}
