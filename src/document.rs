use crate::{
    Value,
    Xid,
    element::Element,
    events::Event,
    engine::{DOM, Engine},
    node::Node
};

pub fn root() -> Element {
    Element(0)//TODO
}

pub fn create_element(
    tag: &str,
    props: Option<impl IntoIterator<Item = (impl Into<String>, impl Into<Value>)>>,
    children: &[Element]
) -> Element {
    let xid = Engine::xid();
    let mut node = Node {
        xid,
        tag: tag.into(),
        attributes: {
            props
                .map(|prop| {
                    prop
                        .into_iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect()
                })
                .unwrap_or_default()
        },
        
        ..Default::default()
    };
    let el = Element(xid);
    
    for child in children {
        node.children.push(child.0);
        child.change_parent(Some(el), None);
    }
    
    dom!().nodes.insert(xid, node);
    
    el
}

pub fn active_element() -> Element {
    Element(dom!().state.focused)
}

pub fn elements_at_point(point: lyon::math::Point) -> Vec<Element> {
    dom!().nodes.keys().filter(|uuid| Engine::is_within(**uuid, point)).into_iter().map(|id| Element(*id)).collect()
}

#[cfg(feature = "wust")]
pub fn register_event_handler(name: impl Into<String>, handler: impl Fn(Event) + Send + Sync + 'static) -> String {
    dom!().handlers.insert(name.into(), std::sync::Arc::new(handler));
    
    label
}

pub fn remove_event_handler() -> Option<std::rc::Rc<dyn Fn(Event)>> {
    todo!()
}

// return bool indicating if event was cancelled?
pub fn trigger() {
    todo!()
}

pub fn select(selector: &str) -> Vec<Element> {
    Engine::select(selector)
}

pub fn select_one(selector: &str) -> Option<Element> {
    Engine::select(selector).first().copied()
}
