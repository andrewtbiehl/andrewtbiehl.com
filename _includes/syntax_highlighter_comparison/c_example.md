```c
typedef struct {
  const char *value;
  uint32_t depth;
  Node *left_child;
  Node *right_child;
} Node;

Node node_new(const char *value, uint32_t depth) {
  return (Node) {.value = value, .depth = depth};
}

const char *node_value(Node *self) {
  return self->value;
}

Node *node_left_child(Node *self) {
  return self->left_child;
}
```
{: highlighter="compare"}
