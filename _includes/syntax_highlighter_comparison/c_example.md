{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-2086367422" id="sht-2086367422-1" checked>
  <label for="sht-2086367422-1">Rouge</label>
  <input type="radio" name="sht-2086367422" id="sht-2086367422-2">
  <label for="sht-2086367422-2">Tree-sitter</label>
<pre><code data-language="c" data-highlighter="rouge"><span class="k">typedef</span> <span class="k">struct</span> <span class="p">{</span>
  <span class="k">const</span> <span class="kt">char</span> <span class="o">*</span><span class="n">value</span><span class="p">;</span>
  <span class="kt">uint32_t</span> <span class="n">depth</span><span class="p">;</span>
  <span class="n">Node</span> <span class="o">*</span><span class="n">left_child</span><span class="p">;</span>
  <span class="n">Node</span> <span class="o">*</span><span class="n">right_child</span><span class="p">;</span>
<span class="p">}</span> <span class="n">Node</span><span class="p">;</span>

<span class="n">Node</span> <span class="nf">node_new</span><span class="p">(</span><span class="k">const</span> <span class="kt">char</span> <span class="o">*</span><span class="n">value</span><span class="p">,</span> <span class="kt">uint32_t</span> <span class="n">depth</span><span class="p">)</span> <span class="p">{</span>
  <span class="k">return</span> <span class="p">(</span><span class="n">Node</span><span class="p">)</span> <span class="p">{.</span><span class="n">value</span> <span class="o">=</span> <span class="n">value</span><span class="p">,</span> <span class="p">.</span><span class="n">depth</span> <span class="o">=</span> <span class="n">depth</span><span class="p">};</span>
<span class="p">}</span>

<span class="k">const</span> <span class="kt">char</span> <span class="o">*</span><span class="nf">node_value</span><span class="p">(</span><span class="n">Node</span> <span class="o">*</span><span class="n">self</span><span class="p">)</span> <span class="p">{</span>
  <span class="k">return</span> <span class="n">self</span><span class="o">-&gt;</span><span class="n">value</span><span class="p">;</span>
<span class="p">}</span>

<span class="n">Node</span> <span class="o">*</span><span class="nf">node_left_child</span><span class="p">(</span><span class="n">Node</span> <span class="o">*</span><span class="n">self</span><span class="p">)</span> <span class="p">{</span>
  <span class="k">return</span> <span class="n">self</span><span class="o">-&gt;</span><span class="n">left_child</span><span class="p">;</span>
<span class="p">}</span>
</code></pre>

<pre><code data-language="c" data-highlighter="tree-sitter"><span class="ts-keyword">typedef</span> <span class="ts-keyword">struct</span> {
  <span class="ts-keyword">const</span> <span class="ts-type">char</span> <span class="ts-operator">*</span><span class="ts-property">value</span><span class="ts-delimiter">;</span>
  <span class="ts-type">uint32_t</span> <span class="ts-property">depth</span><span class="ts-delimiter">;</span>
  <span class="ts-type">Node</span> <span class="ts-operator">*</span><span class="ts-property">left_child</span><span class="ts-delimiter">;</span>
  <span class="ts-type">Node</span> <span class="ts-operator">*</span><span class="ts-property">right_child</span><span class="ts-delimiter">;</span>
} <span class="ts-type">Node</span><span class="ts-delimiter">;</span>

<span class="ts-type">Node</span> <span class="ts-function">node_new</span>(<span class="ts-keyword">const</span> <span class="ts-type">char</span> <span class="ts-operator">*</span><span class="ts-variable">value</span>, <span class="ts-type">uint32_t</span> <span class="ts-variable">depth</span>) {
  <span class="ts-keyword">return</span> (<span class="ts-type">Node</span>) {<span class="ts-delimiter">.</span><span class="ts-property">value</span> <span class="ts-operator">=</span> <span class="ts-variable">value</span>, <span class="ts-delimiter">.</span><span class="ts-property">depth</span> <span class="ts-operator">=</span> <span class="ts-variable">depth</span>}<span class="ts-delimiter">;</span>
}

<span class="ts-keyword">const</span> <span class="ts-type">char</span> <span class="ts-operator">*</span><span class="ts-function">node_value</span>(<span class="ts-type">Node</span> <span class="ts-operator">*</span><span class="ts-variable">self</span>) {
  <span class="ts-keyword">return</span> <span class="ts-variable">self</span><span class="ts-operator">-&gt;</span><span class="ts-property">value</span><span class="ts-delimiter">;</span>
}

<span class="ts-type">Node</span> <span class="ts-operator">*</span><span class="ts-function">node_left_child</span>(<span class="ts-type">Node</span> <span class="ts-operator">*</span><span class="ts-variable">self</span>) {
  <span class="ts-keyword">return</span> <span class="ts-variable">self</span><span class="ts-operator">-&gt;</span><span class="ts-property">left_child</span><span class="ts-delimiter">;</span>
}
</code></pre>

</div>
