import { build_tree } from "./tree-builder.mjs";
import { get_edit } from "./delta.mjs";

export class PipeText
{
    constructor(div)
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

        this.div.innerHTML = "";

        this.div.appendChild(this.lineDiv);
        this.div.appendChild(this.codeDiv);

        this.lastTextContent = this.codeDiv.textContent;

        let self = this;

        //TODO: make sure plain text only is pasted
        this.codeDiv.addEventListener("input", function(e) { self.refresh_state(self); });

        this.init(self, "javascript").then((p) => console.log("initialized"));

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

        await self.initState(self);
    }

    async initState(self, cursorIndex)
    {
        var t0 = performance.now(); 

        await self.incremental_parse(self);

        console.log("Parse took " + (performance.now() - t0) + " milliseconds.");

        let lineNumbers = self.tree.rootNode.endPosition.row;

        await self.updateCodeTree(self, cursorIndex);
        await self.refreshLineNums(lineNumbers,self);

        console.log("Total " + (performance.now() - t0) + " milliseconds.");
    }

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

    async incremental_parse(self) 
    { 
        if(self.tree) self.last_tree = self.tree; 
        self.tree = self.parser.parse(self.lastTextContent, self.last_tree);
    }

    updateCodeTree(self, cursorIndex)
    {
        let result = build_tree(self.tree, document.createElement("div"), self.lastTextContent, cursorIndex);

        self.codeDiv.innerHTML = "";

        self.codeDiv.appendChild(result.html);
        if(result.cursor_div) placeCursorBack(result.cursor_div, result.offset);

        self.lastTextContent = self.codeDiv.textContent;
    }

    refreshLineNums(lines,self)
    {
        self.lineDiv.innerHTML = "";
        for(var i = 0; i <= lines-1; i++)
        {
            let line = document.createElement('line');
            line.textContent = (i+1).toString();
            self.lineDiv.appendChild(line);
            let newline = document.createTextNode('\n');
            self.lineDiv.appendChild(newline);
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