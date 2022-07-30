# frozen_string_literal: true

require 'kramdown/converter/syntax_highlighter/rouge'

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
      # Currently, one highlighter is supported: Rouge. To select a highlighter, call
      # the highlighter with an options map that includes the key-value pair
      # 'highlighter'/<choice-of-highlighter> for the given code block. The value of
      # <choice-of-highlighter> must be 'rouge'. If it is anything else, the delegator
      # will default to using a non-highlighter.
      module CustomHighlighterDelegator
        HIGHLIGHTERS = { 'rouge' => Rouge }.freeze

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
