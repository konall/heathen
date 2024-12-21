use crate::{
    Json,
    animations::Animation,
    element::Element,
    engine::{Engine, State},
    macros::instance
};

#[derive(Clone)]
pub enum MouseEvent {
    Move(lyon::math::Point),
    Enter,
    Leave
}

#[derive(Clone)]
pub enum ScrollEvent {
    Up,
    Down
}

#[derive(Clone)]
pub enum ClickEvent {
    Left,
    Right,
    Middle,
    Other(usize)
}

#[derive(Clone)]
pub enum DoubleClickEvent {
    Left,
    Right,
    Middle,
    Other(usize)
}

#[derive(Clone)]
pub enum KeyEvent {
    Down(String),
    Up(String),
    Press(String)
}

#[derive(Clone)]
pub enum AnimationEvent {
    Start(Animation),
    End(Animation),
    Repeat(Animation)
}

#[derive(Clone)]
pub enum DragEvent {
    Start,
    End
}

#[derive(Clone)]
pub enum WindowEvent {
    Resize,
    Fullscreen
}

#[derive(Clone)]
pub enum FocusEvent {
    In,
    Out
}

#[derive(Clone)]
pub struct ChangeEvent {
    pub attribute: String,
    pub from: Json,
    pub to: Json
}

#[derive(Clone)]
pub enum EventTy {
    Mouse(MouseEvent),
    Scroll(ScrollEvent),
    Click(ClickEvent),
    Key(KeyEvent),  // TODO: special built-in behaviours like inspector, find, fullscreen, etc.?
    Animation(AnimationEvent),
    Drag(DragEvent),
    Window(WindowEvent),
    Focus(FocusEvent),
    Change(ChangeEvent),
    DoubleClick(DoubleClickEvent),
    Custom(String),
    Any(Box<EventTy>)
}


#[derive(Clone)]
pub struct Event {
    pub(crate) timestamp: u128,
    pub(crate) ty: EventTy,
    pub(crate) state: State,
    pub(crate) prev: State,
    pub(crate) target: Element,
    pub(crate) src: Element,
    pub(crate) extra: Json
}
impl Event {
    pub fn new(ty: EventTy, src: Element, extra: Json) -> Event {
        Event {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            ty,
            state: instance!(0).state.clone(),
            prev: instance!(0).state.clone(),
            target: Element { xid: 0, iid: 0 },
            src,
            extra
        }
    }
    pub fn halt(&self) {
        // instance!(id).halted_events.insert(self.id.clone());
    }
}

pub struct Handler(pub(crate) String);

impl From<&str> for Handler {
    fn from(value: &str) -> Self {
        // if let Some(handler) = instance!(id).
        Self(value.into())
    }
}

impl<T: Fn(Event) + Send + Sync + 'static>  From<T> for Handler {
    fn from(value: T) -> Self {
        let name = Engine::xid(1).to_string();
        instance!(1).handlers.insert(name.clone(), std::sync::Arc::new(std::sync::Mutex::new(value)));
        Self(name)
    }
}

#[cfg(feature = "wust")]
impl From<wust::Script> for Handler {
    fn from(value: wust::Script) -> Self {
        
        Self()
    }
}
