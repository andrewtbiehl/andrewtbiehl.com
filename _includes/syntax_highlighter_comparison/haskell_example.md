{% comment %} HTML snippet originally copied from generated site {% endcomment %}
<div class="syntax-highlighter-tabs">
  <input type="radio" name="sht-225082566" id="sht-225082566-1" checked>
  <label for="sht-225082566-1">Rouge</label>
  <input type="radio" name="sht-225082566" id="sht-225082566-2">
  <label for="sht-225082566-2">Tree-sitter</label>
<pre><code data-language="haskell" data-highlighter="rouge"><span class="kr">import</span> <span class="nn">Control.Monad</span> <span class="p">(</span><span class="nf">mfilter</span><span class="p">)</span>
<span class="kr">import</span> <span class="nn">Data.Maybe</span> <span class="p">(</span><span class="nf">isJust</span><span class="p">)</span>
<span class="kr">import</span> <span class="nn">Text.Read</span> <span class="p">(</span><span class="nf">readMaybe</span><span class="p">)</span>

<span class="n">digitSum</span> <span class="o">::</span> <span class="kt">Integer</span> <span class="o">-&gt;</span> <span class="kt">Integer</span>
<span class="n">digitSum</span> <span class="mi">0</span> <span class="o">=</span> <span class="mi">0</span>
<span class="n">digitSum</span> <span class="n">n</span> <span class="o">=</span> <span class="p">(</span><span class="n">n</span> <span class="p">`</span><span class="n">mod</span><span class="p">`</span> <span class="mi">10</span><span class="p">)</span> <span class="o">+</span> <span class="n">digitSum</span> <span class="p">(</span><span class="n">n</span> <span class="p">`</span><span class="n">div</span><span class="p">`</span> <span class="mi">10</span><span class="p">)</span>

<span class="n">checkDigit</span> <span class="o">::</span> <span class="p">[</span><span class="kt">Integer</span><span class="p">]</span> <span class="o">-&gt;</span> <span class="kt">Integer</span>
<span class="n">checkDigit</span> <span class="o">=</span> <span class="p">(</span><span class="mi">10</span> <span class="o">-</span><span class="p">)</span> <span class="o">.</span> <span class="p">(`</span><span class="n">mod</span><span class="p">`</span> <span class="mi">10</span><span class="p">)</span> <span class="o">.</span> <span class="n">checksum</span>
  <span class="kr">where</span>
    <span class="n">checksum</span> <span class="o">=</span> <span class="n">sum</span> <span class="o">.</span> <span class="n">map</span> <span class="n">digitSum</span> <span class="o">.</span> <span class="n">doubleEveryOther</span>
    <span class="n">doubleEveryOther</span> <span class="o">=</span> <span class="n">zipWith</span> <span class="p">(</span><span class="o">$</span><span class="p">)</span> <span class="p">(</span><span class="n">cycle</span> <span class="p">[</span><span class="n">id</span><span class="p">,</span> <span class="p">(</span><span class="o">*</span> <span class="mi">2</span><span class="p">)])</span>

<span class="n">isValidLuhnSequence</span> <span class="o">::</span> <span class="p">[</span><span class="kt">Integer</span><span class="p">]</span> <span class="o">-&gt;</span> <span class="kt">Bool</span>
<span class="n">isValidLuhnSequence</span> <span class="o">=</span> <span class="p">(</span><span class="o">==</span><span class="p">)</span> <span class="o">&lt;$&gt;</span> <span class="n">calculatedCheckDigit</span> <span class="o">&lt;*&gt;</span> <span class="n">givenCheckDigit</span>
  <span class="kr">where</span>
    <span class="n">givenCheckDigit</span> <span class="o">=</span> <span class="n">last</span>
    <span class="n">calculatedCheckDigit</span> <span class="o">=</span> <span class="n">checkDigit</span> <span class="o">.</span> <span class="n">init</span>

