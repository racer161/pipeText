use tree_sitter::{ Node };

//TODO: Finish implementing using web-sys
fn build_node(node : &Node, source_string : &str, cursor_index : &usize)
{
    lazy_static! {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    static ref window = web_sys::window().expect("no global `window` exists");
    static ref document = window.document().expect("should have a document on window");
    //const body = document.body().expect("document should have a body");
    }

    let tagName = if node.is_named() { node.kind() } else { get_tag(node) };

    let htmlNode = document.create_element(tagName);

    setHTMLAttributes(htmlNode, node, cursorIndex);
    
    //the node has no children so set the textContent and return
    if(node.child_count() == 0)
    {
        htmlNode.textContent = node.text;
        return htmlNode;
    }

    let children = node.children;

    let last_child = undefined;

    for child in node.children() 
    {
        let new_child = buildNode(child, srcString, cursorIndex);

        if(last_child)
        {
            let new_whitespace = document.createElement("whitespace");

            let start = last_child.endIndex;
            let end = new_child.startIndex;

            setCursorDiv(new_whitespace, cursorIndex, start, end)

            new_whitespace.textContent = srcString.substring(start, end );
            htmlNode.appendChild(new_whitespace);
        }

        htmlNode.appendChild(new_child);

        last_child = new_child;
    }

    return htmlNode;
}

/*
fn setHTMLAttributes(htmlNode, node, cursorIndex)
{
    htmlNode.start = node.startPosition;
    htmlNode.end = node.endPosition;

    htmlNode.startIndex = node.startIndex;
    htmlNode.endIndex = node.endIndex;

    htmlNode.setAttribute("nodeId", node.id);

    if(node.childCount == 0) setCursorDiv(htmlNode, cursorIndex, node.startIndex, node.endIndex)
}

fn set_cursor_div(htmlNode, cursorIndex, start, end)
{
    if(cursorIndex >= start && cursorIndex < end)
    {

        htmlNode.id = "cursorDiv";
        htmlNode.cursorOffset = cursorIndex-start;
    }
}

fn get_tag(node : &Node, source_string : &str) -> &str
{
    return "undefined";
    //let betweenBits = node.utf8_text(source_string);
    //let tag = betweenBits.trim();
    //tag = matchToken[tag];

    //return tag;
}*/