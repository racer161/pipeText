use crate::tree_sitter_wasm_bindings::{ TreeCursor };
use web_sys::{ Element, console };
use xi_rope::{ Rope };

use wasm_bindgen::prelude::*;

use super::Pipetext;

//#[wasm_bindgen]
impl Pipetext
{
    //renders the html and returns a reference to the div the cursor needs to be placed in
    // as well as the offset at which it should be placed
    pub fn render_html(&self, cursor_index : u32) -> (Element, Element, u32)
    {
        let cursor : TreeCursor = match &self.tree
        {
            Some(tree) => tree.walk(),
            None => panic!("Pipetext Internal Error: render_html called before tree initialized")
        };

        //console::log_1(&JsValue::from(&cursor));

        let source_string : String = String::from(&self.text);

        let mut visitedChildren = false;
        let mut parent : Element = self.document.create_element("code").unwrap();
        let mut cursor_div : Element = parent.clone();
        let mut cursor_offset : u32 = 0;

        loop
        {
            if visitedChildren
            {
                //get the last node endIndex
                let start = cursor.endIndex();
                //there is a next sibling so we haven't visited all the children
                if cursor.gotoNextSibling()
                {
                    // we've gone to the next sibling which means there might be whitespace 
                    //in between this sibling and the last sibling
                    let new_whitespace = self.document.create_element("whitespace").unwrap();

                    let end = cursor.startIndex().clone();

                    //if this is the cursor div
                    if cursor_index >= start && cursor_index < end { cursor_div = new_whitespace.clone(); cursor_offset = cursor_index - start; }

                    new_whitespace.set_text_content(Some(&source_string[start as usize..end as usize]));
                    parent.append_child(&new_whitespace);

                    visitedChildren = false; 
                }
                //there's no next sibling so we go back to the parent
                else if cursor.gotoParent() { visitedChildren = true; parent = parent.parent_element().unwrap(); } //console.log("parent : " + parent.parentNode);
                //otherwise we're back at the root so we've covered the whole tree
                else { break };
            } 
            else 
            {
                let temp_node = self.get_html(&cursor, &source_string);
                parent.append_child(&temp_node);

                //if there's a first child we make the node we just added the new parent
                if cursor.gotoFirstChild() { visitedChildren = false; parent = temp_node; } //console.log("first_child : " + temp_node);
                //otherwise there are no children for this node which means the inner text is the only child so we set textContent
                else 
                { 
                    visitedChildren = true; 
                    temp_node.set_text_content(Some(&source_string[cursor.startIndex() as usize..cursor.endIndex()as usize]));
                    //if this is the cursor div
                    if cursor_index >= cursor.startIndex() && cursor_index < cursor.endIndex() { cursor_div = temp_node; cursor_offset = cursor_index - cursor.startIndex(); }
                }
            }
        }

        return (parent, cursor_div, cursor_offset);
    }

    fn get_html(&self, cursor : &TreeCursor, source_string : &String) -> Element
    {
        //TODO: Handle the missing node case
        let tagName : String = match cursor.nodeIsNamed()
        {
            true => cursor.nodeType(),
            false => get_match_token(&source_string[cursor.startIndex() as usize..cursor.endIndex() as usize])
        };


        let html_node = self.document.create_element(&tagName).unwrap();

        //TODO : add function to set debug info

        return html_node;
    }
}

fn get_match_token(tag : &str) -> String
{
    match tag
    {
        "{" => "open_bracket".to_owned(),
        "}" => "close_bracket".to_owned(),
        "(" => "open_paren".to_owned(),
        ")" => "close_paren".to_owned(),
        ";" => "semicolon".to_owned(),
        ":" => "colon".to_owned(),
        "," => "comma".to_owned(),
        r#"'"# => "single_quote".to_owned(),
        "\"" => "double_quote".to_owned(),
        "while" => "while".to_owned(),
        "import" => "import".to_owned(),
        "from" => "from".to_owned(),
        "export" => "export".to_owned(),
        "default" => "default".to_owned(),
        "if" => "if".to_owned(),
        "const" => "const".to_owned(),
        "static" => "static".to_owned(),
        "return" => "return".to_owned(),
        "function" => "function_keyword".to_owned(),
        _ => "undefined".to_owned()
    }
}