<span class="n">main</span> <span class="o">=</span> <span class="kr">do</span>
  <span class="n">putStrLn</span> <span class="s">"Input a number to validate:"</span>
  <span class="n">input</span> <span class="o">&lt;-</span> <span class="n">getLine</span>
  <span class="kr">let</span> <span class="n">response</span> <span class="o">=</span> <span class="kr">if</span> <span class="n">isValidLuhnNumber</span> <span class="n">input</span> <span class="kr">then</span> <span class="s">"Valid!"</span> <span class="kr">else</span> <span class="s">"Not valid."</span>
  <span class="n">putStrLn</span> <span class="n">response</span>
  <span class="kr">where</span>
    <span class="n">isValidLuhnNumber</span> <span class="o">=</span> <span class="n">isJust</span> <span class="o">.</span> <span class="p">(</span><span class="n">mfilter</span> <span class="n">isValidLuhnSequence</span><span class="p">)</span> <span class="o">.</span> <span class="n">digits</span>
    <span class="n">digits</span> <span class="o">=</span> <span class="n">mapM</span> <span class="n">readMaybe</span> <span class="o">.</span> <span class="n">map</span> <span class="p">(</span><span class="nf">\</span><span class="n">c</span> <span class="o">-&gt;</span> <span class="p">[</span><span class="n">c</span><span class="p">])</span>
</code></pre>

<pre><code data-language="haskell" data-highlighter="tree-sitter"><span class="ts-include">import</span> <span class="ts-operator"><span class="ts-namespace">Control</span>.<span class="ts-namespace">Monad</span></span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable">mfilter</span><span class="ts-punctuation-bracket">)</span>
<span class="ts-include">import</span> <span class="ts-operator"><span class="ts-namespace">Data</span>.<span class="ts-namespace">Maybe</span></span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable">isJust</span><span class="ts-punctuation-bracket">)</span>
<span class="ts-include">import</span> <span class="ts-operator"><span class="ts-namespace">Text</span>.<span class="ts-namespace">Read</span></span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable">readMaybe</span><span class="ts-punctuation-bracket">)</span>

<span class="ts-variable">digitSum</span> <span class="ts-operator">::</span> <span class="ts-type">Integer</span> <span class="ts-operator">-&gt;</span> <span class="ts-type">Integer</span>
<span class="ts-variable">digitSum</span> <span class="ts-number">0</span> <span class="ts-operator">=</span> <span class="ts-number">0</span>
<span class="ts-variable">digitSum</span> <span class="ts-variable">n</span> <span class="ts-operator">=</span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable"><span class="ts-function">n</span></span> <span class="ts-operator">`</span><span class="ts-variable">mod</span><span class="ts-operator">`</span> <span class="ts-number">10</span><span class="ts-punctuation-bracket">)</span> <span class="ts-operator">+</span> <span class="ts-variable">digitSum</span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable"><span class="ts-function">n</span></span> <span class="ts-operator">`</span><span class="ts-variable">div</span><span class="ts-operator">`</span> <span class="ts-number">10</span><span class="ts-punctuation-bracket">)</span>

<span class="ts-variable">checkDigit</span> <span class="ts-operator">::</span> <span class="ts-punctuation-bracket">[</span><span class="ts-type">Integer</span><span class="ts-punctuation-bracket">]</span> <span class="ts-operator">-&gt;</span> <span class="ts-type">Integer</span>
<span class="ts-variable">checkDigit</span> <span class="ts-operator">=</span> <span class="ts-punctuation-bracket">(</span><span class="ts-number">10</span> <span class="ts-operator">-</span><span class="ts-punctuation-bracket">)</span> <span class="ts-operator">.</span> <span class="ts-punctuation-bracket">(</span><span class="ts-operator">`</span><span class="ts-variable">mod</span><span class="ts-operator">`</span> <span class="ts-number">10</span><span class="ts-punctuation-bracket">)</span> <span class="ts-operator">.</span> <span class="ts-variable"><span class="ts-function">checksum</span></span>
  <span class="ts-keyword">where</span>
    <span class="ts-variable">checksum</span> <span class="ts-operator">=</span> <span class="ts-variable"><span class="ts-function">sum</span></span> <span class="ts-operator">.</span> <span class="ts-variable">map</span> <span class="ts-variable">digitSum</span> <span class="ts-operator">.</span> <span class="ts-variable"><span class="ts-function">doubleEveryOther</span></span>
    <span class="ts-variable">doubleEveryOther</span> <span class="ts-operator">=</span> <span class="ts-variable">zipWith</span> <span class="ts-punctuation-bracket">(</span><span class="ts-operator">$</span><span class="ts-punctuation-bracket">)</span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable">cycle</span> <span class="ts-punctuation-bracket">[</span><span class="ts-variable">id</span><span class="ts-punctuation-delimiter">,</span> <span class="ts-punctuation-bracket">(</span><span class="ts-operator">*</span> <span class="ts-number">2</span><span class="ts-punctuation-bracket">)</span><span class="ts-punctuation-bracket">]</span><span class="ts-punctuation-bracket">)</span>

