```typescript
class ListNode {
  private value: string;
  private next: ListNode | null;

  constructor(value: string, next?: ListNode) {
    this.value = value;
    this.next = next ?? null;
  }

  getValue() : string {
    return this.value;
  }

  getNext() : ListNode | null {
    return this.next;
  }
}

function commaSeparate(head: ListNode | null) : string {
  let values = [];
  while (head != null) {
    values.push(head.getValue());
    head = head.getNext();
  }
  return values.join(", ");
}
```
{: highlighter="compare"}
