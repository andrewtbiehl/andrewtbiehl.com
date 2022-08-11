#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rutie;

mod tree_sitter_adapter;

use crate::tree_sitter_adapter::Language;
use rutie::{Class, Object, RString};
use std::str::FromStr;

class!(TreeSitterAdapterRubyBinding);

methods!(
    TreeSitterAdapterRubyBinding,
    _rtself,
    fn highlight(raw_code: RString, raw_language_str: RString) -> RString {
        let code = raw_code.unwrap().to_string();
        let language_str = raw_language_str.unwrap().to_string();
        let possible_language = Language::from_str(&language_str).ok();
        let formatted_code = match possible_language {
            Some(language) => tree_sitter_adapter::highlight(&code, &language),
            None => tree_sitter_adapter::no_highlight(&code),
        };
        RString::new_utf8(&formatted_code)
    }
);

#[no_mangle]
pub extern "C" fn init() {
    Class::new("TreeSitterAdapterRubyBinding", None).define(|class_| {
        class_.def_self("highlight", highlight);
    });
}
