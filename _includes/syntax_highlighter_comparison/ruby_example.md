{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-3489881573" id="sht-3489881573-1" checked>
  <label for="sht-3489881573-1">Rouge</label>
  <input type="radio" name="sht-3489881573" id="sht-3489881573-2">
  <label for="sht-3489881573-2">Tree-sitter</label>
<pre><code data-language="ruby" data-highlighter="rouge"><span class="k">def</span> <span class="nf">process_list</span><span class="p">(</span><span class="n">list</span><span class="p">)</span>
  <span class="n">context</span> <span class="o">=</span> <span class="n">current_context</span>
  <span class="n">list</span><span class="p">.</span><span class="nf">map</span> <span class="k">do</span> <span class="o">|</span><span class="n">item</span><span class="o">|</span>
    <span class="n">process_item</span><span class="p">(</span><span class="n">item</span><span class="p">,</span> <span class="n">context</span><span class="p">)</span>
  <span class="k">end</span>
<span class="k">end</span>

<span class="n">item</span> <span class="o">=</span> <span class="mi">5</span>
<span class="n">list</span> <span class="o">=</span> <span class="p">[</span><span class="n">item</span><span class="p">]</span>
</code></pre>

<pre><code data-language="ruby" data-highlighter="tree-sitter"><span class="ts-keyword">def</span> <span class="ts-function-method">process_list</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">list</span><span class="ts-punctuation-bracket">)</span>
  <span class="ts-variable">context</span> <span class="ts-operator">=</span> <span class="ts-function-method">current_context</span>
  <span class="ts-variable-parameter">list</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">map</span> <span class="ts-keyword">do</span> |<span class="ts-variable-parameter">item</span>|
    <span class="ts-function-method">process_item</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">item</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-variable">context</span><span class="ts-punctuation-bracket">)</span>
  <span class="ts-keyword">end</span>
<span class="ts-keyword">end</span>

<span class="ts-variable">item</span> <span class="ts-operator">=</span> <span class="ts-number">5</span>
<span class="ts-variable">list</span> <span class="ts-operator">=</span> <span class="ts-punctuation-bracket">[</span><span class="ts-variable">item</span><span class="ts-punctuation-bracket">]</span>
</code></pre>

</div>
