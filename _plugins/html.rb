# frozen_string_literal: true

require 'kramdown'

# Some minor customizations of Kramdown's HTML converter.
module CustomHtmlConverter
  LANGUAGE_DATA_ATTRIBUTE_NAME = 'data-language'

  # Formats a block of code.
  #
  # Returns a syntax-highlighted version of the code based on its associated language,
  # wrapped in a <code> element inside a <pre> element. A data attribute is also added
  # to the <code> element containing the language of the code block.
  #
  # This customized version was built for use with the bare-bones
  # Rouge::Formatters::HTML syntax highlighter and may produce unexpected results if
  # used incautiously with other highlighters.
  def convert_codeblock(element, indent)
    raw_content, attributes, language = extract_code_information element
    # Language of a code block is the default language if not specified explicitly
    language ||= default_language
    language_info = { LANGUAGE_DATA_ATTRIBUTE_NAME => language }
    raw_content
      .then { highlight_code _1, language, :block }
      # Formatting inside a pre element must be careful not to introduce newlines
      .then { format_as_span_html 'code', language_info, _1 }
      .then { format_as_block_html 'pre', attributes, _1, indent }
  end

  # Formats an inline span of code.
  #
  # Returns the code wrapped inside an inline <code> element. If (and only if) a
  # language for the code was explicitly defined (i.e. via a 'language-*' class
  # attribute), then the code is also syntax highlighted and a data attribute is added
  # to the <code> element containing the associated language.
  #
  # This customized version was built for use with the bare-bones
  # Rouge::Formatters::HTML syntax highlighter and may produce unexpected results
  # if used incautiously with other highlighters.
  def convert_codespan(element, _indent) = _convert_codespan(element)

  private

  # See public version of this method for more documentation.
  def _convert_codespan(element)
    raw_content, attributes, language = extract_code_information element
    # Add the language data attribute iff the language is specified explicitly
    attributes[LANGUAGE_DATA_ATTRIBUTE_NAME] = language if language
    raw_content
      .then { highlight_code _1, language, :span }
      .then { format_as_span_html 'code', attributes, _1 }
  end

  # Utility method for extracting the content, attributes, and language of a block (or
  # span) of code, for use in formatting the code as HTML.
  def extract_code_information(element)
    raw_content = element.value
    attributes = element.attr.dup
    # Extract the language and remove the language class attribute
    language = extract_code_language! attributes
    [raw_content, attributes, language]
  end

  # Alias for accessing the global default language.
  def default_language = options[:syntax_highlighter_opts][:default_lang]
end

Kramdown::Converter::Html.prepend CustomHtmlConverter
