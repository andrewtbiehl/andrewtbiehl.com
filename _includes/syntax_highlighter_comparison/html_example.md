{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-2552661169" id="sht-2552661169-1" checked>
  <label for="sht-2552661169-1">Rouge</label>
  <input type="radio" name="sht-2552661169" id="sht-2552661169-2">
  <label for="sht-2552661169-2">Tree-sitter</label>
<pre><code data-language="html" data-highlighter="rouge"><span class="cp">&lt;!DOCTYPE html&gt;</span>
<span class="nt">&lt;html&gt;</span>
   <span class="nt">&lt;body&gt;</span>
      <span class="nt">&lt;button</span> <span class="na">onclick=</span><span class="s">"updateButtonInfo()"</span><span class="nt">&gt;</span>Click me<span class="nt">&lt;/button&gt;</span>
      <span class="nt">&lt;p</span> <span class="na">id=</span><span class="s">"button-info"</span><span class="nt">&gt;</span>Not yet clicked.<span class="nt">&lt;/p&gt;</span>
      <span class="nt">&lt;script&gt;</span>
         <span class="kd">function</span> <span class="nx">updateButtonInfo</span><span class="p">()</span> <span class="p">{</span>
           <span class="nb">document</span><span class="p">.</span><span class="nx">getElementById</span><span class="p">(</span><span class="dl">"</span><span class="s2">button-info</span><span class="dl">"</span><span class="p">).</span><span class="nx">innerHTML</span> <span class="o">=</span> <span class="dl">"</span><span class="s2">Clicked!</span><span class="dl">"</span><span class="p">;</span>
         <span class="p">}</span>
      <span class="nt">&lt;/script&gt;</span>
   <span class="nt">&lt;/body&gt;</span>
<span class="nt">&lt;/html&gt;</span>
</code></pre>

<pre><code data-language="html" data-highlighter="tree-sitter"><span class="ts-constant">&lt;!DOCTYPE html<span class="ts-punctuation-bracket">&gt;</span></span>
<span class="ts-punctuation-bracket">&lt;</span><span class="ts-tag">html</span><span class="ts-punctuation-bracket">&gt;</span>
   <span class="ts-punctuation-bracket">&lt;</span><span class="ts-tag">body</span><span class="ts-punctuation-bracket">&gt;</span>
      <span class="ts-punctuation-bracket">&lt;</span><span class="ts-tag">button</span> <span class="ts-attribute">onclick</span>="<span class="ts-string">updateButtonInfo()</span>"<span class="ts-punctuation-bracket">&gt;</span>Click me<span class="ts-punctuation-bracket">&lt;/</span><span class="ts-tag">button</span><span class="ts-punctuation-bracket">&gt;</span>
      <span class="ts-punctuation-bracket">&lt;</span><span class="ts-tag">p</span> <span class="ts-attribute">id</span>="<span class="ts-string">button-info</span>"<span class="ts-punctuation-bracket">&gt;</span>Not yet clicked.<span class="ts-punctuation-bracket">&lt;/</span><span class="ts-tag">p</span><span class="ts-punctuation-bracket">&gt;</span>
      <span class="ts-punctuation-bracket">&lt;</span><span class="ts-tag">script</span><span class="ts-punctuation-bracket">&gt;</span>
         <span class="ts-keyword">function</span> <span class="ts-function">updateButtonInfo</span><span class="ts-punctuation-bracket">(</span><span class="ts-punctuation-bracket">)</span> <span class="ts-punctuation-bracket">{</span>
           <span class="ts-variable-builtin">document</span><span class="ts-punctuation-delimiter">.</span><span class="ts-function-method">getElementById</span><span class="ts-punctuation-bracket">(</span><span class="ts-string">"button-info"</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-delimiter">.</span><span class="ts-property">innerHTML</span> <span class="ts-operator">=</span> <span class="ts-string">"Clicked!"</span><span class="ts-punctuation-delimiter">;</span>
         <span class="ts-punctuation-bracket">}</span>
      <span class="ts-punctuation-bracket">&lt;/</span><span class="ts-tag">script</span><span class="ts-punctuation-bracket">&gt;</span>
   <span class="ts-punctuation-bracket">&lt;/</span><span class="ts-tag">body</span><span class="ts-punctuation-bracket">&gt;</span>
<span class="ts-punctuation-bracket">&lt;/</span><span class="ts-tag">html</span><span class="ts-punctuation-bracket">&gt;</span>
</code></pre>

</div>
