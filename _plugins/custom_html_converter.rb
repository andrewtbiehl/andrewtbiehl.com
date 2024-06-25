# frozen_string_literal: true

require 'kramdown'

# Some minor customizations of Kramdown's HTML converter.
module CustomHtmlConverter
  TREE_SITTER_LANGUAGES = Set['java', 'python', 'ruby', 'rust'].freeze
  LANGUAGE_DATA_ATTRIBUTE_NAME = 'data-language'
  HIGHLIGHTER_DATA_ATTRIBUTE_NAME = 'data-highlighter'

  # Formats a paragraph.
  #
  # Returns the paragraph formatted normally unless the paragraph consists of a
  # standalone image, in which case there is no need to wrap it inside a paragraph.
  def convert_p(element, indent)
    return convert(element.children.first, indent) if contains_standalone_image? element

    super
  end

  # Formats a block of code.
  #
  # Returns a syntax-highlighted version of the code based on its associated language,
  # wrapped in a <code> element inside a <pre> element. A data attribute is also added
  # to the <code> element containing the language of the code block.
  #
  # This customized version was originally built for use with the bare-bones
  # Rouge::Formatters::HTML syntax highlighter and may produce unexpected results if
  # used incautiously with other highlighters.
  def convert_codeblock(element, indent)
    raw_content, attributes, language = extract_code_information element
    _convert_codeblock raw_content, attributes, language, indent
  end

  # Formats an inline span of code.
  #
  # Returns the code wrapped inside an inline <code> element. If (and only if) a
  # language for the code was explicitly defined (i.e. via a 'language-*' class
  # attribute), then the code is also syntax highlighted and a data attribute is added
  # to the <code> element containing the associated language.
  #
  # This customized version was originally built for use with the bare-bones
  # Rouge::Formatters::HTML syntax highlighter and may produce unexpected results
  # if used incautiously with other highlighters.
  def convert_codespan(element, _indent) = _convert_codespan(element)

  private

  # Helper method that checks whether a given element contains a standalone image.
  def contains_standalone_image?(element)
    element.children.size == 1 && image?(element.children.first)
  end

  # Helper method that checks whether a given element is an image.
  def image?(element)
    element.type == :img || (element.type == :html_element && element.value == 'img')
  end

  # See public version of this method for more documentation.
  def _convert_codeblock(raw_content, attributes, language, indent)
    # Language of a code block is the default language if not specified explicitly
    language ||= default_language
    highlighter = determine_highlighter! attributes, language
    code_info = {
      LANGUAGE_DATA_ATTRIBUTE_NAME => language,
      HIGHLIGHTER_DATA_ATTRIBUTE_NAME => highlighter
    }
    raw_content
      .then { highlight_code _1, language, :block, { highlighter: } }
      # Formatting inside a pre element must be careful not to introduce newlines
      .then { format_as_span_html 'code', code_info, _1 }
      .then { format_as_block_html 'pre', attributes, _1, indent }
  end

  # See public version of this method for more documentation.
  def _convert_codespan(element)
    raw_content, attributes, language = extract_code_information element
    highlighter = determine_highlighter! attributes, language
    # Add the data attributes iff the language is specified explicitly
    if language
      code_info = {
        LANGUAGE_DATA_ATTRIBUTE_NAME => language,
        HIGHLIGHTER_DATA_ATTRIBUTE_NAME => highlighter
      }
      attributes.merge(code_info)
    end
    raw_content
      .then { highlight_code _1, language, :span, { highlighter: } }
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

  # Utility method for determining which syntax highlighter to use for a given block (or
  # span) of code. Two highlighters are supported: Rouge and Tree-sitter. Consult the
  # method body for exactly how the highlighter is determined.
  def determine_highlighter!(attributes, language)
    default = TREE_SITTER_LANGUAGES.include?(language) ? 'tree-sitter' : 'rouge'
    # Override the default when a highlighter is explicity set
    attributes.delete('highlighter') { |_| default }
  end

  # Alias for accessing the global default language.
  def default_language = options[:syntax_highlighter_opts][:default_lang]
end

Kramdown::Converter::Html.prepend CustomHtmlConverter
