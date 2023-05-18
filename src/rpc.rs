use crate::{
    engine::DOM,
    document::*,
    element::Element
};

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum Id<'a> {
    String(&'a str),
    Number(serde_json::Number)
}

#[derive(serde::Deserialize)]
pub(crate) struct Request<'a> {
    id: Option<Id<'a>>,
    method: &'a str,
    params: serde_json::Map<String, serde_json::Value>
}

#[derive(serde::Serialize)]
pub(crate) struct Error<'a> {
    code: isize,
    message: &'a str,
    data: serde_json::Value
}

#[derive(serde::Serialize)]
pub(crate) struct Response<'a> {
    id: Option<Id<'a>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: Option<Error<'a>>
}
impl Response<'_> {
    pub(crate) fn ok(id: Option<Id>, result: serde_json::Value) -> String {
        let response = Response {
            id,
            result: Some(result),
            error: None
        };
        
        serde_json::to_string(&response).unwrap()
    }
    
    pub(crate) fn err(id: Option<Id>, error: Error) -> String {
        let response = Response {
            id,
            result: None,
            error: Some(error)
        };
        
        serde_json::to_string(&response).unwrap()
    }
}

pub(crate) fn call(request: &str) -> String {
    let Ok(Request { id, method, mut params }) = serde_json::from_str(request) else {
        return Response::ok(None, serde_json::Value::Null);
    };
    
    let (target, fx) = match method.split_once('.') {
        Some((target, fx)) => (target, fx),
        None => ("document", method)
    };
    
    let x = match target {
        "document" => {
            match fx {
                // fn root() -> Element
                "root" => serde_json::to_string(&root()).unwrap(),
                
                // fn create_element(tag, props, children) -> Element
                "create_element" => {
                    let tag = match params["tag"].take() {
                        serde_json::Value::String(tag) => tag,
                        _ => return Response::ok(id, serde_json::Value::Null)
                    };
                    
                    let props = match params["props"].take() {
                        serde_json::Value::Object(props) => Some(props),
                        _ => None
                    };
                    
                    let children = match params["children"].take() {
                        serde_json::Value::Array(children) => {
                            children
                                .into_iter()
                                .filter_map(|c| c.as_u64())
                                .map(|c| Element(c))
                                .collect::<Vec<_>>()
                        },
                        _ => return Response::ok(id, serde_json::Value::Null)
                    };
                    
                    serde_json::to_string(&create_element(&tag, props, &children)).unwrap()
                },
                
                // pub fn active_element() -> Element
                "active_element" => serde_json::to_string(&active_element()).unwrap(),
                
                // fn elements_at_point(point: lyon::math::Point) -> Vec<Element>
                "elements_at_point" => {
                    let point = match serde_json::from_value(params["point"].take()) {
                        Ok(point) => point,
                        _ => return Response::ok(id, serde_json::Value::Null)
                    };
                    
                    serde_json::to_string_pretty(&elements_at_point(point)).unwrap()
                }
                
                // fn remove_event_handler() -> Option<std::rc::Rc<dyn Fn(Event)>>
                // "remove_event_handler" => {}
                
                // fn trigger()
                // "trigger" => {}
                
                // fn select(selector: &str) -> Vec<Element>
                "select" => {
                    let Some(selector) = params["selector"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string_pretty(&select(selector)).unwrap()
                }
                
                // fn select_one(selector: &str) -> Option<Element>
                "select_one" => {
                    let Some(selector) = params["selector"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&select_one(selector)).unwrap()
                }
                
                _ => return Response::ok(id, serde_json::Value::Null)
            }
        },
        
        "element" => {
            let Some(xid) = params["xid"].as_u64() else {
                return Response::ok(id, serde_json::Value::Null);
            };
            let el = {
                if dom!().nodes.get(&xid).is_some() {
                    Element(xid)
                } else {
                    return Response::ok(id, serde_json::Value::Null);
                }
            };
            
            match fx {
                // tree
                
                // -- parent
                
                // fn parent(&self) -> Option<Element>
                "parent" => serde_json::to_string(&el.parent()).unwrap(),
                
                // fn ancestors(&self) -> Vec<Element>
                "ancestors" => serde_json::to_string_pretty(&el.ancestors()).unwrap(),
                
                // fn change_parent(&self, parent: Option<Element>, idx: Option<isize>) -> Option<Element>
                "change_parent" => {
                    let parent = params["parent"].as_u64().map(|p| Element(p));
                    let idx = params["idx"].as_i64().map(|i| i as isize);
                    
                    serde_json::to_string(&el.change_parent(parent, idx)).unwrap()
                }
                
                // fn detach(&self) -> Option<Element>
                "detach" => serde_json::to_string(&el.detach()).unwrap(),
                
                // -- children
                
                // fn children(&self) -> Vec<Element>
                "children" => serde_json::to_string_pretty(&el.children()).unwrap(),
                
                // fn descendants(&self) -> Vec<Element>
                "descendants" => serde_json::to_string_pretty(&el.descendants()).unwrap(),
                
                // fn nth_child(&self, n: isize) -> Option<Element>
                "nth_child" => {
                    let Some(n) = params["n"].as_i64() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.nth_child(n as isize)).unwrap()
                }
                
                // fn first_child(&self) -> Option<Element>
                "first_child" => serde_json::to_string(&el.first_child()).unwrap(),
                
                // fn last_child(&self) -> Option<Element>
                "last_child" => serde_json::to_string(&el.last_child()).unwrap(),
                
                // fn splice_children(&self, range: std::ops::Range<usize>, replacement: Vec<Element>) -> Vec<Element>
                "splice_children" => {
                    let Some(start) = params["start"].as_u64() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    let  Some(end) = params["end"].as_u64() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    let Some(replacement) = params["replacement"].as_array() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    let replacement =
                        replacement
                            .into_iter()
                            .filter_map(|r| r.as_u64())
                            .map(|xid| Element(xid))
                            .collect();
                    
                    serde_json::to_string_pretty(&el.splice_children(start as usize..end as usize, replacement)).unwrap()
                }
                
                // -- siblings

                // fn siblings(&self) -> Vec<Element>
                "siblings" => serde_json::to_string_pretty(&el.siblings()).unwrap(),
                
                // fn nth_sibling(&self, n: isize) -> Option<Element>
                "nth_sibling" => {
                    let Some(n) = params["n"].as_i64() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.nth_sibling(n as isize)).unwrap()
                }
                
                // fn next_sibling(&self) -> Option<Element>
                "next_sibling" => serde_json::to_string(&el.next_sibling()).unwrap(),
                
                // fn prev_sibling(&self) -> Option<Element>
                "prev_sibling" => serde_json::to_string(&el.prev_sibling()).unwrap(),
                
                // fn next_siblings(&self) -> Vec<Element>
                "next_siblings" => serde_json::to_string_pretty(&el.next_siblings()).unwrap(),
                
                // fn prev_siblings(&self) -> Vec<Element>
                "prev_siblings" => serde_json::to_string_pretty(&el.prev_siblings()).unwrap(),
                
                // events
                
                // fn add_event_listener(&self, event: &str, handler: &str)
                "add_event_listener" => {
                    let Some(event) = params["event"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    let Some(handler) = params["handler"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    el.add_event_listener(event, handler);
                    
                    return Response::ok(id, serde_json::Value::Null);
                }
                
                // fn remove_event_listener(&self, event: &str, handler: &str)
                "remove_event_listener" => {
                    let Some(event) = params["event"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    let Some(handler) = params["handler"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    el.remove_event_listener(event, handler);
                    
                    return Response::ok(id, serde_json::Value::Null);
                }
                
                // attributes
                
                // pub fn attribute(&self, name: &str) -> Option<Value>
                "attribute" => {
                    let Some(name) = params["name"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string_pretty(&el.attribute(name)).unwrap()
                }
                
                // pub fn attributes(&self) -> std::collections::HashMap<String, Value> {
                "attributes" => serde_json::to_string_pretty(&el.attributes()).unwrap(),
                
                // pub fn set_attribute<T: Into<String>>(&self, name: &str, value: T) -> Option<Value> {
                "set_attribute" => {
                    let name = match params["name"].take() {
                        serde_json::Value::String(name) => name,
                        _ => return Response::ok(id, serde_json::Value::Null)
                    };
                    
                    let value = params["value"].take();
                    
                    serde_json::to_string_pretty(&el.set_attribute(&name, value)).unwrap()
                }
                
                // pub fn remove_attribute(&self, name: &str) -> Option<Value> {
                "remove_attribute" => {
                    let Some(name) = params["name"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string_pretty(&el.remove_attribute(name)).unwrap()
                }
                
                // text
                
                // fn text(&self) -> Option<String>
                "text" => serde_json::to_string(&el.text()).unwrap(),
                
                // fn set_text(&self, new_text: impl Into<String>) -> Option<String>
                "set_text" => {
                    let Some(new_text) = params["new_text"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.set_text(new_text)).unwrap()
                }
                
                // selectors
                
                // fn tag(&self) -> String
                "tag" => serde_json::to_string(&el.tag()).unwrap(),
                
                // fn id(&self) -> Option<String>
                "id" => serde_json::to_string(&el.id()).unwrap(),
                
                // fn set_id(&self, new_id: impl Into<String>) -> Option<String>
                "set_id" => {
                    let Some(new_id) = params["new_id"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.set_id(new_id)).unwrap()
                }
                
                // fn classes(&self) -> Vec<String>
                "classes" => serde_json::to_string_pretty(&el.classes()).unwrap(),
                
                // fn closest_ancestor(&self, selector: &str) -> Option<Element>
                "closest_ancestor" => {
                    let Some(selector) = params["selector"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.closest_ancestor(selector)).unwrap()
                }
                
                // fn matches(&self, selector: &str) -> bool
                "matches" => {
                    let Some(selector) = params["selector"].as_str() else {
                        return Response::ok(id, serde_json::Value::Null);
                    };
                    
                    serde_json::to_string(&el.matches(selector)).unwrap()
                }
                
                // fn bounding_rectangle(&self) -> lyon::math::Box2D
                // "bounding_rectangle" => {}
                
                // fn delete(&self) -> Vec<Element>
                "delete" => serde_json::to_string_pretty(&el.delete()).unwrap(),
                
                // fn scroll(&self)
                // "scroll" => {}
                
                // fn scroll_offset(&self) -> lyon::math::Point
                "scroll_offset" => serde_json::to_string_pretty(&el.scroll_offset()).unwrap(),
                
                // fn duplicate(&self) -> Element
                // "duplicate" => {},
                
                _ => return Response::ok(id, serde_json::Value::Null)
            }
        },
        
        _ => return Response::ok(None, serde_json::Value::Null)
    };
    
    // x
    // String::default()
    return Response::ok(id, serde_json::Value::from(x));
}
