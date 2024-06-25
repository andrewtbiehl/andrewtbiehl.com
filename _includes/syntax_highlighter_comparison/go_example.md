{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-35234018" id="sht-35234018-1" checked>
  <label for="sht-35234018-1">Rouge</label>
  <input type="radio" name="sht-35234018" id="sht-35234018-2">
  <label for="sht-35234018-2">Tree-sitter</label>
<pre><code data-language="go" data-highlighter="rouge"><span class="k">type</span> <span class="n">Node</span> <span class="k">struct</span> <span class="p">{</span>
   <span class="n">value</span> <span class="kt">string</span>
   <span class="n">height</span> <span class="kt">uint32</span>
   <span class="n">leftChild</span> <span class="o">*</span><span class="n">Node</span>
   <span class="n">rightChild</span> <span class="o">*</span><span class="n">Node</span>
<span class="p">}</span>

<span class="k">func</span> <span class="n">NewNode</span><span class="p">(</span><span class="n">value</span> <span class="kt">string</span><span class="p">,</span> <span class="n">height</span> <span class="kt">uint32</span><span class="p">)</span> <span class="n">Node</span> <span class="p">{</span>
  <span class="k">return</span> <span class="n">Node</span><span class="p">{</span><span class="n">value</span><span class="o">:</span> <span class="n">value</span><span class="p">,</span> <span class="n">height</span><span class="o">:</span> <span class="n">height</span><span class="p">}</span>
<span class="p">}</span>

<span class="k">func</span> <span class="p">(</span><span class="n">self</span> <span class="o">*</span><span class="n">Node</span><span class="p">)</span> <span class="n">GetValue</span><span class="p">()</span> <span class="kt">string</span> <span class="p">{</span>
  <span class="k">return</span> <span class="n">self</span><span class="o">.</span><span class="n">value</span>
<span class="p">}</span>

<span class="k">func</span> <span class="p">(</span><span class="n">self</span> <span class="o">*</span><span class="n">Node</span><span class="p">)</span> <span class="n">GetLeftChild</span><span class="p">()</span> <span class="o">*</span><span class="n">Node</span> <span class="p">{</span>
  <span class="k">return</span> <span class="n">self</span><span class="o">.</span><span class="n">leftChild</span>
<span class="p">}</span>
</code></pre>

<pre><code data-language="go" data-highlighter="tree-sitter"><span class="ts-keyword">type</span> <span class="ts-type">Node</span> <span class="ts-keyword">struct</span> {
   <span class="ts-property">value</span> <span class="ts-type">string</span>
   <span class="ts-property">height</span> <span class="ts-type">uint32</span>
   <span class="ts-property">leftChild</span> <span class="ts-operator">*</span><span class="ts-type">Node</span>
   <span class="ts-property">rightChild</span> <span class="ts-operator">*</span><span class="ts-type">Node</span>
}

<span class="ts-keyword">func</span> <span class="ts-function">NewNode</span>(<span class="ts-variable">value</span> <span class="ts-type">string</span>, <span class="ts-variable">height</span> <span class="ts-type">uint32</span>) <span class="ts-type">Node</span> {
  <span class="ts-keyword">return</span> <span class="ts-type">Node</span>{<span class="ts-variable">value</span>: <span class="ts-variable">value</span>, <span class="ts-variable">height</span>: <span class="ts-variable">height</span>}
}

<span class="ts-keyword">func</span> (<span class="ts-variable">self</span> <span class="ts-operator">*</span><span class="ts-type">Node</span>) <span class="ts-function-method">GetValue</span>() <span class="ts-type">string</span> {
  <span class="ts-keyword">return</span> <span class="ts-variable">self</span>.<span class="ts-property">value</span>
}

<span class="ts-keyword">func</span> (<span class="ts-variable">self</span> <span class="ts-operator">*</span><span class="ts-type">Node</span>) <span class="ts-function-method">GetLeftChild</span>() <span class="ts-operator">*</span><span class="ts-type">Node</span> {
  <span class="ts-keyword">return</span> <span class="ts-variable">self</span>.<span class="ts-property">leftChild</span>
}
</code></pre>

</div>
