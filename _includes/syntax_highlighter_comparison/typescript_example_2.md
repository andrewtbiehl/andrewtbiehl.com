{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-3821257580" id="sht-3821257580-1" checked>
  <label for="sht-3821257580-1">Rouge</label>
  <input type="radio" name="sht-3821257580" id="sht-3821257580-2">
  <label for="sht-3821257580-2">Tree-sitter</label>
<pre><code data-language="typescript" data-highlighter="rouge"><span class="kd">class</span> <span class="nx">Node</span> <span class="p">{</span>
  <span class="nl">value</span><span class="p">:</span> <span class="kr">string</span><span class="p">;</span>
  <span class="nl">height</span><span class="p">:</span> <span class="kr">number</span><span class="p">;</span>
  <span class="nl">leftChild</span><span class="p">:</span> <span class="nx">Node</span><span class="p">;</span>
  <span class="nl">rightChild</span><span class="p">:</span> <span class="nx">Node</span><span class="p">;</span>

  <span class="kd">constructor</span><span class="p">(</span><span class="nx">value</span><span class="p">:</span> <span class="kr">string</span><span class="p">,</span> <span class="nx">height</span><span class="p">:</span> <span class="kr">number</span><span class="p">)</span> <span class="p">{</span>
    <span class="k">this</span><span class="p">.</span><span class="nx">value</span> <span class="o">=</span> <span class="nx">value</span><span class="p">;</span>
    <span class="k">this</span><span class="p">.</span><span class="nx">height</span> <span class="o">=</span> <span class="nx">height</span><span class="p">;</span>
  <span class="p">}</span>

  <span class="nx">getValue</span><span class="p">()</span> <span class="p">:</span> <span class="kr">string</span> <span class="p">{</span>
    <span class="k">return</span> <span class="k">this</span><span class="p">.</span><span class="nx">value</span>
  <span class="p">}</span>

  <span class="nx">getLeftChild</span><span class="p">()</span> <span class="p">:</span> <span class="nx">Node</span> <span class="p">{</span>
    <span class="k">return</span> <span class="k">this</span><span class="p">.</span><span class="nx">leftChild</span><span class="p">;</span>
  <span class="p">}</span>
<span class="p">}</span>
</code></pre>

<pre><code data-language="typescript" data-highlighter="tree-sitter"><span class="ts-keyword">class</span> <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>
  <span class="ts-property">value</span>: <span class="ts-type-builtin">string</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-property">height</span>: <span class="ts-type-builtin">number</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-property">leftChild</span>: <span class="ts-type">Node</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-property">rightChild</span>: <span class="ts-type">Node</span><span class="ts-punctuation-delimiter">;</span>

  <span class="ts-function-method">constructor</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">value</span>: <span class="ts-type-builtin">string</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-variable-parameter">height</span>: <span class="ts-type-builtin">number</span><span class="ts-punctuation-bracket">)</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">value</span> <span class="ts-operator">=</span> <span class="ts-variable-parameter">value</span><span class="ts-punctuation-delimiter">;</span>
    <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">height</span> <span class="ts-operator">=</span> <span class="ts-variable-parameter">height</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>

  <span class="ts-function-method">getValue</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span> : <span class="ts-type-builtin">string</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-keyword">return</span> <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">value</span>
  <span class="ts-punctuation-bracket">}</span>

  <span class="ts-function-method">getLeftChild</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span> : <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-keyword">return</span> <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">leftChild</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>
<span class="ts-punctuation-bracket">}</span>
</code></pre>

</div>
