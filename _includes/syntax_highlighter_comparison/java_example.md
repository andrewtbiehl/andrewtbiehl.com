```java
public class DisjointSet {
    private final int[] representatives, componentSizes;

    public DisjointSet(int n) {
        this.representatives = new int[n];
        for (int i = 0; i < n; i++) this.representatives[i] = i;
        this.componentSizes = new int[n];
        for (int i = 0; i < n; i++) this.componentSizes[i] = 1;
    }

    public void union(int i, int j) {
        int r1 = this.find(i), r2 = this.find(j);
        if (r1 == r2) return;
        if (this.componentSizes[r1] < this.componentSizes[r2]) {
            int temp = r1; r1 = r2; r2 = temp;
        }
        this.componentSizes[r1] += this.componentSizes[r2];
        this.representatives[r2] = r1;
    }

    public boolean areConnected(int i, int j) {
        return this.find(i) == this.find(j);
    }

    private int find(int i) {
        int r = i;
        while (this.representatives[r] != r) r = this.representatives[r];
        this.compressPath(i, r);
        return r;
    }

    private void compressPath(int i, int representative) {
        while (i != representative) {
            int temp = this.representatives[i];
            this.representatives[i] = representative;
            i = temp;
        }
    }
}
```
{: highlighter="compare"}
