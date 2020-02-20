import { build_tree } from "./tree-builder.mjs";
import { get_edit } from "./delta.mjs";

import init, { Core } from '../pkg/pipetext.js';



export class PipeText
{
    constructor(div)
    {

        init().then(()=>
        {
            this.div = div;
            
            //create lineNums div
            this.lineDiv = document.createElement('lines');
            this.lineDiv.contentEditable = false;
            
            //move initial text to code div
            this.codeDiv = document.createElement('code');
            this.codeDiv.contentEditable = true;
            this.codeDiv.style = "white-space: pre; float: left;"
            this.codeDiv.innerHTML = this.div.innerHTML;

            this.core = Core.from_div(this.codeDiv, this);

            div.addEventListener("keydown", (e) => 
            {
                event.preventDefault(); 
                event.stopPropagation();
                //console.log(e);
                var t0 = performance.now(); 
                this.core.key_down_handler(e);
                console.log("Updated in " + (performance.now() - t0) + " milliseconds.");
            });

            // addEventListener version
            document.addEventListener('selectionchange', () => 
            {
                this.core.selection_change_handler();
            });

            this.div.innerHTML = "";

            this.div.appendChild(this.lineDiv);
            this.div.appendChild(this.codeDiv);

            let self = this;

            //TODO: make sure plain text only is pasted
            //this.codeDiv.addEventListener("input", function(e) { self.refresh_state(self); });

            this.init(self, "javascript").then((p) => console.log("initialized"));
        });
    }

    refresh(source_string, cursor_index)
    {
        //incremental parse
        if(this.tree) this.last_tree = this.tree; 
        this.tree = this.parser.parse(source_string, this.last_tree);

        let result = build_tree(this.tree, document.createElement("div"), source_string, cursor_index);

        this.codeDiv.innerHTML = "";

        this.codeDiv.appendChild(result.html);
        if(result.cursor_div) placeCursorBack(result.cursor_div, result.offset);

    }

    edit_tree(start_index, old_end_index, new_end_index, 
              start_row, start_column, old_end_row, 
              old_end_column, new_end_row, new_end_column)
    {
        console.log(`{
            startIndex: ${ start_index },
            oldEndIndex: ${ old_end_index },
            newEndIndex: ${ new_end_index},
            startPosition: {row: ${ start_row }, column: ${ start_column }},
            oldEndPosition: {row: ${ old_end_row }, column: ${ old_end_column }},
            newEndPosition: {row: ${ new_end_row }, column: ${ new_end_column }},
          }`);

          //console.log(get_edit(this));

        this.tree.edit({
            startIndex: start_index,
            oldEndIndex: old_end_index,
            newEndIndex: new_end_index,
            startPosition: {row: start_row, column: start_column},
            oldEndPosition: {row: old_end_row, column: old_end_column },
            newEndPosition: {row: new_end_row, column: new_end_column},
        });
    }


    async init(self, language)
    {
        await TreeSitter.init();

        self.parser = new TreeSitter();

        const url = `pipetext/tree-sitter/tree-sitter-${language}.wasm`;

        try { language = await TreeSitter.Language.load(url); } 
        catch (e) { console.error(e); return; }

        self.parser.setLanguage(language);
        self.tree = null;
        self.last_tree = null;

        var t0 = performance.now(); 

        self.tree = self.parser.parse(self.codeDiv.textContent, null);

        console.log("Parse took " + (performance.now() - t0) + " milliseconds.");

        let lineNumbers = self.tree.rootNode.endPosition.row;

        self.updateCodeTree(0, self.codeDiv.textContent);
        self.refreshLineNums(lineNumbers);

        console.log("Total " + (performance.now() - t0) + " milliseconds.");
    }

    
    /*
    async refresh_state(self)
    {
        //console.log(cursorIndex);
        var t0 = performance.now();

        let result = get_edit(self);

        await self.tree.edit(result.edit); 
    
        //Update parser data
        self.lastTextContent = self.codeDiv.textContent;
        await self.incremental_parse(self);

        let lineNumbers = self.tree.rootNode.endPosition.row;

        self.updateCodeTree(self, result.cursor_index);
        

        self.refreshLineNums(lineNumbers,self);

        console.log("Total " + (performance.now() - t0) + " milliseconds.");
    }

    async parse(self) { self.tree = self.parser.parse(self.lastTextContent, self.tree); }
    
    incremental_parse() 
    { 
        if(this.tree) this.last_tree = this.tree; 
        this.tree = this.parser.parse(this.lastTextContent, this.last_tree);
    }*/

    updateCodeTree(cursorIndex, source_string)
    {
        let result = build_tree(this.tree, document.createElement("div"), source_string, cursorIndex);

        this.codeDiv.innerHTML = "";

        this.codeDiv.appendChild(result.html);
        if(result.cursor_div) placeCursorBack(result.cursor_div, result.offset);

        this.lastTextContent = this.codeDiv.textContent;
    }

    refreshLineNums(lines)
    {
        this.lineDiv.innerHTML = "";
        for(var i = 0; i <= lines-1; i++)
        {
            let line = document.createElement('line');
            line.textContent = (i+1).toString();
            this.lineDiv.appendChild(line);
            let newline = document.createTextNode('\n');
            this.lineDiv.appendChild(newline);
        }
    }
}

function placeCursorBack(cursorDiv, cursorOffset)
{
    var range = document.createRange();

    let content  = cursorDiv.childNodes[0];

    range.setStart(content, cursorOffset);
    range.collapse(true);

    var sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(range);   
}