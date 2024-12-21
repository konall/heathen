use crate::{
    engine::Engine,
    macros::instance,
    selectors::Selector,
    style::Style,
    wit::{
        traits::GuestElement,
        types::{Element, InstanceId}
    }
};

pub(crate) type ElementId = u64;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct ElementX {
    pub(crate) id: ElementId,
    pub(crate) instance_id: InstanceId
}

impl GuestElement for ElementX {
    fn parent(&self) -> Option<Element> {
        instance!(self.instance_id)
            .nodes
            .get(&self.id)
            .map(|node| node.parent)
            .flatten()
            .map(|p| ElementX { id: p, instance_id: self.instance_id })
    }
    
    fn ancestors(&self) -> Vec<Element> {
        let mut res = vec![];
        let mut current = *self;
        
        while let Some(ancestor) = current.parent() {
            res.push(ancestor);
            current = ancestor;
        }
        
        res
    }
    
    fn change_parent(&self, parent: Option<Element>, idx: Option<isize>) -> Option<Element> {
        if !instance!(self.instance_id).nodes.contains_key(&self.id) {
            return None;
        };
        
        let layout_id = instance!(self.instance_id).nodes[&self.id].layout_id;
        
        let prev_p = instance!(self.instance_id).nodes[&self.id].parent;
        if let Some(prev_p) = prev_p {
            if instance!(self.instance_id).nodes.contains_key(&prev_p) {
                instance!(self.instance_id).nodes.get_mut(&prev_p).unwrap().children.retain(|c| self.id != *c);
                
                let prev_parent_layout_id = instance!(self.instance_id).nodes[&prev_p].layout_id;
                instance!(self.instance_id).layout.remove_child(prev_parent_layout_id, layout_id);
            }
        }
        
        if let Some(parent) = parent {
            if instance!(self.instance_id).nodes.contains_key(&parent.id) {
                instance!(self.instance_id).nodes.get_mut(&self.id).unwrap().parent = Some(parent.id);
                
                let num_children = instance!(self.instance_id).nodes[&parent.id].children.len();
                let idx = idx.map(|i| {
                    if i < 0 {
                        std::cmp::max(0, num_children - (i.abs() as usize + 1))
                    } else {
                        std::cmp::min(i as usize, num_children)
                    }
                });
                
                let parent_layout_id = instance!(self.instance_id).nodes[&parent.id].layout_id;
                if let Some(idx) = idx {
                    instance!(self.instance_id).nodes.get_mut(&parent.id).unwrap().children.insert(idx, self.id);
                    
                    let mut children = instance!(self.instance_id).layout.children(parent_layout_id).ok().unwrap_or_default();
                    children.insert(idx, layout_id);
                    instance!(self.instance_id).layout.set_children(parent_layout_id, &children);
                } else {
                    instance!(self.instance_id).nodes.get_mut(&parent.id).unwrap().children.push(self.id);
                    
                    instance!(self.instance_id).layout.add_child(parent_layout_id, layout_id);
                }
            }
        }
        
        prev_p.map(|prev_p| ElementX { id: prev_p, instance_id: self.instance_id })
    }
    
    fn detach(&self) -> Option<Element> {
        self.change_parent(None, None)
    }
    
