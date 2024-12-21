use crate::{
    Json, InstanceId, Xid,
    animations::Animation,
    element::Element,
    events::*,
    macros::instance,
    node::{Node, Vertex},
    selectors::{Selector, Op, Link},
    style::Style,
    text::TextResources
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
    iid: InstanceId,
    pub(crate) mouse_position: lyon::math::Point,
    pub(crate) hovered: Xid,
    pub(crate) focused: Xid,
    pub(crate) window_size: lyon::math::Size,
    pub(crate) modifiers: ModifiersState
}
impl State {
    pub fn mouse_position(&self) -> lyon::math::Point {
        self.mouse_position
    }
    pub fn hovered(&self) -> Element {
        Element { xid: self.hovered, iid: self.iid }
    }
    pub fn focused(&self) -> Element {
        Element {xid: self.focused, iid: self.iid }
    }
    pub fn control_key_pressed(&self) -> bool {
        (self.modifiers & (Modifiers::Control as u8)) != 0
    }
    pub fn alt_key_pressed(&self) -> bool {
        (self.modifiers & (Modifiers::Alt as u8)) != 0
    }
    pub fn shift_key_pressed(&self) -> bool {
        (self.modifiers & (Modifiers::Shift as u8)) != 0
    }
    pub fn super_key_pressed(&self) -> bool {
        (self.modifiers & (Modifiers::Super as u8)) != 0
    }
}

#[derive(Default)]
pub(crate) struct Engine<'a> {
    pub(crate) state: State,
    pub(crate) text_resources: TextResources,
    pub(crate) layout: taffy::Taffy,
    pub(crate) nodes: std::collections::HashMap<Xid, Node>,
    pub(crate) root: Option<Xid>,
    
    pub(crate) components: std::collections::HashMap<String, fn() -> Element>,
    pub(crate) listeners: std::collections::HashMap<String, Vec<(Option<Selector<'a>>, String)>>,
    pub(crate) handlers: std::collections::HashMap<String, std::sync::Arc<std::sync::Mutex<dyn Fn(Event) + Send + Sync + 'static>>>,
    pub(crate) indicators: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) requirements: std::collections::HashMap<String, std::collections::HashSet<String>>,
    pub(crate) event_queue: Vec<Event>,
    pub(crate) event_queue_being_cleared: bool,
    pub(crate) halted_events: std::collections::HashSet<Xid>,
    
    pub(crate) animations: Vec<(Xid, Animation)>,
    pub(crate) stylesheet: Vec<(Selector<'a>, Style)>
}

