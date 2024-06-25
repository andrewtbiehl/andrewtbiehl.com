---
layout: post
title: "Syntax highlight your Jekyll site with Tree-sitter!"
permalink: "/blog/jekyll-tree-sitter"
---

***2024-06-25 Update:*** Unsurprisingly, the syntax highlighting tools referenced and
reviewed in this post have continued to develop since its publication, rendering some of
the exposition and discussion on their behavior and features outdated. Even so, I have
decided to freeze the syntax highlighting comparisons provided in this post in their
original state so that their associated discussions continue to remain coherent.

***2022-11-11 Update:*** This concept is now a plugin! Now there's no need to
DIY; instead visit
[github.com/andrewtbiehl/kramdown-syntax_tree_sitter](https://github.com/andrewtbiehl/kramdown-syntax_tree_sitter#kramdown-tree-sitter-highlighter)
and
[follow a few steps to integrate the plugin into your Jekyll project](https://github.com/andrewtbiehl/kramdown-syntax_tree_sitter#usage-with-jekyll).

Dear fellow developer: when was the last time you took a good hard look at your syntax
highlighter? If you're like me, you probably never noticed that your average syntax
highlighter likely fails to do its one job all that well. All this changed for me,
however, when I discovered [Tree-sitter](https://tree-sitter.github.io), the open-source
parser and syntax highlighter *of the future*:

{% figure caption: "*Click the tabs to toggle between highlighters.*" %}
{% include syntax_highlighter_comparison/typescript_example_1.md -%}
{% endfigure %}

Now, like any good consumer, as soon as I learn of some fancy new product, I won't be
satisfied until I'm clutching one of my own – and Tree-sitter was certainly no exception
to this. Soon after learning of this better alternative, I resolved that my own
[Jekyll](https://jekyllrb.com) site deserved nothing but the best code highlighting
available. I decided that I'd spend "a few hours" replacing Jekyll's default syntax
highlighter with [the new hotness](https://youtu.be/jn_P13FkQYw) so that my readers and
I could reap the benefits of a modern code browsing experience. After all, I thought to
myself, how hard could it be to integrate Tree-sitter into Jekyll?

Multiple weeks, hundreds of compiler errors, and one newly learned programming language
later, I am relieved to announce that *I got it working*! And now I'm here to share my
experience with you, so that you may get it working on your site too. What's more, you
probably won't even have to pull out your hair in the process! I've already done enough
of that for both of us.

### What's a Jekyll? What's a Tree-sitter?

Before I get to the *how*, I'll pause to briefly reiterate the *what*.

- [Tree-sitter](https://tree-sitter.github.io) is an exciting parsing library created by
  Max Brunsfeld. Its killer functionality is its ability to incrementally parse many
  languages into syntax trees with a standardized interface. Because of this, it boasts
  the ability to support features unattainable by many other parser implementations in
  use today.

  For a deeper look into the library's capabilities, as well as some motivating examples
  of its superior highlighting quality, I strongly recommend
  [Max Brunsfeld's talk at Strange Loop 2018](https://youtu.be/Jes3bD6P0To). This
  fantastic talk is what got me excited about Tree-sitter in the first place.

- [Jekyll](https://jekyllrb.com) is a static site generator and, in particular, the one
  used to build this site. Regarding syntax highlighting, the latest version (v4) of
  Jekyll delegates this responsibility to a library called
  [Rouge](http://rouge.jneen.net) by default, but can be configured to use alternatives.

The inspiration for the project underlying this post was the particularly intriguing
claim that Tree-sitter outperforms many established tools at the task of syntax
highlighting. Reputedly, most general-purpose syntax highlighters rely on
"regex-based systems"[^1] to parse code, even though no practical programming languages
are [regular](https://en.wikipedia.org/wiki/Regular_language) (that is, expressible by
classical regular expressions). In other words, most existing syntax highlighters are
fundamentally incapable of comprehensively parsing the very languages they highlight, a
deficiency that Tree-sitter fortunately avoids.

Hence my goal was to integrate Tree-sitter into Jekyll and thereby enable better syntax
highlighting on this site. After much trial and error, I did eventually succeed in doing
this. The following walkthrough explains how.

### How to use Tree-sitter with Jekyll

After evaluating several potential methods for interoperating with Tree-sitter through
Jekyll, I eventually settled on the following approach: create a local
[Rust](https://www.rust-lang.org) package ("crate") that serves as an adapter for the
[Tree-sitter highlight](https://tree-sitter.github.io/tree-sitter/syntax-highlighting)
library. Next, use a Ruby to Rust binding library (namely,
[Rutie](https://github.com/danielpclark/rutie)) to enable a custom Jekyll plugin to
invoke the adapter during the site build process. It's a bit involved, but this approach
met many personal requirements that I will explain after the walkthrough.

To follow along, **you'll of course need
[Jekyll and its prerequisites installed](https://jekyllrb.com/docs/installation/#requirements)
and a Jekyll site to modify**. In the unlikely event that you're still reading this
despite lacking a Jekyll site, you can create one by following
[Jekyll's quick start guide](https://jekyllrb.com/docs). Presumably, portions of this
method can also be adapted for other static site generators, if you're so inclined. Some
Rust programming is also involved, so **a basic Rust installation is also required**.
Information on installing Rust can be found
[on the Rust website](https://www.rust-lang.org/tools/install).

#### Using Jekyll plugins to customize syntax highlighting

The first step is to create a custom highlighter that will interface with Jekyll.
Fortunately for us, [Jekyll's plugin system](https://jekyllrb.com/docs/plugins) provides
everything we need to do this. Jekyll plugins are essentially arbitrary Ruby scripts
that reside in the `_plugins` directory of a Jekyll project, so create that directory in
the root of your project if it doesn't already exist. Next, add the following plugin to
that directory, for example under the filename `custom_highlighter.rb`:

```ruby
require 'kramdown'

module Kramdown
  module Converter
    module SyntaxHighlighter
      # A "highlighter" that does not actually highlight code.
      #
      # Escapes the code so that it can be safely inserted into HTML text.
      module NoHighlight
        def self.call(converter, text, _, type, _)
          output = converter.escape_html text
          # Code blocks are additionally wrapped in HTML code tags
          type == :block ? "<pre><code>#{output}</code></pre>" : output
        end
      end
    end
  end
end

Kramdown::Converter.add_syntax_highlighter(
  :no_highlight,
  Kramdown::Converter::SyntaxHighlighter::NoHighlight
)
```

This script creates a new custom syntax highlighter called 'NoHighlight', which, as the
name suggests, doesn't actually highlight anything. It does, however, minimally
implement
[the interface of a Jekyll (or, in particular, Kramdown) highlighter](https://kramdown.gettalong.org/rdoc/Kramdown/Converter/SyntaxHighlighter.html#module-Kramdown::Converter::SyntaxHighlighter-label-Implementing+a+Syntax+Highlighter).[^2]
The last four lines in the script register this syntax highlighter under the identifier
`no_highlight` with [Kramdown](https://kramdown.gettalong.org), Jekyll's default[^3]
Markdown renderer.

Next, we need to configure Jekyll to use our new syntax highlighter in place of Rouge.
To do this, append the following lines to the site's Jekyll configuration
(`_config.yml`) file:

```yaml
kramdown:
  syntax_highlighter: no_highlight
```

Finally, one idiosyncratic caveat to this approach is that it requires that we
specifically
[use fenced notation](https://www.markdownguide.org/extended-syntax/#fenced-code-blocks)
(i.e., ```` ``` ````) to define code blocks, so modify your site's existing code blocks
accordingly if necessary.

Now when we build the site we should see the consequence of these changes: code blocks
are no longer highlighted. Not exactly thrilling, but still a small success! Later we'll
swap out this non-highlighter with one that actually uses Tree-sitter under the hood.

#### Interacting with Tree-sitter's Rust bindings

Before we interoperate with Tree-sitter via our Jekyll plugin, we will first
interoperate with it via a homemade Rust library. In my nascent experience as a user of
Tree-sitter, its API (and associated documentation) is currently somewhat rough around
the edges. This, of course, is understandable given that, at the time of writing this,
Tree-sitter is still in initial development (the latest version is v0.20.7). Regardless,
I found it worthwhile to smooth out many of the library's peculiarities behind an
adapter of my own making.

To accomplish this, create a new Rust library in the root of your Jekyll project. For
example, you may use the following command, which directs the Rust package manager,
Cargo, to create a library by the name of 'tree_sitter_ruby_adapter' inside a new
`_tree_sitter_ruby_adapter`[^4] directory.

```console
cargo new --lib --name tree_sitter_ruby_adapter _tree_sitter_ruby_adapter
```
{: .lang-descriptor-hidden}

Next, change directories into this new library and declare the crate's three requisite
Tree-sitter dependencies,
'[tree-sitter-cli](https://github.com/tree-sitter/tree-sitter/tree/master/cli#tree-sitter-cli)',
'[tree-sitter-highlight](https://github.com/tree-sitter/tree-sitter/tree/master/highlight#tree-sitter-highlight)',
and
'[tree-sitter-loader](https://github.com/tree-sitter/tree-sitter/tree/master/cli/loader#tree-sitter-loader)',
for example via the following command:[^5]

```console
cargo add \
  tree-sitter-cli@0.20 \
  tree-sitter-highlight@0.20 \
  tree-sitter-loader@0.20
```
{: .lang-descriptor-hidden}

Now we're ready to build out our library. The following snippet is a self-contained,
minimal working example of interoperating with Tree-sitter's highlight library:

```rust
use tree_sitter_cli::highlight::Theme;
use tree_sitter_highlight::{Highlight, Highlighter, HtmlRenderer};
use tree_sitter_loader::{Config, Loader};

// Adapter function for interoperating with Tree-sitter's highlight library.
//
// * `code` - The code snippet to highlight.
// * `scope` - The TextMate scope identifying the language of the code snippet.
pub fn highlight_adapter(code: &str, scope: &str) -> String {
    // The directory to search for parsers
    let parser_directory = std::env::current_dir()
        .unwrap()
        .join("parsers");

    let theme = Theme::default();

    // The loader is used to load parsers
    let loader = {
        let mut loader = Loader::new().unwrap();
        let config = {
            let parser_directories = vec![parser_directory];
            Config { parser_directories }
        };
        loader.find_all_languages(&config).unwrap();
        loader.configure_highlights(&theme.highlight_names);
        loader
    };

    // Retrieve the highlight config for the given language scope
    let config = loader
        .language_configuration_for_scope(scope)
        .unwrap()
        .and_then(|(language, config)| config.highlight_config(language).ok())
        .unwrap()
        .unwrap();

    let code = code.as_bytes();

    // Highlight the code
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code, None, |_| None)
        .unwrap();

    // Render and return the highlighted code as an HTML snippet
    let get_style_css = |h: Highlight| {
        theme.styles[h.0].css.as_ref().unwrap().as_bytes()
    };
    let mut renderer = HtmlRenderer::new();
    renderer.render(highlights, code, &get_style_css).unwrap();
    renderer.lines().collect()
}
```

After dropping this snippet into our library's `src/lib.rs` file (replacing the default
starter code), running `cargo check` should (after installing dependencies) verify that
our library can compile without issue, which, in a language as strict as Rust, is a
promising indicator that everything is working.

A bit of exposition regarding this code: the singular function `highlight_adapter` takes
two strings[^6] as arguments: the code snippet to highlight, and a language identifier
called a "TextMate scope" (explained next). If everything goes well, this function then
returns an HTML snippet of the code marked up with inline CSS. Later we will seamlessly
stitch this HTML output into our site as it's built by Jekyll.

[A TextMate scope is a string that Tree-sitter uses to uniquely identify languages.](https://tree-sitter.github.io/tree-sitter/syntax-highlighting#basics)
For example, the scope string for Python is 'source.python', whereas for HTML it is
'text.html.basic'. The `highlight_adapter` function requires that the relevant scope be
provided verbatim to choose the corresponding language. To determine a supported
language's scope string, find the source repository of the language's parser in
[Tree-sitter's list of available parsers](https://tree-sitter.github.io/tree-sitter/#available-parsers),
navigate to the `package.json` file, and locate the value associated with the
Tree-sitter 'scope' key. For example, the scope string for Python can be seen
[here](https://github.com/tree-sitter/tree-sitter-python/blob/de221eccf9a221f5b85474a553474a69b4b5784d/package.json#L27).

The previous aside on TextMate scopes raises another relevant point about how
Tree-sitter, and in turn the `highlight_adapter` function, operates. Namely, Tree-sitter
implements a parser for each language that it supports. Each such parser is independent
of the core library and therefore must be installed separately to parse that language.
So, even though our adapter library currently compiles without issue, it can't parse any
languages until we've installed some parsers for it to use.

The first line of code in `highlight_adapter` tells it to look for parser libraries in a
sibling `parsers` directory, so let's create that directory in the root of our Rust
library. Next, let's install a parser so that we can highlight a language. For example,
we can install the Python parser by downloading
[its source repository](https://github.com/tree-sitter/tree-sitter-python) into this new
`parsers` directory.[^7]

Now we're ready to highlight some Python! Let's test our library by temporarily
packaging it into an executable program. To do this, add a new file called `main.rs` to
the `src` directory with the following content:

```rust
use tree_sitter_ruby_adapter as tsra;

fn main() {
    let raw_code = "print('Hello, World!')";
    let scope = "source.python";
    let highlighted_code = tsra::highlight_adapter(raw_code, scope);
    println!("{}", highlighted_code);
}
```

In this program, we're highlighting the classic 'Hello, World' Python snippet. Also,
recall that the Python scope string is 'source.python', so that's the scope we pass to
`highlight_adapter`. Upon running the program via `cargo run`, you should see the
following printed to the console:

```
<span style='font-weight: bold;color: #005fd7'>print</span>(<span style='color: #008700'>&#39;Hello, World!&#39;</span>)
```
{: .lang-descriptor-hidden}

This is our HTML snippet! If we manually wrap this snippet in a `<pre>` tag, drop it in
a basic HTML file, and render that file in a browser, the highlighted code will look
something like this:

<pre>
<span style='font-weight: bold;color: #005fd7'>print</span>(<span style='color: #008700'>&#39;Hello, World!&#39;</span>)
</pre>

Huzzah! We can now leverage Tree-sitter to highlight code! You may now delete
`src/main.rs`; we won't be needing it anymore.

#### Invoking Tree-sitter from Ruby

Now comes the final step. Our goal is to connect our custom Jekyll plugin to our
Tree-sitter adapter library. The main obstacle in this endeavor is that our plugin is
written in Ruby, while our library is written in Rust. Fortunately, open-source binding
libraries like [Rutie](https://github.com/danielpclark/rutie) exist precisely for
situations like this, allowing us to integrate these two languages with ease.

To use Rutie, we need to install it as a dependency both in our Rust library and in our
Jekyll project. This is accomplished by running, for example, `cargo add rutie@0.8` in
the root of our Rust library, and `bundle add rutie --version '0.0.4'` in the root of
our Jekyll project, respectively.

[Rutie's documentation on using Rust in Ruby](https://github.com/danielpclark/rutie#using-rust-in-ruby)
is quite comprehensive, but, for convenience, I will reiterate much of it here. First,
append the following TOML table to the Rust library's manifest (`Cargo.toml`) file:

```toml
[lib]
crate-type = [ "cdylib" ]
```

Then prepend the following code to the `src/lib.rs` file:

```rust
#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString};

class!(TreeSitterRubyAdapter);

methods!(
    TreeSitterRubyAdapter,
    _rtself,
    fn highlight(raw_code: RString, raw_scope: RString) -> RString {
        let highlighted_code = {
            let code = raw_code.unwrap().to_string();
            let scope = raw_scope.unwrap().to_string();
            highlight_adapter(&code, &scope)
        };
        RString::new_utf8(&highlighted_code)
    }
);

#[no_mangle]
pub extern "C" fn init() {
    Class::new("TreeSitterRubyAdapter", None).define(|class_| {
        class_.def_self("highlight", highlight);
    });
}
```

We now need to make one final adjustment to our `highlight_adapter` function so that it
will behave appropriately when invoked from the root of our Jekyll project. Namely, we
have to update the `parser_directory` path to take into account that our parser
directory is nested under the `_tree_sitter_ruby_adapter` directory inside our Jekyll
project. Hence we need to update the `parser_directory` path as follows:

```rust
let parser_directory = std::env::current_dir()
    .unwrap()
    .join("_tree_sitter_ruby_adapter") // Add this line
    .join("parsers");
```

With these edits in place, run `cargo build --release` from within the Rust library to
compile it. The Rust library is now primed for consumption by our Jekyll plugin.

Now it's time to update our Jekyll plugin accordingly. First, import Rutie by adding the
following line to the top of the Jekyll plugin file:

```ruby
require 'rutie'
```

Next, place the following code after the imports so that we can invoke our Tree-sitter
library via Rutie:

```ruby
Rutie.new(:tree_sitter_ruby_adapter).init(
  'init',
  '_tree_sitter_ruby_adapter/target/'
)
```

Let's also insert a global lookup table into our script, so that we can seamlessly
translate between language names and TextMate scopes:

```ruby
# Add more items to this lookup table as necessary
SCOPES = Hash['python' => 'source.python'].freeze
```

Finally, update the singular method of the existing highlighter to delegate to the
Tree-sitter adapter library for syntax highlighting whenever possible:

```ruby
def self.call(converter, text, language, type, _)
  scope = SCOPES[language]
  tree_sitter = ->(text_) { TreeSitterRubyAdapter.highlight text_, scope }
  # Only use Tree-sitter for highlighting if it supports the given language
  highlighter = scope ? tree_sitter : converter.method(:escape_html)
  output = highlighter.call text
  type == :block ? "<pre><code>#{output}</code></pre>" : output
end
```

And that's all we have to do! Of course, these changes also render the name and
description of the updated highlighter module obsolete, so feel free to update those as
well. I chose to name this new highlighter 'TreeSitter'.

Our Tree-sitter Jekyll highlighter is ready for action! Let's test it out by adding our
Python code snippet to any Markdown source file in our Jekyll project:

````markdown
```python
print('Hello, World!')
```
````

Upon building and serving our Jekyll site, we should see our example snippet highlighted
and rendered automatically. And with that, our Tree-sitter Jekyll highlighter is
complete! &#127881;

### Bonus: improvements and implementation details

#### Improvements

We now have a minimal, working example of our highlighter, but it could still benefit
from many improvements. One notable example is as follows. In the HTML output of the
previous 'Hello, World' Python example, notice that each highlighted token is surrounded
by a `<span>` element, which in turn contains an inline CSS attribute describing how to
style that token. As it turns out, such attributes can be customized. Within our current
implementation, these inline CSS attributes are determined by a Rust closure, reproduced
here:

```rust
let get_style_css = |h: Highlight| {
    theme.styles[h.0].css.as_ref().unwrap().as_bytes()
};
```

This closure is passed to the
[`HtmlRenderer::render` method](https://docs.rs/tree-sitter-highlight/0.20.1/tree_sitter_highlight/struct.HtmlRenderer.html#method.render)
as the `attribute_callback` argument. By modifying this closure, we can affect what
attributes appear in the resulting HTML. For example, while the above closure returns
inline CSS *style* attributes, the productionalized version used by this site returns
*class* attributes, which can be styled independently. This customization allows for
significantly more control over the appearance of highlighted syntax on the site.

My own implementation of this particular enhancement, as well as implementations of
several other improvements, can be found in
[the source repository for this site, on GitHub](https://github.com/andrewtbiehl/andrewtbiehl.com).
Feel free to check them out and adapt them for your own needs!

#### Implementation

The previous walkthrough describes an abridged version of the actual approach I use to
highlight code with Tree-sitter on this site. You might wonder why I chose this
particular approach. As it turns out, the journey I took to arrive at this approach was
long and fraught with dead ends. I leave the following discussion as advice and caution
for anyone interested in attempting something similar.

Fundamentally, the goal of this project was to integrate Tree-sitter highlighting into
the site you see before you. To this end, I had several requirements and constraints
from the start. First, all code snippets on this site are immutable, so I had no need
for (and, frankly, an interest in avoiding) dynamic client-side highlighting. Static,
server-side rendering seemed more appropriate for my use case. As a corollary to this, I
wanted a method that could interface with Ruby, the language used by Jekyll. Third, I
wanted deep control over how the highlighting looked on my site. Fourth and finally, I
of course wanted to take advantage of as much existing Tree-sitter functionality as
possible to achieve maximal highlighting quality.

While working on this project, I considered multiple alternative approaches before
settling on the one described in the walkthrough. Most failed to meet at least one of my
criteria.

- The
  [Tree-sitter WebAssembly bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_web#web-tree-sitter)
  are often recommended as one method for syntax highlighting in the browser.
  Unfortunately, this approach involves client-side highlighting, which I preferred to
  avoid.

- I tried taking advantage of existing Tree-sitter Ruby bindings, as such bindings would
  have worked seamlessly with Jekyll. Unfortunately,
  [Tree-sitter's official Ruby bindings](https://github.com/tree-sitter/ruby-tree-sitter)
  appear to be nonfunctional and unmaintained, and otherwise promising projects like
  [Faveod/ruby-tree-sitter](https://github.com/Faveod/ruby-tree-sitter) do not currently
  provide bindings to the official Tree-sitter highlighter, so using these bindings
  would require me to build my own highlighter as well. I found this latter task
  daunting and ultimately decided not to attempt it.

- I spent a while playing with the Tree-sitter CLI, which can syntax highlight and
  render code in HTML in a single command. Despite my novice Ruby status, I expect that
  it would have been easy to build a Jekyll plugin that could execute this shell command
  from Ruby. Unfortunately, however, the CLI highlights code with inline CSS and, to my
  knowledge, doesn't currently provide the option to use alternative methods.

After spending a lot of time evaluating these options, I decided to try out
Tree-sitter's Rust API directly. After all, the Tree-sitter highlighter and CLI are both
natively implemented in Rust. After tinkering with the Tree-sitter highlight API for a
while, I concluded that this approach would meet my criteria and decided to run with it.
After significant trial and error, I eventually figured out everything described here.
Much of this may have also been possible with the other approaches I considered but, for
my effort, only this approach was able to fulfill my wishlist entirely.

### Assessing Tree-sitter's highlighting quality

Now that we have everything working, the question is, does it work well? In other words,
does Tree-sitter actually live up to its claim of better syntax highlighting? And, if
so, how much better is it?

In my case, the clear standard against which to measure the quality of Tree-sitter is
that of Rouge. All of the work required to integrate Tree-sitter into Jekyll is only
worthwhile if the product of this effort can outperform Jekyll's built-in default.

A head-to-head comparison of the two highlighters is therefore appropriate. In fact,
I'll let you decide for yourself on whether Tree-sitter outcompetes Rouge. What follows
is a collection of code snippets across a variety of languages. Each snippet has been
rendered twice, once with Rouge and a second time with Tree-sitter. All output is
stylized with the same highlighting theme, one which was custom built to complement both
highlighters. You can even use your arrow keys to quickly toggle between each
highlighter and examine their differences in high relief.

Naturally, our first snippet is none other than the example from the walkthrough:

{% include syntax_highlighter_comparison/python_example_1.md %}

Not much to see here, the only minor difference being how each highlighter views the
'print' token (Rouge recognizes it as a keyword, which is reasonable, while Tree-sitter
correctly identifies it as a function).

Now let's take a look at the actual snippets showcased by Max Brunsfeld in his talk.[^8]
Note that the highlighting theme used here is distinct from any seen in the talk.

{% include syntax_highlighter_comparison/c_example.md %}

{% include syntax_highlighter_comparison/cpp_example.md %}

{% include syntax_highlighter_comparison/go_example.md %}

{% include syntax_highlighter_comparison/rust_example.md %}

{% include syntax_highlighter_comparison/typescript_example_2.md %}

In my opinion, these snippets portray Tree-sitter fantastically in comparison to Rouge.
Whereas Rouge struggles to recognize almost anything more than a built-in keyword or
type, Tree-sitter deftly distinguishes between fields, functions, user-defined types,
and more. I will, however, point out one small but significant discrepancy in how the
Rust example was highlighted: in the talk, Tree-sitter recognizes that `None`, a variant
of the `Option` enum, is a value and not a type, whereas it fails to do so here. On the
other hand, Rouge gets this wrong too.

Next are some longer snippets for the languages that I am currently most familiar
with:[^9]

{% include syntax_highlighter_comparison/python_example_2.md %}

{% include syntax_highlighter_comparison/java_example.md %}

{% include syntax_highlighter_comparison/haskell_example.md %}

In these examples, Tree-sitter arguably fares less favorably against its competitor than
before. With Python, Tree-sitter fails to recognize built-in types and keywords like
`int` and `self`, whereas Rouge does not. In the Java example, I was disappointed to
find that neither Rouge nor Tree-sitter distinguishes field variables, even despite the
explicit use of `this` throughout the class. Finally, both Tree-sitter and Rouge
struggle to make much sense of most variables in Haskell. Tree-sitter's saving grace
across all three languages is its unique ability to recognize function calls, although,
in my opinion, it is inconsistent enough (and sometimes even incorrect) in Haskell for
this to actually be distracting.

Finally, there are a couple of interesting highlighting features that Tree-sitter
currently claims to support. First, Tree-sitter is purportedly able to contingently
highlight a token based on its scope. The following Ruby snippet [is provided in
Tree-sitter's documentation](https://tree-sitter.github.io/tree-sitter/syntax-highlighting#local-variables)
to illustrate this:

{% include syntax_highlighter_comparison/ruby_example.md %}

As we can see, Tree-sitter is impressively able to distinguish between multiple
different senses of the variables `list` and `item`: within the scope of `process_list`,
both are correctly identified as formal parameters, whereas outside of this scope they
are recognized as ordinary local variables. Also noteworthy is Tree-sitter's recognition
that `current_context` must reference a function – despite lacking associated
parentheses and arguments – since it is not declared earlier as a parameter. Presumably,
this level of understanding of the code goes far beyond anything Rouge is capable of.

Another Tree-sitter highlighting feature is language injection; that is, it can
highlight one language embedded in another. The following HTML snippet illustrates this.

{% include syntax_highlighter_comparison/html_example.md %}

As you can see, Rouge also supports language injection, at least for common use cases
such as this. I suspect, however, that Tree-sitter is again significantly more capable
in this respect, supporting arbitrarily deep levels of nesting as well as nesting of
unconventional language pairings. Unfortunately, this is merely an educated guess; I
have not yet seen examples that support this.

### Final thoughts

As seen in the previous examples, Tree-sitter's performance is currently a bit fickle,
but overall quite encouraging. On one hand, Tree-sitter almost always demonstrates a
deeper understanding of the code it's parsing than its counterpart, Rouge, and sometimes
this disparity is vast. On the other hand, Tree-sitter does have its flaws and
shortcomings, at least by my analysis. For example, it struggles to recognize fields in
Java, functions in Haskell, and types in Python. And, while some of its capabilities,
such as contingent highlighting of formal parameters, are incredibly impressive, I would
be even more excited to learn that more features of this caliber are yet to be
implemented.

Quibbles aside, I am quite pleased with Tree-sitter's overall highlighting quality and
plan to retain it as my site's primary code highlighter. With luck, I may have even
helped you get it working on your site too, in which case I hope that you feel the same.
Regardless, I'm comfortable *going out on a limb* with the following prediction: whether
it's my site, your site, or someone else's, this won't be the last time you come across
this tool. Whenever something with this much potential comes along, it stays.

*Thanks for reading! If you have questions or suggestions, feel free to
[reach out](/about/#contact-me)! Happy highlighting!*

### Additional resources

- [Max Brunsfeld's fantastic talk on Tree-sitter at Strange Loop 2018](https://youtu.be/Jes3bD6P0To),
  the inspiration behind the project of this post.
- [Tree-sitter's homepage](https://tree-sitter.github.io/tree-sitter).
- [This post](https://adrian.schoenig.me/blog/2022/05/27/tree-sitter-highlighting-in-jekyll)
  by Adrian Schönig, explaining how to use Tree-sitter's WebAssembly bindings to achieve
  similar results.
- [This post](https://dcreager.net/2021/06/getting-started-with-tree-sitter/) by Douglas
  Creager on how to use the Tree-sitter CLI.
- [The source code for this site](https://github.com/andrewtbiehl/andrewtbiehl.com),
  which, if you're willing to wade through it, contains a robust, working example of the
  approach outlined in this post, including various enhancements not mentioned here.
- The homepages for [Jekyll](https://jekyllrb.com), [Kramdown](https://kramdown.gettalong.org),
  and [Rouge](http://rouge.jneen.net).
- Documentation for [Rutie](https://github.com/danielpclark/rutie).

[^1]: Brunsfeld, Max. "Tree-sitter - a new parsing system for programming tools".
    ([8:15](https://youtu.be/Jes3bD6P0To?t=495)). Strange Loop, 2022.

[^2]: Based on my testing, I expect that this minimal highlighter implementation will
    'just work' for most normal Jekyll sites. However, for sites with a lot of
    pre-existing CSS or Markdown rendering customizations, slight adjustments to this
    template may prove necessary to ensure everything renders nicely.

[^3]: Like Rouge, Kramdown is an interchangeable component of Jekyll. Hence this step is
    predicated on the assumption that Kramdown indeed remains the Markdown renderer of
    choice for the site being modified.

[^4]: FYI, the initial underscore in the directory name is functional: it instructs
    Jekyll to ignore this directory when looking for source files to build.

[^5]: As previously mentioned, Tree-sitter is still in initial development.
    Consequently, its API is likely to change over time. For this reason, I recommend
    (at least initially) pinning the versions of these libraries collectively to v0.20
    to ensure success when following the walkthrough. This is the version I used to test
    my code and hence the one in which I have the most confidence of being compatible
    with the walkthrough.

[^6]: Technically, these arguments are string *references*, but this detail is not
    relevant for our purposes.

[^7]: In the likely scenario that your Jekyll project is version-controlled with Git, I
    specifically recommend using Git's submodules functionality to install parsers into
    your project. Information about Git submodules can be found
    [here](https://git-scm.com/docs/gitsubmodules).

[^8]: Specifically, these snippets are reproduced verbatim from the following source:

    Brunsfeld, Max. "Tree-sitter - a new parsing system for programming tools".
    ([6:12](https://youtu.be/Jes3bD6P0To?t=372)). Strange Loop, 2022.

    Note also that all errata from the original examples are left unedited in these
    reproductions.

[^9]: For anyone curious, the Python program draws a
    [Koch snowflake](https://en.wikipedia.org/wiki/Koch_snowflake), the Java class is a
    basic implementation of the
    [disjoint-set data structure](https://en.wikipedia.org/wiki/Disjoint-set_data_structure),
    and the Haskell program implements the
    [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm). I chose these
    particular examples mainly just for fun. :)
