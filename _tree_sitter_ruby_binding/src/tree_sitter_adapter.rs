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

const HIGHLIGHTS: [Highlight; 50] = [
    Highlight { name: "attribute",               class: "a"     },
    Highlight { name: "_bool",                   class: "b1"    },
    Highlight { name: "boolean",                 class: "b2"    },
    Highlight { name: "character",               class: "ch"    },
    Highlight { name: "comment",                 class: "cm"    },
    Highlight { name: "conditional",             class: "cd"    },
    Highlight { name: "constant",                class: "ct"    },
    Highlight { name: "constant.builtin",        class: "ct-b"  },
    Highlight { name: "constant.macro",          class: "ct-m"  },
    Highlight { name: "constructor",             class: "cr"    },
    Highlight { name: "delimiter",               class: "d"     },
    Highlight { name: "embedded",                class: "em"    },
    Highlight { name: "escape",                  class: "es"    },
    Highlight { name: "float",                   class: "fl"    },
    Highlight { name: "function",                class: "f"     },
    Highlight { name: "function.builtin",        class: "f-b"   },
    Highlight { name: "function.macro",          class: "f-ma"  },
    Highlight { name: "function.method",         class: "f-m"   },
    Highlight { name: "function.method.builtin", class: "f-m-b" },
    Highlight { name: "function.special",        class: "f-s"   },
    Highlight { name: "glimmer",                 class: "g"     },
    Highlight { name: "include",                 class: "i"     },
    Highlight { name: "injection.content",       class: "ij-c"  },
    Highlight { name: "injection.language",      class: "ij-l"  },
    Highlight { name: "keyword",                 class: "k"     },
    Highlight { name: "label",                   class: "la"    },
    Highlight { name: "local.definition",        class: "l-d"   },
    Highlight { name: "local.reference",         class: "l-r"   },
    Highlight { name: "local.scope",             class: "l-s"   },
    Highlight { name: "_name",                   class: "nm"    },
    Highlight { name: "namespace",               class: "na"    },
    Highlight { name: "number",                  class: "n"     },
    Highlight { name: "operator",                class: "o"     },
    Highlight { name: "property",                class: "pr"    },
    Highlight { name: "punctuation.bracket",     class: "pu-b"  },
    Highlight { name: "punctuation.delimiter",   class: "pu-d"  },
    Highlight { name: "punctuation.special",     class: "pu-s"  },
    Highlight { name: "repeat",                  class: "r"     },
    Highlight { name: "string",                  class: "s"     },
    Highlight { name: "string.special",          class: "s-s"   },
    Highlight { name: "string.special.regex",    class: "s-s-r" },
    Highlight { name: "string.special.symbol",   class: "s-s-s" },
    Highlight { name: "symbol",                  class: "sy"    },
    Highlight { name: "tag",                     class: "tg"    },
    Highlight { name: "tag.error",               class: "tg-e"  },
    Highlight { name: "type",                    class: "t"     },
    Highlight { name: "type.builtin",            class: "t-b"   },
    Highlight { name: "variable",                class: "v"     },
    Highlight { name: "variable.builtin",        class: "v-b"   },
    Highlight { name: "variable.parameter",      class: "v-p"   },
];

lazy_static! {
    static ref CLASS_ATTRIBUTE_STRINGS: [String; 50] =
        HIGHLIGHTS.map(|Highlight { class, .. }| format!("class=\"{}\"", class));
    static ref PARSER_LOADER: Loader = {
        let mut loader = Loader::new().unwrap();
        let parser_directories = {
            let parser_directory = env::current_dir()
                .unwrap()
                .join("_tree_sitter_ruby_binding")
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
