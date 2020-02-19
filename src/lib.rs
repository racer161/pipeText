extern crate wasm_bindgen;
//#[macro_use] extern crate lazy_static;

use futures::executor::block_on;

mod tree_sitter_wasm_bindings;
mod tree_builder;
mod input;


use wasm_bindgen::prelude::*;
use web_sys::{ Element, Document, Selection, console };

use xi_rope::{ Rope };

use tree_sitter_wasm_bindings::{ TreeSitter, Tree, get_lang };
use tree_builder::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Pipetext
{
    text : Rope,
    //engine : Engine
    div : Element,
    selection : Option<Selection>,
    document : Document,
    TreeSitter : TreeSitter,
    tree : Option<Tree>
}

#[wasm_bindgen]
impl Pipetext
{

    pub async fn from_div(div : Element) -> Pipetext
    {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let tree_sitter_parser : TreeSitter = TreeSitter::new();

        //Hard coded for now but eventually many languages will be supported
        let target_lang = "javascript";

        let lang_path = format!("tree-sitter/tree-sitter-{}.wasm", target_lang);

        
        let lang_promise = get_lang(lang_path);
        let lang = wasm_bindgen_futures::JsFuture::from(lang_promise).await.unwrap();

        tree_sitter_parser.setLanguage(&lang);

        let first_text : String = div.text_content().unwrap();

        //console::log_1(&JsValue::from(first_text.clone()));

        let first_tree = tree_sitter_parser.parse(&first_text, &JsValue::NULL);

        let result = Pipetext
        {
            text : div.text_content().unwrap().into(),
            div : div.clone(),
            selection : document.get_selection().unwrap(),
            document : document,
            TreeSitter : tree_sitter_parser,
            tree : Some(first_tree)
        };

        result
    }

    //TODO: streamline this by passing option as a JsValueS
    fn incremental_parse(&mut self) 
    { 
        self.tree = match self.tree.take()
        {
            None => Some(self.TreeSitter.parse(&String::from(&self.text), &JsValue::NULL)),
            Some(t) => Some(self.TreeSitter.parse(&String::from(&self.text), &t))
        }
    }

    pub fn update_html(&mut self)
    {
        let (code_div, cursor_div, offset) =  self.render_html(0);

        self.div.remove_child(&self.div.first_child().unwrap());

        self.div.append_child(&code_div);
    }

    //pub fn edit(&mut self, start : usize, end : usize, addition : String){ self.text.edit(start..end, addition); }

    #[wasm_bindgen(getter)]
    pub fn get_str(&self) -> String { String::from(&self.text) }
}