{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-2273307019" id="sht-2273307019-1" checked>
  <label for="sht-2273307019-1">Rouge</label>
  <input type="radio" name="sht-2273307019" id="sht-2273307019-2">
  <label for="sht-2273307019-2">Tree-sitter</label>
<pre><code data-language="rust" data-highlighter="rouge"><span class="k">struct</span> <span class="n">Node</span> <span class="p">{</span>
    <span class="n">value</span><span class="p">:</span> <span class="nb">String</span><span class="p">,</span>
    <span class="n">height</span><span class="p">:</span> <span class="nb">u32</span><span class="p">,</span>
    <span class="n">left_child</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="nb">Rc</span><span class="o">&lt;</span><span class="n">Node</span><span class="o">&gt;&gt;</span><span class="p">,</span>
    <span class="n">right_child</span><span class="p">:</span> <span class="nb">Option</span><span class="o">&lt;</span><span class="nb">Rc</span><span class="o">&lt;</span><span class="n">Node</span><span class="o">&gt;&gt;</span><span class="p">,</span>
<span class="p">}</span>

<span class="k">impl</span> <span class="n">Node</span> <span class="p">{</span>
    <span class="k">pub</span> <span class="k">fn</span> <span class="nf">new</span><span class="p">(</span><span class="n">value</span><span class="p">:</span> <span class="nb">String</span><span class="p">,</span> <span class="n">height</span><span class="p">:</span> <span class="nb">u32</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="n">Node</span> <span class="p">{</span>
        <span class="n">Node</span> <span class="p">{</span><span class="n">value</span><span class="p">,</span> <span class="n">height</span><span class="p">,</span> <span class="n">left_child</span><span class="p">:</span> <span class="nb">None</span><span class="p">,</span> <span class="n">right_child</span><span class="p">:</span> <span class="nb">None</span><span class="p">}</span>
    <span class="p">}</span>

    <span class="k">pub</span> <span class="k">fn</span> <span class="nf">get_value</span><span class="p">(</span><span class="o">&amp;</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="o">&amp;</span><span class="nb">str</span> <span class="p">{</span>
        <span class="k">self</span><span class="py">.value</span><span class="nf">.as_str</span><span class="p">()</span>
    <span class="p">}</span>

    <span class="k">pub</span> <span class="k">fn</span> <span class="nf">get_left_child</span><span class="p">(</span><span class="o">&amp;</span><span class="k">self</span><span class="p">)</span> <span class="k">-&gt;</span> <span class="nb">Option</span><span class="o">&lt;&amp;</span><span class="n">Node</span><span class="o">&gt;</span> <span class="p">{</span>
        <span class="k">self</span><span class="py">.left_child</span><span class="nf">.as_ref</span><span class="p">()</span><span class="nf">.map</span><span class="p">(|</span><span class="n">child</span><span class="p">|</span> <span class="n">child</span><span class="nf">.as_ref</span><span class="p">())</span>
    <span class="p">}</span>
<span class="p">}</span>
</code></pre>

<pre><code data-language="rust" data-highlighter="tree-sitter"><span class="ts-keyword">struct</span> <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-property">value</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type">String</span><span class="ts-punctuation-delimiter">,</span>
    <span class="ts-property">height</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type-builtin">u32</span><span class="ts-punctuation-delimiter">,</span>
    <span class="ts-property">left_child</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type">Option</span><span class="ts-punctuation-bracket">&lt;</span><span class="ts-type">Rc</span><span class="ts-punctuation-bracket">&lt;</span><span class="ts-type">Node</span><span class="ts-punctuation-bracket">&gt;</span><span class="ts-punctuation-bracket">&gt;</span><span class="ts-punctuation-delimiter">,</span>
    <span class="ts-property">right_child</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type">Option</span><span class="ts-punctuation-bracket">&lt;</span><span class="ts-type">Rc</span><span class="ts-punctuation-bracket">&lt;</span><span class="ts-type">Node</span><span class="ts-punctuation-bracket">&gt;</span><span class="ts-punctuation-bracket">&gt;</span><span class="ts-punctuation-delimiter">,</span>
<span class="ts-punctuation-bracket">}</span>

<span class="ts-keyword">impl</span> <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-keyword">pub</span> <span class="ts-keyword">fn</span> <span class="ts-function">new</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">value</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type">String</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-variable-parameter">height</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-type-builtin">u32</span><span class="ts-punctuation-bracket">)</span> -&gt; <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>
        <span class="ts-type">Node</span> <span class="ts-punctuation-bracket">{</span>value<span class="ts-punctuation-delimiter">,</span> height<span class="ts-punctuation-delimiter">,</span> <span class="ts-property">left_child</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-constructor">None</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-property">right_child</span><span class="ts-punctuation-delimiter">:</span> <span class="ts-constructor">None</span><span class="ts-punctuation-bracket">}</span>
    <span class="ts-punctuation-bracket">}</span>

    <span class="ts-keyword">pub</span> <span class="ts-keyword">fn</span> <span class="ts-function">get_value</span><span class="ts-punctuation-bracket">(</span><span class="ts-operator">&amp;</span><span class="ts-variable-builtin">self</span><span class="ts-punctuation-bracket">)</span> -&gt; <span class="ts-operator">&amp;</span><span class="ts-type-builtin">str</span> <span class="ts-punctuation-bracket">{</span>
        <span class="ts-variable-builtin">self</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">value</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">as_str</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span>
    <span class="ts-punctuation-bracket">}</span>

    <span class="ts-keyword">pub</span> <span class="ts-keyword">fn</span> <span class="ts-function">get_left_child</span><span class="ts-punctuation-bracket">(</span><span class="ts-operator">&amp;</span><span class="ts-variable-builtin">self</span><span class="ts-punctuation-bracket">)</span> -&gt; <span class="ts-type">Option</span><span class="ts-punctuation-bracket">&lt;</span><span class="ts-operator">&amp;</span><span class="ts-type">Node</span><span class="ts-punctuation-bracket">&gt;</span> <span class="ts-punctuation-bracket">{</span>
        <span class="ts-variable-builtin">self</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">left_child</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">as_ref</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">map</span><span class="ts-punctuation-bracket">(</span>|child| child<span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">as_ref</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-bracket">)</span>
    <span class="ts-punctuation-bracket">}</span>
<span class="ts-punctuation-bracket">}</span>
</code></pre>

</div>
