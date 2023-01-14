// An adapter to the Tree-sitter highlight library.
//
// Provides a simple interface to the Tree-sitter highlight library that also creates
// CSS classes for highlight blocks. This latter functionality differs from the default
// behavior of the Tree-sitter highlight library, which adds inline CSS to each
// highlight block.

use std::env;
use std::str::FromStr;
use tree_sitter_highlight::{
    Highlight as TSHighlight, HighlightConfiguration, Highlighter, HtmlRenderer,
};
use tree_sitter_loader::{Config, Loader};

struct Highlight<'a> {
    pub name: &'a str,
    pub class: &'a str,
}

const HIGHLIGHTS: [Highlight; 48] = [
    Highlight { name: "attribute",               class: "ts-attribute"               },
    Highlight { name: "_bool",                   class: "ts-_bool"                   },
    Highlight { name: "boolean",                 class: "ts-boolean"                 },
    Highlight { name: "character",               class: "ts-character"               },
    Highlight { name: "comment",                 class: "ts-comment"                 },
    Highlight { name: "conditional",             class: "ts-conditional"             },
    Highlight { name: "constant",                class: "ts-constant"                },
    Highlight { name: "constant.builtin",        class: "ts-constant-builtin"        },
    Highlight { name: "constant.macro",          class: "ts-constant-macro"          },
    Highlight { name: "constructor",             class: "ts-constructor"             },
    Highlight { name: "delimiter",               class: "ts-delimiter"               },
    Highlight { name: "embedded",                class: "ts-embedded"                },
    Highlight { name: "escape",                  class: "ts-escape"                  },
    Highlight { name: "float",                   class: "ts-float"                   },
    Highlight { name: "function",                class: "ts-function"                },
    Highlight { name: "function.builtin",        class: "ts-function-builtin"        },
    Highlight { name: "function.macro",          class: "ts-function-macro"          },
    Highlight { name: "function.method",         class: "ts-function-method"         },
    Highlight { name: "function.method.builtin", class: "ts-function-method-builtin" },
    Highlight { name: "function.special",        class: "ts-function-special"        },
    Highlight { name: "include",                 class: "ts-include"                 },
    Highlight { name: "injection.content",       class: "ts-injection-content"       },
    Highlight { name: "injection.language",      class: "ts-injection-language"      },
    Highlight { name: "keyword",                 class: "ts-keyword"                 },
    Highlight { name: "label",                   class: "ts-label"                   },
    Highlight { name: "local.definition",        class: "ts-local-definition"        },
    Highlight { name: "local.reference",         class: "ts-local-reference"         },
    Highlight { name: "local.scope",             class: "ts-local-scope"             },
    Highlight { name: "namespace",               class: "ts-namespace"               },
    Highlight { name: "number",                  class: "ts-number"                  },
    Highlight { name: "operator",                class: "ts-operator"                },
    Highlight { name: "property",                class: "ts-property"                },
    Highlight { name: "punctuation.bracket",     class: "ts-punctuation-bracket"     },
    Highlight { name: "punctuation.delimiter",   class: "ts-punctuation-delimiter"   },
    Highlight { name: "punctuation.special",     class: "ts-punctuation-special"     },
    Highlight { name: "repeat",                  class: "ts-repeat"                  },
    Highlight { name: "string",                  class: "ts-string"                  },
    Highlight { name: "string.special",          class: "ts-string-special"          },
    Highlight { name: "string.special.regex",    class: "ts-string-special-regex"    },
    Highlight { name: "string.special.symbol",   class: "ts-string-special-symbol"   },
    Highlight { name: "symbol",                  class: "ts-symbol"                  },
    Highlight { name: "tag",                     class: "ts-tag"                     },
    Highlight { name: "tag.error",               class: "ts-tag-error"               },
    Highlight { name: "type",                    class: "ts-type"                    },
    Highlight { name: "type.builtin",            class: "ts-type-builtin"            },
    Highlight { name: "variable",                class: "ts-variable"                },
    Highlight { name: "variable.builtin",        class: "ts-variable-builtin"        },
    Highlight { name: "variable.parameter",      class: "ts-variable-parameter"      },
];

