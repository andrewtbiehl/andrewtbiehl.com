// An adapter to the Tree-sitter highlight library.
//
// Provides a simple interface to the Tree-sitter highlight library that also creates
// CSS classes for highlight blocks. This latter functionality differs from the default
// behavior of the Tree-sitter highlight library, which adds inline CSS to each
// highlight block.

use std::str::FromStr;
use tree_sitter_highlight::{
    Highlight as TSHighlight, HighlightConfiguration, Highlighter, HtmlRenderer,
};

struct Highlight<'a> {
    pub name: &'a str,
    pub class: &'a str,
}

const HIGHLIGHTS: [Highlight; 17] = [
    Highlight { name: "comment",             class: "cm"   },
    Highlight { name: "constant",            class: "ct"   },
    Highlight { name: "constant.builtin",    class: "ct-b" },
    Highlight { name: "constructor",         class: "cr"   },
    Highlight { name: "embedded",            class: "em"   },
    Highlight { name: "escape",              class: "es"   },
    Highlight { name: "function",            class: "f"    },
    Highlight { name: "function.builtin",    class: "f-b"  },
    Highlight { name: "function.method",     class: "f-m"  },
    Highlight { name: "keyword",             class: "k"    },
    Highlight { name: "number",              class: "n"    },
    Highlight { name: "operator",            class: "o"    },
    Highlight { name: "property",            class: "pr"   },
    Highlight { name: "punctuation.special", class: "pu-s" },
    Highlight { name: "string",              class: "s"    },
    Highlight { name: "type",                class: "t"    },
    Highlight { name: "variable",            class: "v"    },
];

lazy_static! {
    static ref CLASS_ATTRIBUTE_STRINGS: [String; 17] =
        HIGHLIGHTS.map(|highlight| format!("class=\"{}\"", highlight.class));
    static ref PYTHON_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(&HIGHLIGHTS.map(|highlight| highlight.name));
        config
    };
}

pub enum Language {
    Python,
}

impl Language {
    fn config(&self) -> &HighlightConfiguration {
        match *self {
            Language::Python => &PYTHON_CONFIG,
        }
    }
}

pub struct UnknownLanguageError;

impl FromStr for Language {
    type Err = UnknownLanguageError;

    fn from_str(language_name: &str) -> Result<Self, Self::Err> {
        match language_name {
            "python" => Ok(Language::Python),
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
        let highlights = highlighter.highlight(config, code, None, |_| None).unwrap();
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
