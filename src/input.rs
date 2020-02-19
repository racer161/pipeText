use super::{ Pipetext };
use web_sys::{ KeyboardEvent, Selection, console }; //
use wasm_bindgen::prelude::*;

use xi_rope::{ LinesMetric, Rope };

#[wasm_bindgen]
impl Pipetext
{
    pub fn key_down_handler(&mut self, event: KeyboardEvent)
    {

        match self.selection.take()
        {
            Some(sel) => {
                
                //convert the ranges into std::ops::Ranges
                let mut ranges : Vec<std::ops::Range<usize>> = (0..sel.range_count()).map(|i| 
                { 
                    let range = sel.get_range_at(i).unwrap(); 
                    (range.start_offset().unwrap() as usize..range.end_offset().unwrap() as usize)
                }).collect();

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
                    let before_line_nums = self.text.count::<LinesMetric>(range.start);
                    let after_line_nums = before_line_nums + rope_newlines;

                    self.text.edit(range, key_rope.clone());
                    
                    console::log_2(&JsValue::from(before_line_nums as u32), &JsValue::from(after_line_nums as u32));
                }

                //self.incremental_parse();

                //update the text_content with the changes
                self.div.set_text_content(Some(self.get_str().as_str()));

                self.put_cursor_back(sel, cursor_offset as u32);
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