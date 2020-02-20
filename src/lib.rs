extern crate wasm_bindgen;
//#[macro_use] extern crate lazy_static;

mod input;

use wasm_bindgen::prelude::*;
use web_sys::{ Element, Document, Selection, console };

use xi_rope::{ Rope };

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(raw_module = "pipetext/pipetext.mjs")]
extern "C" {
    pub type Pipetext;

    #[wasm_bindgen(method)]
    pub fn refresh(this: &Pipetext, s : String, cursor_offset : u32);

    #[wasm_bindgen(method)]
    pub fn edit_tree(this: &Pipetext, start_index : u32, old_end_index : u32, new_end_index : u32, 
        start_row : u32, start_column : u32, old_end_row : u32, 
        old_end_column : u32, new_end_row : u32, new_end_column : u32);
}


#[wasm_bindgen]
pub struct Core
{
    text : Rope,
    //engine : Engine
    div : Element,
    selection : Option<Selection>,
    document : Document,
    pipetext_web_instance : Pipetext
}

#[wasm_bindgen]
impl Core
{

    pub fn from_div(div : Element, instance : Pipetext) -> Core
    {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let result = Core
        {
            text : div.text_content().unwrap().into(),
            div : div.clone(),
            selection : document.get_selection().unwrap(),
            document : document,
            pipetext_web_instance : instance
        };

        result
    }

    //pub fn edit(&mut self, start : usize, end : usize, addition : String){ self.text.edit(start..end, addition); }

    #[wasm_bindgen(getter)]
    pub fn get_str(&self) -> String { String::from(&self.text) }
}