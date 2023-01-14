# frozen_string_literal: true

require 'kramdown/converter/syntax_highlighter/rouge'
require 'kramdown/syntax_tree_sitter'

# Some custom Kramdown syntax highlighters.
module Kramdown
  module Converter
    module SyntaxHighlighter
      # 'Highlighter' that does not actually highlight code.
      #
      # Escapes the code block so that it can be safely inserted into HTML text.
      module NoHighlight
        def self.call(converter, text, _, _, _) = converter.escape_html text
      end

      # Highlighter used for delegating to other highlighters as requested.
      #
      # Currently, two highlighters are supported: Rouge and Tree-sitter. To select a
      # highlighter, call the highlighter with an options map that includes the
      # key-value pair 'highlighter'/<choice-of-highlighter> for the given code block.
      # The value of <choice-of-highlighter> must be either 'rouge' or 'tree-sitter'. If
      # it is anything else, the delegator will default to using a non-highlighter.
      module CustomHighlighterDelegator
        HIGHLIGHTERS = { 'rouge' => Rouge, 'tree-sitter' => TreeSitter }.freeze

        def self.call(converter, text, language, type, options)
          highlighter = HIGHLIGHTERS.fetch(options[:highlighter], NoHighlight)
          rendered_text = highlighter.call converter, text, language, type, options
          # Remove the surrounding tags added by the Kramdown Tree-sitter plugin
          if options[:highlighter] == 'tree-sitter'
            rendered_text['<pre><code>'.length..-'</code></pre>'.length - 1]
          else
            rendered_text
          end
        end
      end
    end
  end
end

Kramdown::Converter.add_syntax_highlighter(
  :custom_highlighter_delegator,
  Kramdown::Converter::SyntaxHighlighter::CustomHighlighterDelegator
)
