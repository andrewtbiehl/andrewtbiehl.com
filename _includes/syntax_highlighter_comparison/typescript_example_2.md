```typescript
class Node {
  value: string;
  height: number;
  leftChild: Node;
  rightChild: Node;

  constructor(value: string, height: number) {
    this.value = value;
    this.height = height;
  }

  getValue() : string {
    return this.value
  }

  getLeftChild() : Node {
    return this.leftChild;
  }
}
```
{: highlighter="compare"}
