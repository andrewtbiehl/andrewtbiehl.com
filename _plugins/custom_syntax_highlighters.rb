# frozen_string_literal: true

require 'kramdown/converter/syntax_highlighter/rouge'
require 'rutie'

Rutie.new(:tree_sitter_ruby_binding).init 'init', '_tree_sitter_ruby_binding/target/'

# Some custom Kramdown syntax highlighters.
module Kramdown
  module Converter
    module SyntaxHighlighter
      # Highlighter that uses the Tree-sitter syntax highlighting library.
      module TreeSitter
        def self.call(_, raw_content, language, _, _)
          TreeSitterAdapterRubyBinding.highlight raw_content, language
        end
      end

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
          highlighter.call converter, text, language, type, options
        end
      end
    end
  end
end

Kramdown::Converter.add_syntax_highlighter(
  :custom_highlighter_delegator,
  Kramdown::Converter::SyntaxHighlighter::CustomHighlighterDelegator
)
