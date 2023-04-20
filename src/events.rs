use crate::{
    engine::{DOM, Engine, State},
    animations::Animation,
    element::Element
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
    pub timestamp: u128,
    pub ty: EventTy,
    pub(crate) state: State,
    pub(crate) prev: State,
    pub target: Element,
    pub src: Element
}
impl Event {
    pub fn halt(&self) {
        // dom!().halted_events.insert(self.id.clone());
    }
}

pub struct Handler(pub(crate) String);

impl From<&str> for Handler {
    fn from(value: &str) -> Self {
        // if let Some(handler) = dom!().
        Self(value.into())
    }
}

impl<T: Fn(Event) + Send + Sync + 'static>  From<T> for Handler {
    fn from(value: T) -> Self {
        let name = Engine::xid().to_string();
        dom!().handlers.insert(name.clone(), std::sync::Arc::new(std::sync::Mutex::new(value)));
        Self(name)
    }
}

#[cfg(feature = "wust")]
impl From<wust::Script> for Handler {
    fn from(value: wust::Script) -> Self {
        
        Self()
    }
}