impl Engine<'_> {
    pub(crate) fn new_instance(iid: InstanceId, width: f32, height: f32) -> InstanceId {
        crate::ENGINES.get_or_init(|| Default::default()).insert(iid, Engine::default());
        instance!(iid).state.iid = iid;
        instance!(iid).state.window_size = lyon::math::size(width, height);
        iid
    }
    
    pub(crate) fn xid(iid: InstanceId) -> Xid {
        (0..Xid::MAX).find(|xid| !instance!(iid).nodes.contains_key(xid)).unwrap_or_default()
    }
    
    pub(crate) fn set_scroll_offset(iid: InstanceId, xid: Xid, scroll_offset: lyon::math::Point) {
        instance!(iid).nodes.get_mut(&xid).map(|node| {
            node.scroll_offset.x -= 10.0 * scroll_offset.x;
            node.scroll_offset.y -= 10.0 * scroll_offset.y;
        });
        // self.refresh_layout(&self.gfx);
    }
    
    pub(crate) fn emit(iid: InstanceId, ty: EventTy, src: Xid, extra: Json) {
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
        
        let state = instance!(iid).state;
        
        let mut event = Event {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            ty,
            state: state.clone(),
            prev: state.clone(),
            target: Element { xid: src, iid },
            src: Element { xid: src, iid },
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
                let nxt_focused = instance!(iid).state.hovered;
                
                event.state.focused = nxt_focused;
                instance!(iid).event_queue.push(event.clone());
                
                let new_focused = nxt_focused != event.prev.focused; //TODO: TAB FOCUS
                if new_focused {
                    instance!(iid).state.focused = nxt_focused;
                    current.push(Event {
                        ty: EventTy::Focus(FocusEvent::Out),
                        target: Element { xid: event.prev.focused, iid },
                        src: Element { xid: event.prev.focused, iid },
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
                // instance!(id).state.mouse_position = e;
                let nxt_hovered =
                    instance!(iid)
                        .nodes
                        .values()
                        .filter(|n| {
                                n.is_within(instance!(iid).state.mouse_position)
                                && !n.hidden
                        })
                        .reduce(|acc, item| { if acc.style.z > item.style.z { acc } else { item } })
                        .map(|n| n.xid)
                        .unwrap_or_default();
                    
                event.state.hovered = nxt_hovered;
                event.target = Element { xid: nxt_hovered, iid };
                event.src = Element { xid: nxt_hovered, iid };
                
                current.push(event.clone());
                
                let new_hovered = nxt_hovered != event.prev.hovered;
                if new_hovered {
                    instance!(iid).state.hovered = nxt_hovered;
                    current.push(Event {
                        ty: EventTy::Mouse(MouseEvent::Leave),
                        target: Element { xid: event.prev.hovered, iid },
                        src: Element { xid: event.prev.hovered, iid},
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
                instance!(iid).event_queue.push(event.clone());
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
        
        if !instance!(iid).event_queue_being_cleared {
            instance!(iid).event_queue_being_cleared = true;
            
            let mut queue = instance!(iid).event_queue.drain(..).collect::<Vec<_>>();
            
            while !queue.is_empty() {
                for event in queue {
                    // if instance!(id).halted_events.contains(&event.id) {
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
                    //                 instance!(id).handlers.get(handler).cloned()
                    //             }) //TODO: filter out event.target not in dom.select(selector)
                    //             .collect::<Vec<_>>()
                    //     })
                    //     .for_each(|fx| fx(event.clone()));
                }
                
                queue = instance!(iid).event_queue.drain(..).collect();
            }
            
            instance!(iid).halted_events.clear();
            
            instance!(iid).event_queue_being_cleared = false;
        }
    }
    
    pub(crate) fn select(iid: InstanceId, selectors: &str) -> Vec<Element> {
        let mut res = std::collections::HashSet::new();
        
        for selector in {
            selectors
                .split(',')
                .filter_map(|s| Selector::parse(std::borrow::Cow::Borrowed(s)))
        } {
            let mut matching =
                instance!(iid)
                    .nodes
                    .keys()
                    .map(|xid| Element { xid: *xid, iid })
                    .collect::<Vec<_>>();
            
            for (rule, link) in &selector.rules {
                matching.retain(|el| {
                    let node = &instance!(iid).nodes[&el.xid];
                    
                    if let Some(tag) = &rule.tag {
                        if node.tag != selector.get(tag.0) {
                            return false;
                        }
                    }
                    
                    if let Some(xid) = &rule.xid {
                        if node.xid.to_string() != selector.get(xid.0) {
                            return false;
                        }
                    }
                    
                    if let Some(id) = &rule.id {
                        if let Some(nid) = &node.id {
                            if nid != selector.get(id.0) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    
                    if !rule.classes.iter().all(|class| node.classes.contains(selector.get(class.0))) {
                        return false;
                    }
                    
                    for attr in &rule.attributes {
                        match &attr.op {
                            Op::Exists => {
                                if !node.attributes.contains_key(selector.get(attr.name.0)) {
                                    return false;
                                }
                            },
                            op => {
                                let Some(value) = attr.value else {
                                    return false;
                                };
                                
                                if let Some(nvalue) = node.attributes.get(selector.get(attr.name.0)) {
                                    let v = serde_json::to_string(nvalue).unwrap().replace('"', "");
                                    let valid = match op {
                                        Op::Equals => v == selector.get(value),
                                        Op::NotEquals => v != selector.get(value),
                                        Op::StartsWith => v.starts_with(selector.get(value)),
                                        Op::Contains => v.contains(selector.get(value)),
                                        Op::EndsWith => v.ends_with(selector.get(value)),
                                        _ => false
                                    };
                                    if !valid {
                                        return false;
                                    }
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
    
    pub(crate) fn render(iid: InstanceId, root: Xid) -> lyon::tessellation::VertexBuffers<Vertex, u16> {
        if !instance!(iid).nodes.contains_key(&root) {
            return lyon::tessellation::VertexBuffers::new();
        }
        
        instance!(iid).root = Some(root);
        let root_layout_id = instance!(iid).nodes[&root].layout_id;
        
        let size = instance!(iid).state.window_size;
        instance!(iid).layout.compute_layout(
            root_layout_id,
            taffy::prelude::Size {
                width: taffy::prelude::AvailableSpace::Definite(size.width),
                height: taffy::prelude::AvailableSpace::Definite(size.height)
            }
        );
        
        let mut buffers = lyon::tessellation::VertexBuffers::new();
        for node in instance!(iid).nodes.values() {
            let buffer = node.render();
            buffers.indices.extend(buffer.indices);
            buffers.vertices.extend(buffer.vertices);
        }
        buffers
    }
}
