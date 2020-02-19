import { start } from "repl";

function get_cursor_range(element, event, cursor, self)
{

    let newText = self.codeDiv.textContent;
    let oldText = self.getSelection.lastTextContent;
    let delta  = newText.length - oldText.length;

    //return the direction of the event from the cursor
    //+ is an insert
    //- is a deletion

    //[startIndex, oldEndIndex, newEndIndex]
    switch(event.type)
    {
        //INSERT
        case "insertText": 
            return delta;
        case "insertFromPaste": 
            return delta;
        case "insertParagraph":
            return [cursor, cursor, cursor + 1];
        //DELETE
        case "deleteContentBackwards":
            return [delta];
    }



}

export function get_diff_from_input_event(element, event, cursor)
{

    //let diff = get_diff_from_input_event(element, event, cursor);
    let newText = self.codeDiv.textContent;
    let oldText = self.getSelection.lastTextContent;
    let delta  = newText.length - oldText.length;

    //if the delta is greater than zero we added characters
    //which means the current cursor location is the end of the insert
    //otherwise 
    let startIndex = delta > 0 ? cursor - delta: cursor;

    //we still subtract the delta if its negative because -- = +
    let endIndex = delta > 0 ? cursor : cursor - delta;

    let diff = [startIndex, endIndex];

    let length = Math.max(0, newText.length - self.lastTextContent.length);

    let startLines = self.lastTextContent.substring(0, diff[0]).split(/\r\n|\r|\n/);
    let endLines = self.lastTextContent.substring(0, diff[1]).split(/\r\n|\r|\n/);

    let startRow = startLines.length-1;
    let endRow = endLines.length-1;

    let startColumn = startLines[startRow].length;
    let endColumn = endLines[endRow].length;

    let deltaRow = diff[2].split(/\r\n|\r|\n/);
    let deltaRowLength = deltaRow.length - 1;

    //if we're on the same line add the length
    let deltaColumn = (deltaRowLength > 0) ?  length : startColumn + length;

    let edit = {
        startIndex: diff[0],
        oldEndIndex: diff[1],
        newEndIndex: diff[0] + diff[2].length,
        startPosition: {row: startRow, column: startColumn},
        oldEndPosition: {row: endRow, column: endColumn },
        newEndPosition: {row: startRow + deltaRowLength, column: deltaColumn},
    };

    return edit;
}