<span class="ts-variable">isValidLuhnSequence</span> <span class="ts-operator">::</span> <span class="ts-punctuation-bracket">[</span><span class="ts-type">Integer</span><span class="ts-punctuation-bracket">]</span> <span class="ts-operator">-&gt;</span> <span class="ts-type">Bool</span>
<span class="ts-variable">isValidLuhnSequence</span> <span class="ts-operator">=</span> <span class="ts-punctuation-bracket"><span class="ts-function">(<span class="ts-operator">==</span><span class="ts-punctuation-bracket">)</span></span></span> <span class="ts-operator">&lt;$&gt;</span> <span class="ts-variable"><span class="ts-function">calculatedCheckDigit</span></span> <span class="ts-operator">&lt;*&gt;</span> <span class="ts-variable"><span class="ts-function">givenCheckDigit</span></span>
  <span class="ts-keyword">where</span>
    <span class="ts-variable">givenCheckDigit</span> <span class="ts-operator">=</span> <span class="ts-variable">last</span>
    <span class="ts-variable">calculatedCheckDigit</span> <span class="ts-operator">=</span> <span class="ts-variable"><span class="ts-function">checkDigit</span></span> <span class="ts-operator">.</span> <span class="ts-variable"><span class="ts-function">init</span></span>

<span class="ts-variable">main</span> <span class="ts-operator">=</span> <span class="ts-keyword">do</span>
  <span class="ts-variable">putStrLn</span> <span class="ts-string">"Input a number to validate:"</span>
  <span class="ts-variable">input</span> <span class="ts-operator">&lt;-</span> <span class="ts-variable">getLine</span>
  <span class="ts-keyword">let</span> <span class="ts-variable">response</span> <span class="ts-operator">=</span> <span class="ts-conditional">if</span> <span class="ts-variable">isValidLuhnNumber</span> <span class="ts-variable">input</span> <span class="ts-conditional">then</span> <span class="ts-string">"Valid!"</span> <span class="ts-conditional">else</span> <span class="ts-string">"Not valid."</span>
  <span class="ts-variable">putStrLn</span> <span class="ts-variable">response</span>
  <span class="ts-keyword">where</span>
    <span class="ts-variable">isValidLuhnNumber</span> <span class="ts-operator">=</span> <span class="ts-variable"><span class="ts-function">isJust</span></span> <span class="ts-operator">.</span> <span class="ts-punctuation-bracket">(</span><span class="ts-variable">mfilter</span> <span class="ts-variable">isValidLuhnSequence</span><span class="ts-punctuation-bracket">)</span> <span class="ts-operator">.</span> <span class="ts-variable"><span class="ts-function">digits</span></span>
    <span class="ts-variable">digits</span> <span class="ts-operator">=</span> <span class="ts-variable">mapM</span> <span class="ts-variable">readMaybe</span> <span class="ts-operator">.</span> <span class="ts-variable">map</span> <span class="ts-punctuation-bracket">(</span><span class="ts-operator">\</span><span class="ts-variable">c</span> <span class="ts-operator">-&gt;</span> <span class="ts-punctuation-bracket">[</span><span class="ts-variable">c</span><span class="ts-punctuation-bracket">]</span><span class="ts-punctuation-bracket">)</span>
</code></pre>

</div>
