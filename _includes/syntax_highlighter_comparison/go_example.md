```go
type Node struct {
   value string
   height uint32
   leftChild *Node
   rightChild *Node
}

func NewNode(value string, height uint32) Node {
  return Node{value: value, height: height}
}

func (self *Node) GetValue() string {
  return self.value
}

func (self *Node) GetLeftChild() *Node {
  return self.leftChild
}
```
{: highlighter="compare"}
