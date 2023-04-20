use crate::{
    Value,
    Xid,
    node::Node,
    element::Element,
    animations::Animation,
    events::*,
    style::Style,
    parsing::{Selectors, SelectorAttributeRuleType, SelectorLink},
};

#[derive(Copy, Clone, Default)]
pub(crate) struct State {
    pub(crate) mouse_pos: lyon::math::Point,
    pub(crate) hovered: Element,
    pub(crate) focused: Element,
    pub(crate) window_size: lyon::math::Size,
    // pub modifiers: winit::event::ModifiersState,
}

pub(crate) static DOM: once_cell::sync::OnceCell<std::sync::Arc<std::sync::Mutex<Engine>>> = once_cell::sync::OnceCell::new();
macro_rules! dom {
    () => {
        DOM
            .get_or_init(|| {
                let mut engine = crate::engine::Engine::default();
                engine.layout = taffy::Taffy::new();
                
                engine.nodes.insert(0, crate::node::Node {
                    tag: "h1".into(),
                    id: Some("hello".into()),
                    ..Default::default()
                });
                
                std::sync::Arc::new(std::sync::Mutex::new(engine))
            })
            .lock()
            .unwrap()
    }
}

#[derive(Default)]
pub(crate) struct Engine {
    pub(crate) state: State,
    pub(crate) layout: taffy::Taffy,
    pub(crate) nodes: std::collections::HashMap<Xid, Node>,
    pub(crate) root: Xid,
    pub(crate) available_xids: Vec<Xid>,
    
    pub(crate) listeners: std::collections::HashMap<String, Vec<(Option<String>, String)>>, // TODO?: HashMap<String, Vec<Option<dernok_parsing::Selector>, String>>
    pub(crate) handlers: std::collections::HashMap<String, std::sync::Arc<std::sync::Mutex<dyn Fn(Event) + Send + Sync + 'static>>>,
    pub(crate) indicators: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) requirements: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) event_queue: Vec<Event>,
    pub(crate) event_queue_being_cleared: bool,
    pub(crate) halted_events: std::collections::HashSet<Xid>,
    
    // styling
    pub(crate) animations: Vec<(Xid, Animation)>,
    pub(crate) stylesheet: std::collections::HashMap<String, Style>,
}

impl Engine {
    pub(crate) fn xid() -> Xid {
        dom!().available_xids.pop().unwrap_or(dom!().nodes.len())
    }
    
    pub(crate) fn is_within(xid: Xid, point: lyon::math::Point) -> bool {
        // let inv_pos = self.model().inverse().transform_point3(Vec3::new(pos.x, pos.y, 0.0));
        dom!()
            .nodes
            .get(&xid)
            .map(|node| {
                let layout = node.layout();
                // TODO: account for border radius
                (point.x > layout.location.x.into())
                    && (point.x < (layout.location.x + layout.size.width).into())
                    && (point.y > layout.location.y.into())
                    && (point.y < (layout.location.y + layout.size.height).into())
            })
            .unwrap_or_default()
    }
    
    pub(crate) fn set_scroll_offset(&mut self, xid: Xid, scroll_offset: lyon::math::Point) {
        self.nodes.get_mut(&xid).map(|node| {
            node.scroll_offset.x -= 10.0 * scroll_offset.x;
            node.scroll_offset.y -= 10.0 * scroll_offset.y;
        });
        // self.refresh_layout(&self.gfx);
    }
    