lazy_static! {
    static ref CLASS_ATTRIBUTE_STRINGS: [String; 48] =
        HIGHLIGHTS.map(|Highlight { class, .. }| format!("class=\"{}\"", class));
    static ref PARSER_LOADER: Loader = {
        let mut loader = Loader::new().unwrap();
        let parser_directories = {
            let parser_directory = env::current_dir()
                .unwrap()
                .join("_tree_sitter_ruby_adapter")
                .join("parsers");
            vec![parser_directory]
        };
        loader
            .find_all_languages(&Config { parser_directories })
            .unwrap();
        let highlight_names = HIGHLIGHTS
            .map(|Highlight { name, .. }| name)
            .map(String::from)
            .to_vec();
        loader.configure_highlights(&highlight_names);
        loader
    };
}

pub enum Language {
    C,
    CPlusPlus,
    Go,
    Haskell,
    Html,
    Java,
    JavaScript,
    Python,
    Ruby,
    Rust,
    TypeScript,
}

impl Language {
    fn config(&self) -> &HighlightConfiguration {
        PARSER_LOADER
            .language_configuration_for_scope(self.scope())
            .unwrap()
            .and_then(|(language, config)| config.highlight_config(language).ok())
            .unwrap()
            .unwrap()
    }

    fn scope<'a>(&self) -> &'a str {
        match self {
            Language::C => "source.c",
            Language::CPlusPlus => "source.cpp",
            Language::Go => "source.go",
            Language::Haskell => "source.haskell",
            Language::Html => "text.html.basic",
            Language::Java => "source.java",
            Language::JavaScript => "source.js",
            Language::Python => "source.python",
            Language::Ruby => "source.ruby",
            Language::Rust => "source.rust",
            Language::TypeScript => "source.ts",
        }
    }
}

pub struct UnknownLanguageError;

impl FromStr for Language {
    type Err = UnknownLanguageError;

    fn from_str(language_name: &str) -> Result<Self, Self::Err> {
        match language_name {
            "c" => Ok(Language::C),
            "cpp" => Ok(Language::CPlusPlus),
            "go" => Ok(Language::Go),
            "haskell" => Ok(Language::Haskell),
            "html" => Ok(Language::Html),
            "java" => Ok(Language::Java),
            "javascript" => Ok(Language::JavaScript),
            "python" => Ok(Language::Python),
            "ruby" => Ok(Language::Ruby),
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            _ => Err(UnknownLanguageError),
        }
    }
}

trait TSHighlightExt {
    fn to_class_attribute_str<'a>(self) -> &'a [u8];
}

impl TSHighlightExt for TSHighlight {
    fn to_class_attribute_str<'a>(self) -> &'a [u8] {
        CLASS_ATTRIBUTE_STRINGS[self.0].as_bytes()
    }
}

// Escapes HTML text content.
//
// Not intended for use on other HTML content, such as attribute content.
fn escape_text_html(text: &str) -> String {
    let mut escaped_text = String::new();
    for c in text.chars() {
        match c {
            '&' => escaped_text.push_str("&amp;"),
            '<' => escaped_text.push_str("&lt;"),
            '>' => escaped_text.push_str("&gt;"),
            _ => escaped_text.push(c),
        }
    }
    escaped_text
}

// See the public version of this method, `highlight`, for more documentation.
fn highlight_adapter(code: &[u8], config: &HighlightConfiguration) -> String {
    {
        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(config, code, None, |s| {
                PARSER_LOADER.highlight_config_for_injection_string(s)
            })
            .unwrap();
        let mut renderer = HtmlRenderer::new();
        renderer
            .render(highlights, code, &TSHighlight::to_class_attribute_str)
            .unwrap();
        renderer
    }
    .lines()
    .collect()
}

// Convenience function for not highlighting code.
//
// HTML escapes the given text without highlighting it. Useful for when the code's
// language is not known.
pub fn no_highlight(code: &str) -> String {
    escape_text_html(code)
}

// Adapter function for interoperating with Tree-sitter's highlight library.
//
// In contrast with Tree-sitter's library, which returns inline CSS, this function
// returns CSS classes for each highlight.
pub fn highlight(code: &str, language: &Language) -> String {
    highlight_adapter(code.as_bytes(), language.config())
}
