extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use lipschitz::incremental_diff::IncrementalDiff;
use otcore::list::*;
use lipschitz::lang::Parsable;

mod recursive_tree_builder;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Pipetext
{
    diff : IncrementalDiff,
    doc_state : List<char>,
    html_string : String
}

#[wasm_bindgen]
impl Pipetext
{
    
    pub fn from_string(source_code : &str) -> Pipetext
    {
        Pipetext
        {
            diff : IncrementalDiff::from_string(source_code, Parsable::Javascript),
            doc_state : List::<char>::with_capacity(source_code.len()),
            html_string : String::new()
        }
    }

    pub fn refreshState() 
    {

    }

}