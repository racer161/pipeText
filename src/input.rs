use super::{ Core };
use web_sys::{ KeyboardEvent, Selection, Node, console }; //
use wasm_bindgen::prelude::*;

use xi_rope::{ LinesMetric, Rope };

#[wasm_bindgen]
impl Core
{
    pub fn key_down_handler(&mut self, event: KeyboardEvent)
    {

        match self.selection.take()
        {
            Some(sel) => {
                
                let code_node : Node = self.div.clone().into();

                //convert the ranges into std::ops::Ranges
                let mut ranges : Vec<std::ops::Range<usize>> = (0..sel.range_count()).map(|i| { get_range_relative_to_div(sel.get_range_at(i).unwrap(), &code_node) }).collect();

                let key_string : String = match get_string(event, &mut ranges, sel.type_().as_str() == "Caret")
                {
                    Some(s) => s,
                    //we don't respond to these key presses so put back the selection and return
                    None => { self.selection = Some(sel); return }
                };

                //always place the cursor back at the beginning of the first selection
                let cursor_offset : usize = match sel.type_().as_str() == "Caret" && !key_string.is_empty()
                {
                    //its an insert
                    true => { ranges[0].start.clone() + 1 },
                    //its a delete
                    false => { ranges[0].start.clone() }
                };

                let key_rope = Rope::from(key_string.clone());
                let rope_newlines = key_rope.count::<LinesMetric>(key_rope.len());

                //apply the edits to the Rope
                for range in ranges 
                { 
                    let new_end_index = range.start + key_rope.len();

                    let start_row = self.text.count::<LinesMetric>(range.start);
                    let old_end_row = self.text.count::<LinesMetric>(range.end);
                    let new_end_row = start_row + rope_newlines;

                    let start_column_index = self.text.count_base_units::<LinesMetric>(start_row);
                    let end_column_index = self.text.count_base_units::<LinesMetric>(old_end_row);
                    let new_end_column_index = key_rope.count_base_units::<LinesMetric>(rope_newlines);
                    
                    let start_column = range.start - start_column_index;
                    let old_end_column = range.end - end_column_index;
                    let new_end_column = match new_end_column_index > 0
                    {
                        //the end column is on a new line
                        true => key_rope.len() - new_end_column_index,
                        false => key_rope.len() + start_column
                    };

                    self.text.edit(range.clone(), key_rope.clone());

                    self.pipetext_web_instance.edit_tree(
                        range.start as u32,         //start_index
                        range.end as u32,           //old_end_index
                        new_end_index as u32,       // new_end_index 
                        start_row as u32,           //start_row
                        start_column as u32,        //start_column
                        old_end_row as u32,         //old_end_row
                        old_end_column as u32,      //old_end_column
                        new_end_row as u32,         //new_end_row
                        new_end_column as u32       //new_end_column
                    );      

                }

                self.pipetext_web_instance.refresh(self.get_str(), cursor_offset as u32);

                //self.incremental_parse();

                //update the text_content with the changes
                //self.div.set_text_content(Some(self.get_str().as_str()));

                //console::log_1(&JsValue::from(self.get_str()));

                //self.put_cursor_back(sel, cursor_offset as u32);
            },
            None => {}//log_1(&JsValue::from("No Selection!"))
        }
       
    }

    fn put_cursor_back(&mut self, sel : Selection, offset : u32)
    {
        let target_div = self.div.first_child().unwrap();

        let range = self.document.create_range().unwrap();

        range.set_start(&target_div, offset);

        sel.remove_all_ranges();
        sel.add_range(&range);
    }

    pub fn selection_change_handler(&mut self)
    {
        self.selection = self.document.get_selection().unwrap();
    }
}

fn get_range_relative_to_div(range : web_sys::Range, div : &Node) -> std::ops::Range<usize>
{
    let pre_caret_range = range.clone_range();
    //console::log_1(&pre_caret_range.start_container().unwrap());

    pre_caret_range.select_node_contents(div);
    pre_caret_range.set_end(&range.start_container().unwrap(), range.start_offset().unwrap());

    let start_index : usize = pre_caret_range.to_string().length() as usize;
    let end_index : usize = start_index + range.to_string().length() as usize;

    //console::log_2(&JsValue::from(start_index as u32), &JsValue::from(end_index as u32));

    return start_index..end_index;
}

fn get_string(event : KeyboardEvent, ranges : &mut Vec<std::ops::Range<usize>>, is_caret : bool) -> Option<String>
{
    match event.key_code()
    {
        //don't do anything/ put the selection back since we didn't use it
        91  | //OS 
        17  | //Control
        182 | //AudioVolumeDown
        183 | //AudioVolumeUp
        39  | //ArrowRight
        37  | //ArrowLeft
        38  | //ArrowUp
        40  | //ArrowDown
        16  | //Shift
        20    //CapsLock
            =>  None,
        9   => Some("  ".to_owned()), //Tab
        13  => Some("\n".to_owned()), //Enter
        8   => { //Backspace
            //if the type of selection is a caret then set the start of the range -=1 
            //to delete the character behind the cursor
            if is_caret && ranges[0].start > 0 { ranges[0].start -= 1; }
            //return empty str because we're deleting
            Some("".to_owned())
        },
        46  => //Delete
        {
            //if the type of selection is a caret then set the end of the range +=1
            //to delete the character in front of the cursor
            if is_caret && ranges[0].end > 0 { ranges[0].end += 1; }
            //return empty str because we're deleting
            Some("".to_owned())
        }
        _   => Some(event.key())
    }
}