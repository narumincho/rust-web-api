use wasm_bindgen;
use web_sys::Document;
use web_sys::Node;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
#[allow(unused_must_use)]
pub fn run() {
    let document_maybe = get_document();
    match document_maybe {
        Some(document) => {
            Document::set_title(&document, "やああ");
            match Document::create_element(&document, "div") {
                Ok(div) => {
                    Node::set_text_content(&div, Some("それな"));
                    match Document::body(&document) {
                        Some(body) => {
                            Node::append_child(&body, &div);
                            ()
                        }
                        None => {}
                    }
                }
                Err(_) => {}
            }
        }
        None => {}
    }
}

fn get_document() -> Option<Document> {
    match web_sys::window() {
        Some(window) => match web_sys::Window::document(&window) {
            Some(document) => Some(document),
            None => None,
        },
        None => None,
    }
}
