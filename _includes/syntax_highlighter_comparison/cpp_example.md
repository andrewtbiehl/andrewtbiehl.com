```cpp
class Node {
  string value;
  uint32_t height;
  Node *left_child;
  Node *right_child;

 public:
  Node(const string &value, uint32_t height)
    : value(value),
      height(height) {}

  string &get_value() const {
    return value;
  }

  Node *get_left_child() {
    return left_child;
  }
}
```
{: highlighter="compare"}
