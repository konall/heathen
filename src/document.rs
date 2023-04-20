use crate::{
    Value,
    Xid,
    element::Element,
    events::Event,
    engine::{DOM, Engine},
    node::Node
};

pub struct Document;

impl Document {
    pub fn root() -> Element {
        Element(0)//TODO
    }
    
    pub fn create_element(tag: &str, props: Option<std::collections::HashMap<String, Value>>, children: &[Element]) -> Element {
        let xid = Engine::xid();
        let mut node = Node {
            xid,
            tag: tag.into(),
            attributes: props.unwrap_or_default(),
            
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
        dom!().state.focused
    }
    
    pub fn elements_at_point(point: lyon::math::Point) -> Vec<Element> {
        dom!().nodes.keys().filter(|uuid| Engine::is_within(**uuid, point)).into_iter().map(|id| Element(*id)).collect()
    }
    
    #[cfg(feature = "wust")]
    pub fn register_event_handler(name: Option<&str>, handler: impl Fn(Event) + Send + Sync + 'static) -> String {
        let label = name
            .map(|n| n.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        DOM
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .handlers
            .insert(label.clone(), std::sync::Arc::new(handler));
        
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
}