    pub(crate) fn refresh_layout() {
        let size = dom!().state.window_size;
        
        let root = dom!().nodes.get(&dom!().root).unwrap().layout_id;
        
        dom!().layout.set_style(
            root,
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::prelude::Dimension::Points(size.width as f32),
                    height: taffy::prelude::Dimension::Points(size.height as f32),
                },
                ..Default::default() // ..self.layout.get_style(0).cloned().unwrap()
            },
        );
        
        dom!().layout.compute_layout(
            root,
            taffy::geometry::Size {
                width: taffy::prelude::AvailableSpace::Definite(size.width as f32),
                height: taffy::prelude::AvailableSpace::Definite(size.height as f32),
            },
        )
        .unwrap();
    }
    
    pub(crate) fn emit(ty: EventTy, src: Xid) {
        // --- actions: specifiers ---
        // click: left, right, middle
        // mouse: move, enter, leave,
        // scroll: up, down
        // key: press, down, up
        // focus: in, out
        // animation: start, end, repeat
        // drag: start, end
        // window: resize, fullscreen
        // change: $
        // double click
        // any, custom, regular
        // --- requirements ---
        // ctrl, alt, shift, super
        // outside, this
        // long
        // --- indicators ---
        // bubble, trickle, proximity,
        // default, halt, prevent
        
        let state = dom!().state;
        
        let mut event = Event {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            ty,
            state: state.clone(),
            prev: state.clone(),
            target: Element(src),
            src: Element(src)
        };
        
        // println!("@@@: {}, {}, {}", &event.ty, &event.timestamp, &info);
        
        let mut current = vec![];
        let mut nxt = vec![];
        
        match &event.ty {
            EventTy::Animation(e) => {
                // start, end, repeat
            },
            
            EventTy::Change(_) => {
                current.push(event.clone());
            },
            
            EventTy::Click(_) => {
                // TODO: context menu?
                let nxt_focused = dom!().state.hovered.0;
                
                event.state.focused.0 = nxt_focused;
                dom!().event_queue.push(event.clone());
                
                let new_focused = nxt_focused != event.prev.focused.0; //TODO: TAB FOCUS
                if new_focused {
                    dom!().state.focused.0 = nxt_focused;
                    current.push(Event {
                        ty: EventTy::Focus(FocusEvent::Out),
                        target: event.prev.focused.clone(),
                        src: event.prev.focused.clone(),
                        ..event.clone()
                    });
                    current.push(Event {
                        ty: EventTy::Focus(FocusEvent::In),
                        ..event.clone()
                    });
                }
            },
            
            EventTy::DoubleClick(e) => {
                
            },
            
            EventTy::Drag(e) => {
                // start, end
            },
            
            EventTy::Focus(e) => {
                
            },
            
            EventTy::Key(_) => {
                // press, down, up
                // TODO: special built-in behaviours like inspector, find, fullscreen, etc.?
                current.push(event.clone());
            },
            
            EventTy::Mouse(e) => {
                // dom!().state.mouse_pos = e;
                let nxt_hovered =
                    dom!()
                        .nodes
                        .values()
                        .filter(|n| {
                                Self::is_within(n.xid, dom!().state.mouse_pos)
                                && !n.hidden
                        })
                        .reduce(|acc, item| { if acc.style.z > item.style.z { acc } else { item } })
                        .map(|n| n.xid)
                        .unwrap_or_default();
                    
                event.state.hovered.0 = nxt_hovered;
                event.target.0 = nxt_hovered;
                event.src.0 = nxt_hovered;
                
                current.push(event.clone());
                
                let new_hovered = nxt_hovered != event.prev.hovered.0;
                if new_hovered {
                    dom!().state.hovered.0 = nxt_hovered;
                    current.push(Event {
                        ty: EventTy::Mouse(MouseEvent::Leave),
                        target: event.prev.hovered.clone(),
                        src: event.prev.hovered.clone(),
                        ..event.clone()
                    });
                    current.push(Event {
                        ty: EventTy::Mouse(MouseEvent::Enter),
                        ..event.clone()
                    });
                }
            },
            
            EventTy::Scroll(e) => {
                
            },
            
            EventTy::Window(e) => {
                // resize, fullscreen
            },
            
            EventTy::Custom(_) | EventTy::Any(_) => {
                current.push(event.clone())
            }
        }
        
        while !current.is_empty() {
            for mut event in current {
                dom!().event_queue.push(event.clone());
                // if let Some(parent) = this.borrow().parents.get(&event.target.uuid) {
                //     if *parent != event.target.uuid {
                //         event.target.uuid = *parent;
                //         nxt.push(event);
                //     }
                // }
            }
            current = nxt;
            nxt = vec![];
        }
        
        if !dom!().event_queue_being_cleared {
            dom!().event_queue_being_cleared = true;
            
            let mut queue = dom!().event_queue.drain(..).collect::<Vec<_>>();
            
            while !queue.is_empty() {
                for event in queue {
                    // if dom!().halted_events.contains(&event.id) {
                    //     continue;TODO
                    // }
                    
                    // let num_dots = event.ty.matches('.').count();
                    // (1..=(1 + num_dots))
                    //     .filter_map(|n| event.ty.rsplitn(n, '.').last())
                    //     .flat_map(|listener| {
                    //         DOM
                    //             .get()
                    //             .unwrap()
                    //             .lock()
                    //             .unwrap()
                    //             .listeners
                    //             .get(listener)
                    //             .into_iter()
                    //             .flatten()
                    //             .filter_map(|(selector, handler)| {
                    //                 dom!().handlers.get(handler).cloned()
                    //             }) //TODO: filter out event.target not in dom.select(selector)
                    //             .collect::<Vec<_>>()
                    //     })
                    //     .for_each(|fx| fx(event.clone()));
                }
                
                queue =
                    DOM
                        .get()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .event_queue
                        .drain(..)
                        .collect();
            }
            
            dom!().halted_events.clear();
            
            dom!().event_queue_being_cleared = false;
        }
    }
    
    pub(crate) fn select(selectors: &str) -> Vec<Element> {
        let dom = dom!();
        
        let selectors = selectors
            .split(',')
            .map(|s| Selectors::new(s))
            .collect::<Vec<_>>();
        
        let mut res = std::collections::HashSet::new();
        
        for requirement in selectors {
            let mut matching = dom.nodes.keys().map(|uuid| Element(*uuid)).collect::<Vec<_>>();
            
            for (selector, link) in requirement.iter() {
                matching.retain(|el| {
                    dom.nodes.get(&el.0).map(|node| {
                        if let Some(tag) = &selector.tag {
                            if !node.tag.eq(tag.to_string().as_str()) { return false; }
                        }
                        
                        if let Some(id) = &selector.id {
                            if !node.id.as_ref().map(|nid| nid == id.to_string().as_str()).unwrap_or_default() { return false; }
                        }
                        
                        if !selector.classes.iter().all(|class| node.classes.contains(class.to_string().as_str())) { return false; }
                        
                        for attr_rule in &selector.attribute_rules {
                            match &attr_rule.ty {
                                SelectorAttributeRuleType::Exists => {
                                    if !node.attributes.contains_key(attr_rule.attr.to_string().as_str()) { return false; }
                                },
                                ty => {
                                    let value = attr_rule.value.unwrap();
                                    let valid = node.attributes.get(attr_rule.attr.to_string().as_str()).map(|v| {
                                        let miniserde::json::Value::String(v) = v else { return false };
                                        match ty {
                                            SelectorAttributeRuleType::Equals => {
                                                v.eq(value)
                                            },
                                            SelectorAttributeRuleType::NotEquals => {
                                                v.ne(value)
                                            },
                                            SelectorAttributeRuleType::StartsWith => {
                                                v.starts_with(value)
                                            },
                                            SelectorAttributeRuleType::Contains => {
                                                v.contains(value)
                                            },
                                            SelectorAttributeRuleType::EndsWith => {
                                                v.ends_with(value)
                                            },
                                            _ => unreachable!()
                                        }
                                    }).unwrap_or_default();
                                    if !valid { return false; }
                                }
                            }
                        }
                        true
                    }).unwrap_or_default()
                });
                
                if let Some(link) = link {
                    if matches!(
                        link,
                        SelectorLink::Parent
                            | SelectorLink::NextSibling
                            | SelectorLink::PrevSibling
                    ) {
                        matching = matching
                            .into_iter()
                            .flat_map(|el| match link {
                                SelectorLink::Parent => el.parent(),
                                SelectorLink::NextSibling => el.next_sibling(),
                                SelectorLink::PrevSibling => el.prev_sibling(),
                                _ => unreachable!(),
                            })
                            .collect();
                    } else {
                        matching = matching
                            .into_iter()
                            .flat_map(|el| match link {
                                SelectorLink::Ancestors => el.ancestors(),
                                SelectorLink::Descendants => el.descendants(),
                                SelectorLink::Children => el.children(),
                                SelectorLink::NextSiblings => el.next_siblings(),
                                SelectorLink::PrevSiblings => el.prev_siblings(),
                                SelectorLink::Siblings => el.siblings(),
                                _ => unreachable!(),
                            })
                            .collect();
                    }
                }
            }
            
            res.extend(matching);
        }
        
        res.into_iter().collect()
    }
}
