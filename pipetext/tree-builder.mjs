//let node = get_html(n,0);
function get_html(cursor, srcString)
{
    //TODO: Handle the missing node case
    let tagName = cursor.nodeIsNamed ? cursor.nodeType : getTag(cursor, srcString);

    let html_node = document.createElement(tagName);

    //html_node.start = cursor.startPosition;
    //html_node.end = cursor.endPosition;

    //html_node.startIndex = cursor.startIndex;
    //html_node.endIndex = cursor.endIndex;

    //html_node.setAttribute("node-id", cursor.nodeId);

    return html_node;
}

function getTag(cursor, srcString)
{
    let betweenBits = srcString.substring(cursor.startIndex, cursor.endIndex);
    let tag = betweenBits.trim();
    tag = matchToken[tag];

    return tag;
}

const matchToken = 
{
    "{" : "open_bracket",
    "}" : "close_bracket",
    "(" : "open_paren",
    ")" : "close_paren",
    ";" : "semicolon",
    ":" : "colon",
    "," : "comma",
    '"' : "quote",
    "while" : "while",
    "import" : "import",
    "from" : "from",
    "export" : "export",
    "default" : "default",
    "if" : "if",
    "const" : "const",
    "static" : "static",
    "return" : "return",
    "function" : "function_keyword"
}

export function build_tree(tree, root_html, srcString, cursor_index) 
{
    const cursor = tree.walk();
    let visitedChildren = false;
    var parent = root_html;
    var cursor_div;
    var cursor_offset = 0;

    while(true) 
    {
      if (visitedChildren) 
      {
        let last_end_index = cursor.endIndex;
        //there is a next sibling so we haven't visited all the children
        if (cursor.gotoNextSibling()) 
        {
            // we've gone to the next sibling which means there might be whitespace 
            //in between this sibling and the last sibling
            let new_whitespace = document.createElement("whitespace");

            let start = last_end_index;
            let end = cursor.startIndex;

            //if this is the cursor div
            if(cursor_index >= start && cursor_index < end) { cursor_div = new_whitespace; cursor_offset = cursor_index - start }

            //setCursorDiv(new_whitespace, 0, start, end)

            new_whitespace.textContent = srcString.substring(start, end );
            parent.appendChild(new_whitespace);

            visitedChildren = false; 
        }
        //there's no next sibling so we go back to the parent
        else if (cursor.gotoParent()) { visitedChildren = true; parent = parent.parentNode; /*indentLevel--;*/ } //console.log("parent : " + parent.parentNode);
        //otherwise we're back at the root so we've covered the whole tree
        else break;
      } 
      else 
      {
        let temp_node = get_html(cursor, srcString);
        parent.appendChild(temp_node);

        //if there's a first child we make the node we just added the new parent
        if (cursor.gotoFirstChild()) { visitedChildren = false; parent = temp_node; /*indentLevel++;*/ } //console.log("first_child : " + temp_node);
        //otherwise there are no children for this node which means the inner text is the only child so we set textContent
        else 
        { 
            visitedChildren = true; 
            temp_node.textContent = srcString.substring(cursor.startIndex, cursor.endIndex );
            //if this is the cursor div
            if(cursor_index >= cursor.startIndex && cursor_index < cursor.endIndex) { cursor_div = temp_node; cursor_offset = cursor_index - cursor.startIndex; }
        }
      }
    }

    return { html : parent, cursor_div : cursor_div, offset : cursor_offset };
}


function get_node_string(node)
{
    return ` ${node.type} : { id : ${ node.id } }`;
}


export function render_rows(tree) 
{
    const cursor = tree.walk();

    //let currentRenderCount = parseCount;
    let row = '';
    let rows = [];
    let finishedRow = false;
    let visitedChildren = false;
    let indentLevel = 0;

    while(true) {

      let displayName;
      if (cursor.nodeIsMissing) {
        displayName = `MISSING ${cursor.nodeType}`
      } else if (cursor.nodeIsNamed) {
        displayName = cursor.nodeType;
      }

      if (visitedChildren) 
      {
        if (displayName) {
          finishedRow = true;
        }

        //there is a next sibling so we haven't visited all the children
        if (cursor.gotoNextSibling()) visitedChildren = false;
        //there's no next sibling so we go back to the parent
        else if (cursor.gotoParent()) { visitedChildren = true; indentLevel--; } 
        //otherwise we're back at the root so we've covered the whole tree
        else break;
      } else {
        if (displayName) {
          if (finishedRow) {
            row += '</div>';
            rows.push(row);
            finishedRow = false;
          }
            const start = cursor.startPosition;
            const end = cursor.endPosition;
            const id = cursor.nodeId;
            let fieldName = cursor.currentFieldName();
            if (fieldName) {
                fieldName += ': ';
            } else {
                fieldName = '';
            }
            row = `<div>${'  '.repeat(indentLevel)}${fieldName}<a class='plain' href="#" data-id=${id} data-range="${start.row},${start.column},${end.row},${end.column}">${displayName}</a> [${start.row}, ${start.column}] - [${end.row}, ${end.column}])`;
            finishedRow = true;
        }

        if (cursor.gotoFirstChild()) { visitedChildren = false; indentLevel++; } 
        else visitedChildren = true;
      }
    }
    if (finishedRow) {
      row += '</div>';
      rows.push(row);
    }

    return rows.join('');
}