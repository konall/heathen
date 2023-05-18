use crate::{
    Xid,
    engine::{Engine, DOM},
    events::Event,
    style::Style,
    selectors::Selector,
    Value
};

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq)]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Element(pub(crate) Xid);

impl Element {
    // tree
    
    // -- parent
    
    pub fn parent(&self) -> Option<Element> {
        dom!().nodes.get(&self.0).unwrap().parent.map(|xid| Element(xid))
    }
    
    pub fn ancestors(&self) -> Vec<Element> {
        let mut res = vec![];
        let mut current = *self;
        while let Some(ancestor) = current.parent() {
            res.push(ancestor);
            current = ancestor;
        }
        res
    }
    
    pub fn change_parent(&self, parent: Option<Element>, idx: Option<isize>) -> Option<Element> {
        let mut dom = dom!();
        
        let prev_parent = dom.nodes.get(&self.0).unwrap().parent;
        if let Some(prev_parent) = prev_parent {
            dom.nodes.get_mut(&prev_parent).unwrap().children.retain(|c| self.0.ne(c));
            
            let parent = dom.nodes.get(&prev_parent).unwrap().layout_id;
            let child = dom.nodes.get(&self.0).unwrap().layout_id;
            dom.layout.remove_child(parent, child).unwrap();
        }
        
        if let Some(parent) = parent {
            let parent_layout_id = dom.nodes.get(&parent.0).unwrap().layout_id;
            dom.nodes.get_mut(&self.0).unwrap().parent = Some(parent.0);
            
            let num_children = dom.nodes.get(&parent.0).unwrap().children.len();
            let idx = idx.map(|i| {
                if i < 0 {
                    std::cmp::max(0, num_children - (i.abs() as usize + 1))
                } else {
                    std::cmp::min(i as usize, num_children)
                }
            });

            if let Some(idx) = idx {
                dom.nodes.get_mut(&parent.0).unwrap().children.insert(idx, self.0);
                
                let mut children = dom.layout.children(parent_layout_id).unwrap();
                children.insert(idx, dom.nodes.get(&self.0).unwrap().layout_id);
                
                dom.layout.set_children(parent_layout_id, children.as_slice());
            } else {
                dom.nodes.get_mut(&parent.0).unwrap().children.push(self.0);
                
                let new_child = dom.nodes.get(&self.0).unwrap().layout_id;
                dom.layout.add_child(parent_layout_id, new_child);
            }
        }
        
        prev_parent.map(|uuid| Element(uuid))
    }
    
    pub fn detach(&self) -> Option<Element> {
        self.change_parent(None, None)
    }
    
    // -- children
    
    pub fn children(&self) -> Vec<Element> {
        dom!().nodes.get(&self.0).unwrap().children.iter().map(|uuid| Element(*uuid)).collect()
    }
    
    pub fn descendants(&self) -> Vec<Element> {
        let mut res = vec![];
        let mut current = vec![*self];
        let mut nxt = vec![];
        loop {
            nxt.extend(
                current
                    .drain(..)
                    .map(|el| el.children())
                    .flatten()
            );
            res.extend(nxt.iter());
            
            if nxt.is_empty() {
                break res;
            } else {
                current = nxt;
                nxt = vec![];
            }
        }
    }
    
    pub fn nth_child(&self, n: isize) -> Option<Element> {
        if !n.is_negative() {
            self.children().get(n as usize).copied()
        } else {
            self.children().into_iter().nth_back((n - 1).abs() as usize)
        }
    }
    
    pub fn first_child(&self) -> Option<Element> {
        self.nth_child(0)
    }
    
    pub fn last_child(&self) -> Option<Element> {
        self.nth_child(-1)
    }
    
    pub fn splice_children(&self, range: std::ops::Range<usize>, replacement: Vec<Element>) -> Vec<Element> {
        dom!()
            .nodes
            .get_mut(&self.0)
            .unwrap()
            .children
            .splice(range, replacement.into_iter().map(|el| el.0))
            .map(|uuid| Element(uuid))
            .collect()
    }
    
    // -- siblings

    pub fn siblings(&self) -> Vec<Element> {
        self.prev_siblings().into_iter().chain(self.next_siblings()).collect()
    }
    
    pub fn nth_sibling(&self, n: isize) -> Option<Element> {
        self.parent().map(|parent| {
            if !n.is_negative() {
                parent.children()
                    .into_iter()
                    .skip_while(|x| x.0 != self.0)
                    .nth((n + 1) as usize)
            } else {
                parent.children()
                    .into_iter()
                    .rev()
                    .skip_while(|x| x.0 != self.0)
                    .nth(n.abs() as usize)
            }
        })
        .flatten()
    }
    
    pub fn next_sibling(&self) -> Option<Element> {
        self.nth_sibling(0)
    }
    
    pub fn prev_sibling(&self) -> Option<Element> {
        self.nth_sibling(-1)
    }
    
