{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-3874755989" id="sht-3874755989-1" checked>
  <label for="sht-3874755989-1">Rouge</label>
  <input type="radio" name="sht-3874755989" id="sht-3874755989-2">
  <label for="sht-3874755989-2">Tree-sitter</label>
<pre><code data-language="typescript" data-highlighter="rouge"><span class="kd">class</span> <span class="nx">ListNode</span> <span class="p">{</span>
  <span class="k">private</span> <span class="nx">value</span><span class="p">:</span> <span class="kr">string</span><span class="p">;</span>
  <span class="k">private</span> <span class="nx">next</span><span class="p">:</span> <span class="nx">ListNode</span> <span class="o">|</span> <span class="kc">null</span><span class="p">;</span>

  <span class="kd">constructor</span><span class="p">(</span><span class="nx">value</span><span class="p">:</span> <span class="kr">string</span><span class="p">,</span> <span class="nx">next</span><span class="p">?:</span> <span class="nx">ListNode</span><span class="p">)</span> <span class="p">{</span>
    <span class="k">this</span><span class="p">.</span><span class="nx">value</span> <span class="o">=</span> <span class="nx">value</span><span class="p">;</span>
    <span class="k">this</span><span class="p">.</span><span class="nx">next</span> <span class="o">=</span> <span class="nx">next</span> <span class="o">??</span> <span class="kc">null</span><span class="p">;</span>
  <span class="p">}</span>

  <span class="nx">getValue</span><span class="p">()</span> <span class="p">:</span> <span class="kr">string</span> <span class="p">{</span>
    <span class="k">return</span> <span class="k">this</span><span class="p">.</span><span class="nx">value</span><span class="p">;</span>
  <span class="p">}</span>

  <span class="nx">getNext</span><span class="p">()</span> <span class="p">:</span> <span class="nx">ListNode</span> <span class="o">|</span> <span class="kc">null</span> <span class="p">{</span>
    <span class="k">return</span> <span class="k">this</span><span class="p">.</span><span class="nx">next</span><span class="p">;</span>
  <span class="p">}</span>
<span class="p">}</span>

<span class="kd">function</span> <span class="nx">commaSeparate</span><span class="p">(</span><span class="nx">head</span><span class="p">:</span> <span class="nx">ListNode</span> <span class="o">|</span> <span class="kc">null</span><span class="p">)</span> <span class="p">:</span> <span class="kr">string</span> <span class="p">{</span>
  <span class="kd">let</span> <span class="nx">values</span> <span class="o">=</span> <span class="p">[];</span>
  <span class="k">while</span> <span class="p">(</span><span class="nx">head</span> <span class="o">!=</span> <span class="kc">null</span><span class="p">)</span> <span class="p">{</span>
    <span class="nx">values</span><span class="p">.</span><span class="nx">push</span><span class="p">(</span><span class="nx">head</span><span class="p">.</span><span class="nx">getValue</span><span class="p">());</span>
    <span class="nx">head</span> <span class="o">=</span> <span class="nx">head</span><span class="p">.</span><span class="nx">getNext</span><span class="p">();</span>
  <span class="p">}</span>
  <span class="k">return</span> <span class="nx">values</span><span class="p">.</span><span class="nx">join</span><span class="p">(</span><span class="dl">"</span><span class="s2">, </span><span class="dl">"</span><span class="p">);</span>
<span class="p">}</span>
</code></pre>

<pre><code data-language="typescript" data-highlighter="tree-sitter"><span class="ts-keyword">class</span> <span class="ts-type">ListNode</span> <span class="ts-punctuation-bracket">{</span>
  <span class="ts-keyword">private</span> <span class="ts-property">value</span>: <span class="ts-type-builtin">string</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-keyword">private</span> <span class="ts-property">next</span>: <span class="ts-type">ListNode</span> <span class="ts-operator">|</span> <span class="ts-constant-builtin">null</span><span class="ts-punctuation-delimiter">;</span>

  <span class="ts-function-method">constructor</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">value</span>: <span class="ts-type-builtin">string</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-variable-parameter">next</span>?: <span class="ts-type">ListNode</span><span class="ts-punctuation-bracket">)</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">value</span> <span class="ts-operator">=</span> <span class="ts-variable-parameter">value</span><span class="ts-punctuation-delimiter">;</span>
    <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">next</span> <span class="ts-operator">=</span> <span class="ts-variable-parameter">next</span> <span class="ts-operator">??</span> <span class="ts-constant-builtin">null</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>

  <span class="ts-function-method">getValue</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span> : <span class="ts-type-builtin">string</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-keyword">return</span> <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">value</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>

  <span class="ts-function-method">getNext</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span> : <span class="ts-type">ListNode</span> <span class="ts-operator">|</span> <span class="ts-constant-builtin">null</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-keyword">return</span> <span class="ts-variable-builtin">this</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">next</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>
<span class="ts-punctuation-bracket">}</span>

<span class="ts-keyword">function</span> <span class="ts-function">commaSeparate</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">head</span>: <span class="ts-type">ListNode</span> <span class="ts-operator">|</span> <span class="ts-constant-builtin">null</span><span class="ts-punctuation-bracket">)</span> : <span class="ts-type-builtin">string</span> <span class="ts-punctuation-bracket">{</span>
  <span class="ts-keyword">let</span> <span class="ts-variable">values</span> <span class="ts-operator">=</span> <span class="ts-punctuation-bracket">[</span><span class="ts-punctuation-bracket">]</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-keyword">while</span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">head</span> <span class="ts-operator">!=</span> <span class="ts-constant-builtin">null</span><span class="ts-punctuation-bracket">)</span> <span class="ts-punctuation-bracket">{</span>
    <span class="ts-variable">values</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">push</span><span class="ts-punctuation-bracket">(</span><span class="ts-variable-parameter">head</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">getValue</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-delimiter">;</span>
    <span class="ts-variable-parameter">head</span> <span class="ts-operator">=</span> <span class="ts-variable-parameter">head</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">getNext</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-delimiter">;</span>
  <span class="ts-punctuation-bracket">}</span>
  <span class="ts-keyword">return</span> <span class="ts-variable">values</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">join</span><span class="ts-punctuation-bracket">(</span><span class="ts-string">", "</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-delimiter">;</span>
<span class="ts-punctuation-bracket">}</span>
</code></pre>

</div>
