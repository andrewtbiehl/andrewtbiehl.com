```rust
struct Node {
    value: String,
    height: u32,
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>,
}

impl Node {
    pub fn new(value: String, height: u32) -> Node {
        Node {value, height, left_child: None, right_child: None}
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn get_left_child(&self) -> Option<&Node> {
        self.left_child.as_ref().map(|child| child.as_ref())
    }
}
```
{: highlighter="compare"}
