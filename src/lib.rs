#[wasm_bindgen::prelude::wasm_bindgen(start)]
#[allow(unused_must_use)]
pub fn run() {
    apply_document(&DomElement::Div(TextOrDomElement::DomElement(vec![
        DomElement::Div(TextOrDomElement::Text("あ".to_string())),
        DomElement::Div(TextOrDomElement::Text("い".to_string())),
        DomElement::Div(TextOrDomElement::Text("う".to_string())),
        DomElement::Div(TextOrDomElement::Text("え".to_string())),
        DomElement::Div(TextOrDomElement::Text("お".to_string())),
    ])));
}

fn get_document() -> Option<web_sys::Document> {
    match web_sys::window() {
        Some(window) => match web_sys::Window::document(&window) {
            Some(document) => Some(document),
            None => None,
        },
        None => None,
    }
}

fn apply_document(dom_element: &DomElement) {
    match get_document() {
        Some(document) => match web_sys::Document::body(&document) {
            Some(body) => match dom_element_to_element(dom_element, &document) {
                Some(child) => {
                    let _ = body.append_child(&child);
                }
                None => {}
            },
            None => {}
        },
        None => {}
    };
}

fn dom_element_to_element(
    dom_element: &DomElement,
    document: &web_sys::Document,
) -> Option<web_sys::Element> {
    match dom_element {
        DomElement::Div(text_or_dom_element) => {
            match web_sys::Document::create_element(document, "div") {
                Ok(element) => {
                    match text_or_dom_element {
                        TextOrDomElement::Text(text) => {
                            element.set_text_content(Some(text));
                        }
                        TextOrDomElement::DomElement(dom_element_vec) => {
                            for dom_element in dom_element_vec {
                                match dom_element_to_element(dom_element, document) {
                                    Some(child_element) => {
                                        let _ = element.append_child(&child_element);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    Some(element)
                }
                Err(_) => None,
            }
        }
    }
}

enum DomElement {
    Div(TextOrDomElement),
}

enum TextOrDomElement {
    Text(String),
    DomElement(Vec<DomElement>),
}
