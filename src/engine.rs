use crate::{
    Value,
    Xid,
    node::Node,
    element::Element,
    animations::Animation,
    events::*,
    style::Style,
    selectors::{Selector, Op, Link},
};

#[derive(Clone, Copy, Default)]
pub(crate) enum Modifiers {
    #[default]
    Empty = 0b0000,
    Control = 0b0001,
    Alt = 0b0010,
    Shift = 0b0100,
    Super = 0b1000
}
pub(crate) type ModifiersState = u8;

#[derive(Copy, Clone, Default)]
pub(crate) struct State {
    pub(crate) mouse_position: lyon::math::Point,
    pub(crate) hovered: Xid,
    pub(crate) focused: Xid,
    pub(crate) window_size: lyon::math::Size,
    pub(crate) modifiers: ModifiersState,
}
impl State {
    pub fn mouse_position(&self) -> lyon::math::Point {
        self.mouse_position
    }
    pub fn hovered(&self) -> Element {
        Element(self.hovered)
    }
    pub fn focused(&self) -> Element {
        Element(self.focused)
    }
    pub fn control_key_pressed(&self) -> bool {
        self.modifiers & Modifiers::Control as u8 != 0
    }
    pub fn alt_key_pressed(&self) -> bool {
        self.modifiers & Modifiers::Alt as u8 != 0
    }
    pub fn shift_key_pressed(&self) -> bool {
        self.modifiers & Modifiers::Shift as u8 != 0
    }
    pub fn super_key_pressed(&self) -> bool {
        self.modifiers & Modifiers::Super as u8 != 0
    }
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
pub(crate) struct Engine<'a> {
    pub(crate) state: State,
    pub(crate) layout: taffy::Taffy,
    pub(crate) nodes: std::collections::HashMap<Xid, Node>,
    pub(crate) root: Xid,
    pub(crate) available_xids: Vec<Xid>,
    
    // pub(crate) listeners: std::collections::HashMap<String, Vec<(Option<Selector<'a>>, String)>>,
    pub(crate) listeners: std::collections::HashMap<String, Vec<(Option<String>, String)>>,
    pub(crate) handlers: std::collections::HashMap<String, std::sync::Arc<std::sync::Mutex<dyn Fn(Event) + Send + Sync + 'static>>>,
    pub(crate) indicators: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) requirements: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) event_queue: Vec<Event>,
    pub(crate) event_queue_being_cleared: bool,
    pub(crate) halted_events: std::collections::HashSet<Xid>,
    
    // styling
    pub(crate) animations: Vec<(Xid, Animation)>,
    pub(crate) stylesheet: std::collections::HashMap<Selector<'a>, Style>,
}

impl Engine<'_> {
    pub(crate) fn xid() -> Xid {
        dom!().available_xids.pop().unwrap_or(dom!().nodes.len() as Xid)
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
    
    pub(crate) fn emit(ty: EventTy, src: Xid, extra: Value) {
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
            src: Element(src),
            extra
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
                let nxt_focused = dom!().state.hovered;
                
                event.state.focused = nxt_focused;
                dom!().event_queue.push(event.clone());
                
                let new_focused = nxt_focused != event.prev.focused; //TODO: TAB FOCUS
                if new_focused {
                    dom!().state.focused = nxt_focused;
                    current.push(Event {
                        ty: EventTy::Focus(FocusEvent::Out),
                        target: Element(event.prev.focused),
                        src: Element(event.prev.focused),
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
                // dom!().state.mouse_position = e;
                let nxt_hovered =
                    dom!()
                        .nodes
                        .values()
                        .filter(|n| {
                                Self::is_within(n.xid, dom!().state.mouse_position)
                                && !n.hidden
                        })
                        .reduce(|acc, item| { if acc.style.z > item.style.z { acc } else { item } })
                        .map(|n| n.xid)
                        .unwrap_or_default();
                    
                event.state.hovered = nxt_hovered;
                event.target = Element(nxt_hovered);
                event.src = Element(nxt_hovered);
                
                current.push(event.clone());
                
                let new_hovered = nxt_hovered != event.prev.hovered;
                if new_hovered {
                    dom!().state.hovered = nxt_hovered;
                    current.push(Event {
                        ty: EventTy::Mouse(MouseEvent::Leave),
                        target: Element(event.prev.hovered),
                        src: Element(event.prev.hovered),
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
                // if let Some(parent) = this.borrow().parents.get(&event.target.0) {
                //     if *parent != event.target.0 {
                //         event.target.0 = *parent;
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
        
        let selectors =
            selectors
                .split(',')
                .filter_map(|s| Selector::parse(s))
                .collect::<Vec<_>>();
        
        let mut res = std::collections::HashSet::new();
        
        for selector in selectors {
            let mut matching = dom.nodes.keys().map(|xid| Element(*xid)).collect::<Vec<_>>();
            
            for (rule, link) in selector.x {
                println!("{:?} -- {:?} -- {:?}", rule.tag.as_ref().map(|x| x.0), rule.xid.as_ref().map(|x| x.0), rule.id.as_ref().map(|x| x.0));
                matching.retain(|el| {
                    let node = &dom.nodes[&el.0];
                    
                    if let Some(tag) = &rule.tag {
                        if node.tag != tag.0 {
                            return false;
                        }
                    }
                    
                    if let Some(xid) = &rule.xid {
                        if node.xid.to_string() != xid.0 {
                            return false;
                        }
                    }
                    
                    if let Some(id) = &rule.id {
                        if let Some(nid) = &node.id {
                            if nid != id.0 {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    
                    if !rule.classes.iter().all(|class| node.classes.contains(class.0)) {
                        return false;
                    }
                    
                    for attr in &rule.attributes {
                        match &attr.op {
                            Op::Exists => {
                                if !node.attributes.contains_key(attr.name.0) {
                                    return false;
                                }
                            },
                            op => {
                                let Some(value) = attr.value else {
                                    return false;
                                };
                                
                                if let Some(nvalue) = node.attributes.get(attr.name.0) {
                                    let v = serde_json::to_string(nvalue).unwrap();
                                    return {
                                        match op {
                                            Op::Equals => v == value,
                                            Op::NotEquals => v != value,
                                            Op::StartsWith => v.starts_with(value),
                                            Op::Contains => v.contains(value),
                                            Op::EndsWith => v.ends_with(value),
                                            _ => false
                                        }
                                    };
                                } else {
                                    return false;
                                }
                            }
                        }
                    }
                    
                    true
                });
                
                if let Some(link) = link {
                    match link {
                        Link::Parent | Link::NextSibling | Link::PrevSibling => {
                            matching =
                                matching
                                    .into_iter()
                                    .filter_map(|el| match link {
                                        Link::Parent => el.parent(),
                                        Link::NextSibling => el.next_sibling(),
                                        Link::PrevSibling => el.prev_sibling(),
                                        _ => None
                                    })
                                    .collect();
                        },
                        _ => {
                            matching =
                                matching
                                    .into_iter()
                                    .flat_map(|el| match link {
                                        Link::Ancestors => el.ancestors(),
                                        Link::Descendants => el.descendants(),
                                        Link::Children => el.children(),
                                        Link::NextSiblings => el.next_siblings(),
                                        Link::PrevSiblings => el.prev_siblings(),
                                        Link::Siblings => el.siblings(),
                                        _ => vec![]
                                    })
                                    .collect();
                        }
                    }
                }
            }
            
            res.extend(matching);
        }
        
        res.into_iter().collect()
    }
}
