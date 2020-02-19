use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    pub type Tree;

    #[wasm_bindgen(method, getter)]
    pub fn rootNode(this: &Tree) -> Node;

    #[wasm_bindgen(method)]
    pub fn walk(this: &Tree) -> TreeCursor;
}

#[wasm_bindgen]
extern "C" { pub type Node; }

#[wasm_bindgen]
extern "C" {
    pub type TreeSitter;

    #[wasm_bindgen(constructor)]
    pub fn new() -> TreeSitter;

    #[wasm_bindgen(method)]
    pub fn setLanguage(this : &TreeSitter, e : &JsValue);

    #[wasm_bindgen(method)]
    pub fn parse(this : &TreeSitter, s : &str, t : &JsValue) -> Tree;

    #[wasm_bindgen]
    pub fn get_lang(e : String) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type TreeCursor;

    #[wasm_bindgen(method, getter)]
    pub fn rootNode(this: &TreeCursor) -> Node;

    #[wasm_bindgen(method, getter)]
    pub fn nodeType(this: &TreeCursor) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn nodeTypeId(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn nodeId(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn nodeIsNamed(this: &TreeCursor) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn nodeIsMissing(this: &TreeCursor) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn startPosition(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn endPosition(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn startIndex(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn endIndex(this: &TreeCursor) -> u32;

    #[wasm_bindgen(method)]
    pub fn gotoFirstChild(this: &TreeCursor) -> bool;

    #[wasm_bindgen(method)]
    pub fn gotoNextSibling(this: &TreeCursor) -> bool;

    #[wasm_bindgen(method)]
    pub fn gotoParent(this: &TreeCursor) -> bool;
}