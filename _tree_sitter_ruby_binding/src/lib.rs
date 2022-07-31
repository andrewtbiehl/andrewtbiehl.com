#[macro_use]
extern crate rutie;

mod tree_sitter_adapter;

use rutie::{Class, Object, RString};

class!(TreeSitterAdapterRubyBinding);

methods!(
    TreeSitterAdapterRubyBinding,
    _rtself,
    fn highlight(raw_code: RString, _raw_language_str: RString) -> RString {
        let code = raw_code.unwrap().to_string();
        let formatted_code = tree_sitter_adapter::no_highlight(&code);
        RString::new_utf8(&formatted_code)
    }
);

#[no_mangle]
pub extern "C" fn init() {
    Class::new("TreeSitterAdapterRubyBinding", None).define(|class_| {
        class_.def_self("highlight", highlight);
    });
}
