use crate::{
    engine::Engine,
    element::ElementX,
    events::Event,
    macros::instance,
    node::{Node, Vertex},
    style::Style,
    wit::{
        traits::{GuestInstance, GuestValue},
        types::{Element, InstanceId, Value}
    },
    element
};

use std::collections::HashMap;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct InstanceX(pub(crate) InstanceId);

impl GuestInstance for InstanceX {
    fn id(&self) -> InstanceId {
        self.0
    }

    fn root(&self) -> Option<Element> {
        instance!(self.0).root.map(|root| Element::new(ElementX { xid: root, iid: self.0 }))
    }

    fn create_element(&self, tag: String, props: Vec<(String, Value)>, children: Vec<Element>) -> Element {
        let mut props = props.into_iter().collect::<HashMap<_, _>>();
        
        let id = props.remove("id").map(|id| id.as_string().map(|id| id.to_string())).flatten();
        let classes =
            props.remove("classes").map(|classes| {
                classes.as_array().map(|classes| {
                    classes
                        .into_iter()
                        .filter_map(|class| class.as_string().map(|class| class.to_string()))
                        .collect::<std::collections::HashSet<String>>()
                })
            })
            .flatten()
            .unwrap_or_default();
        
        let text = props.remove("text").map(|text| text.as_string().map(|text| text.to_string())).flatten();
        
        let style = props.remove("style").map(|style| serde_json::from_value::<Style>(style).ok()).flatten().unwrap_or_default();
        
        let el = {
            let maybe_component = instance!(self.0).components.get(tag).cloned(); // avoid deadlock
            if let Some(component) = maybe_component {
                component()
            } else {
                let xid = Engine::xid(self.0);
                let layout_id = instance!(self.0).layout.new_leaf(style.layout.clone()).unwrap();
                instance!(self.0).nodes.insert(xid, Node {
                    xid,
                    layout_id,
                    tag: tag.into(),
                    id,
                    classes,
                    text,
                    style,
                    attributes: props,
                    ..Default::default()
                });
                ElementX { xid, iid: self.0 }
            }
        };
        
        for child in children {
            child.change_parent(Some(el), None);
        }
        
        el
    }

    fn active_element(&self) -> Element {
        instance!(self.0).state.focused()
    }
    
    // pub fn elements_at_point(&self, point: lyon::math::Point) -> Vec<Element> {
    //     instance!(self.0)
    //         .nodes
    //         .values()
    //         .filter(|node| node.is_within(point))
    //         .into_iter()
    //         .map(|node| Element { xid: node.xid, iid: self.0 })
    //         .collect()
    // }
    
    // pub fn remove_event_handler(&self) -> Option<std::rc::Rc<dyn Fn(Event)>> {
    //     todo!()
    // }

    fn trigger(&self) {
        todo!() // return bool indicating if event was cancelled?
    }

    fn select(&self, selector: String) -> Vec<Element> {
        Engine::select(self.0, &selector).into_iter().map(Element::new).collect()
    }

    fn select_one(&self, selector: String) -> Option<Element> {
        Engine::select(self.0, &selector).first().copied().map(Element::new)
    }
    
    // pub fn render(&self, root: Element) -> (Vec<Vertex>, Vec<u16>) {
    //     let buffers = Engine::render(self.0, root.xid);
    //     (buffers.vertices, buffers.indices)
    // }

    fn destroy(&self) {
        crate::ENGINES.get_or_init(|| Default::default()).remove(&self.0);
    }
}