    pub fn next_siblings(&self) -> Vec<Element> {
        self.parent().map(|parent| {
            parent.children()
                .into_iter()
                .skip_while(|x| x.0 != self.0)
                .skip(1)
                .collect()
        })
        .unwrap_or_default()
    }
    
    pub fn prev_siblings(&self) -> Vec<Element> {
        self.parent().map(|parent| {
            parent.children()
                .into_iter()
                .rev()
                .skip_while(|x| x.0 != self.0)
                .skip(1)
                .collect()
        })
        .unwrap_or_default()
    }
    
    // events
    
    // pub fn add_event_listener(&self, event: &str, handler: &str) {
    //     let selector = format!("%{}", self.0);
    //     dom!().listeners.entry(event.into()).or_default().push((Selector::parse(selector.as_str()), handler.into()));
    // }
    pub fn add_event_listener(&self, event: &str, handler: &str) {
        let selector = format!("%{}", self.0);
        dom!().listeners.entry(event.into()).or_default().push((Some(selector), handler.into()));
    }
    
    pub fn remove_event_listener(&self, event: &str, handler: &str) {
        dom!().listeners.entry(event.into()).or_default().retain(|(sel, h)| {
            !(
                sel.as_ref().map(|x| *x == format!("%{}", self.0)).unwrap_or_default()
                && (h == handler)
            )
        });
    }
    
    // attributes
    
    pub fn attribute(&self, name: &str) -> Option<Value> {
        let mut current = *self;
        loop {
            if let Some(value) = current.attributes().get(name) {
                break Some(value.clone());
            }
            
            if let Some(parent) = current.parent() {
                current = parent;
            } else {
                break None;
            }
        }
    }
    
    pub fn attributes(&self) -> std::collections::HashMap<String, Value> {
        dom!().nodes.get(&self.0).unwrap().attributes.clone()
    }
    
    pub fn set_attribute<T: Into<Value>>(&self, name: &str, value: T) -> Option<Value> {
        dom!().nodes.get_mut(&self.0).unwrap().attributes.insert(name.into(), value.into())
    }
    
    pub fn remove_attribute(&self, name: &str) -> Option<Value> {
        dom!().nodes.get_mut(&self.0).unwrap().attributes.remove(name)
    }
    
    // text
    
    pub fn text(&self) -> Option<String> {
        dom!().nodes.get(&self.0).unwrap().text.clone()
    }
    
    pub fn set_text(&self, new_text: impl Into<String>) -> Option<String> {
        dom!().nodes.get_mut(&self.0).unwrap().text.replace(new_text.into())
    }
    
    // selectors
    
    pub fn tag(&self) -> String {
        dom!().nodes.get(&self.0).unwrap().tag.clone()
    }
    
    pub fn id(&self) -> Option<String> {
        dom!().nodes.get(&self.0).unwrap().id.clone()
    }
    
    pub fn set_id(&self, new_id: impl Into<String>) -> Option<String> {
        dom!().nodes.get_mut(&self.0).unwrap().id.replace(new_id.into())
    }
    
    pub fn classes(&self) -> Vec<String> {
        dom!().nodes.get(&self.0).unwrap().classes.iter().cloned().collect()
    }
    
    pub fn closest_ancestor(&self, selector: &str) -> Option<Element> {
        let matches = Engine::select(format!("%{} {}", self.0, selector).as_str());
        if !matches.is_empty() {
            let mut current = *self;
            while let Some(parent) = current.parent() {
                if matches.contains(&parent) {
                    return Some(current);
                }
                current = parent;
            }
        }
        None
    }
    
    pub fn matches(&self, selector: &str) -> bool {
        Engine::select(selector).contains(&self)
    }
    
    pub fn bounding_rectangle(&self) -> lyon::math::Box2D {
        todo!()
    }
    
    pub fn style(&self) -> Style {
        dom!().nodes.get(&self.0).unwrap().style.clone()
    }
    
    pub fn set_style(&self, new_style: Style) -> Style {
        std::mem::replace(&mut dom!().nodes.get_mut(&self.0).unwrap().style, new_style)
    }
    
    pub fn delete(&self) -> Vec<Element> {
        let children = self.children();
        
        let mut dom = dom!();
        
        let node = dom.nodes.remove(&self.0).unwrap();
        
        if let Some(parent) = node.parent {
            dom
                .nodes
                .get_mut(&parent)
                .unwrap()
                .children
                .retain(|c| *c != self.0);
        }
        
        for child in &node.children {
            dom.nodes.get_mut(&child).unwrap().parent = None;
        }
        
        dom.layout.remove(node.layout_id);
        
        dom.animations.retain(|(x, _)| *x != self.0);
        
        dom.available_xids.push(self.0);
        
        children
    }
    
    pub fn scroll(&self) {
        todo!()
    }
    
    pub fn scroll_offset(&self) -> lyon::math::Point {
        dom!().nodes.get(&self.0).unwrap().scroll_offset
    }
    
    pub fn duplicate(&self) -> Element {
        todo!()
    }
}