    fn children(&self) -> Vec<Element> {
        instance!(self.instance_id)
            .nodes
            .get(&self.id)
            .map(|node| {
                node
                    .children
                    .iter()
                    .map(|c| ElementX { id: *c, instance_id: self.instance_id })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    fn descendants(&self) -> Vec<Element> {
        let mut res = vec![];
        
        let mut current = vec![*self];
        let mut nxt = vec![];
        while !current.is_empty() {
            nxt.extend(current.drain(..).map(|el| el.children()).flatten());
            res.extend(nxt.iter());
            
            current = nxt;
            nxt = vec![];
        }
        
        res
    }
    
    fn nth_child(&self, n: isize) -> Option<Element> {
        if !n.is_negative() {
            self.children().get(n as usize).copied()
        } else {
            self.children().into_iter().nth_back((n - 1).abs() as usize)
        }
    }
    
    fn first_child(&self) -> Option<Element> {
        self.nth_child(0)
    }
    
    fn last_child(&self) -> Option<Element> {
        self.nth_child(-1)
    }
    
    fn splice_children(&self, range: std::ops::Range<usize>, replacement: Vec<Element>) -> Vec<Element> {
        instance!(self.instance_id)
            .nodes
            .get_mut(&self.id)
            .map(|node| {
                node
                    .children
                    .splice(range, replacement.into_iter().map(|el| el.id))
                    .map(|c| ElementX { id: c, instance_id: self.instance_id })
                    .collect()
            })
            .unwrap_or_default()
    }
    
    fn siblings(&self) -> Vec<Element> {
        self.prev_siblings().into_iter().chain(self.next_siblings()).collect()
    }
    
    fn nth_sibling(&self, n: isize) -> Option<Element> {
        self
            .parent()
            .map(|p| {
                if !n.is_negative() {
                    p
                        .children()
                        .into_iter()
                        .skip_while(|c| c.id != self.id)
                        .nth((n + 1) as usize)
                } else {
                    p
                        .children()
                        .into_iter()
                        .rev()
                        .skip_while(|c| c.id != self.id)
                        .nth(n.abs() as usize)
                }
            })
            .flatten()
    }
    
    fn next_sibling(&self) -> Option<Element> {
        self.nth_sibling(0)
    }
    
    fn prev_sibling(&self) -> Option<Element> {
        self.nth_sibling(-1)
    }
    
    fn next_siblings(&self) -> Vec<Element> {
        self
            .parent()
            .map(|p| {
                p
                    .children()
                    .into_iter()
                    .skip_while(|c| c.id != self.id)
                    .skip(1)
                    .collect()
            })
            .unwrap_or_default()
    }
    
    fn prev_siblings(&self) -> Vec<Element> {
        self
            .parent()
            .map(|p| {
                p
                    .children()
                    .into_iter()
                    .rev()
                    .skip_while(|c| c.id != self.id)
                    .skip(1)
                    .collect()
            })
            .unwrap_or_default()
    }
    
    fn add_event_listener(&self, event: &str, handler: &str) {
        let selector = format!("%{}", self.id);
        instance!(self.instance_id).listeners.entry(event.into()).or_default().push((Selector::parse(std::borrow::Cow::Owned(selector)), handler.into()));
    }
    
    // fn remove_event_listener(&self, event: &str, handler: &str) {
    //     instance!(self.instance_id).listeners.entry(event.into()).or_default().retain(|(sel, h)| {
    //         !(
    //             sel.as_ref().map(|x| *x == format!("%{}", self.id)).unwrap_or_default()
    //             && (h == handler)
    //         )
    //     });
    // }
    
    fn attribute(&self, name: &str) -> Option<Json> {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.attributes.get(name).cloned()).flatten()
    }
    
    fn attributes(&self) -> std::collections::HashMap<String, Json> {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.attributes.clone()).unwrap_or_default()
    }
    
    fn set_attribute<T: Into<Json>>(&self, name: &str, value: T) -> Option<Json> {
        instance!(self.instance_id).nodes.get_mut(&self.id).map(|node| node.attributes.insert(name.into(), value.into())).unwrap_or_default()
    }
    
    fn remove_attribute(&self, name: &str) -> Option<Json> {
        instance!(self.instance_id).nodes.get_mut(&self.id).map(|node| node.attributes.remove(name)).unwrap_or_default()
    }
    
    fn text(&self) -> Option<String> {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.text.clone()).unwrap_or_default()
    }
    
    fn set_text(&self, new_text: impl Into<String>) -> Option<String> {
        instance!(self.instance_id).nodes.get_mut(&self.id).map(|node| node.text.replace(new_text.into())).unwrap_or_default()
    }
    
    fn tag(&self) -> String {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.tag.clone()).unwrap_or_default()
    }
    
    fn id(&self) -> Option<String> {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.id.clone()).flatten()
    }
    
    fn set_id(&self, new_id: impl Into<String>) -> Option<String> {
        instance!(self.instance_id).nodes.get_mut(&self.id).map(|node| node.id.replace(new_id.into())).flatten()
    }
    
    fn classes(&self) -> Vec<String> {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.classes.iter().cloned().collect()).unwrap_or_default()
    }
    
    fn closest_ancestor(&self, selector: &str) -> Option<Element> {
        let matches = Engine::select(self.instance_id, selector);
        self.ancestors().into_iter().find(|el| matches.contains(el))
    }
    
    fn matches(&self, selector: &str) -> bool {
        Engine::select(self.instance_id, selector).contains(&self)
    }
    
    fn bounding_rectangle(&self) -> lyon::math::Box2D {
        todo!()
    }
    
    fn style(&self) -> Style {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.style.clone()).unwrap_or_default()
    }
    
    fn set_style(&self, new: Style) {
        
    }
    
    fn scroll_to(&self) {
        todo!()
    }
    
    fn scroll_offset(&self) -> lyon::math::Point {
        instance!(self.instance_id).nodes.get(&self.id).map(|node| node.scroll_offset).unwrap_or_default()
    }
    
    fn duplicate(&self) -> ElementX {
        todo!()
    }
    
    fn destroy(self) -> Vec<Element> {
        if !instance!(self.instance_id).nodes.contains_key(&self.id) {
            return vec![];
        }
        
        let children = self.children();
        
        let node = instance!(self.instance_id).nodes.remove(&self.id).unwrap();
        
        if let Some(parent) = node.parent {
            instance!(self.instance_id)
                .nodes
                .get_mut(&parent)
                .map(|p| p.children.retain(|c| *c != self.id));
        }
        
        for child in &node.children {
            instance!(self.instance_id).nodes.get_mut(&child).map(|c| c.parent = None);
        }
        
        instance!(self.instance_id).layout.remove(node.layout_id);
        
        instance!(self.instance_id).animations.retain(|(x, _)| *x != self.id);
        
        children
    }
}
