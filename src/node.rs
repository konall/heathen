use crate::{
    Value,
    Xid,
    engine::DOM,
    style::Style,
};

#[derive(Default)]
pub(crate) struct Node {
    pub(crate) xid: Xid,
    pub(crate) layout_id: taffy::node::Node,
    
    pub(crate) scroll_offset: lyon::math::Point,
    pub(crate) hidden: bool,
    
    pub(crate) tag: String,
    pub(crate) id: Option<String>,
    pub(crate) classes: std::collections::HashSet<String>,
    pub(crate) text: Option<String>,
    
    pub(crate) style: Style,
    pub(crate) attributes: std::collections::HashMap<String, Value>,
    // pub(crate) listeners: Vec<Listener>,
    // pub accessibility: accesskit::Node,
    
    pub(crate) parent: Option<Xid>,
    pub(crate) children: Vec<Xid>
}


pub struct Vertex {
    pub position: glam::Vec3,
    pub colour: colorgrad::Color
}


impl Node {
    pub(crate) fn layout(&self) -> taffy::layout::Layout {
        dom!().layout.layout(self.layout_id).copied().unwrap()
    }
    
    // pub(crate) fn geometry(&self) -> lyon::tessellation::VertexBuffers<Vertex, u16> {
    //     let mut geometry = lyon::tessellation::VertexBuffers::new();
    //     let layout = self.layout();
        
    //     let mut geom_builder = lyon::tessellation::geometry_builder::simple_builder(&mut geometry);
    //     let mut tesselator = lyon::tessellation::FillTessellator::new();
    //     let options = lyon::tessellation::FillOptions::tolerance(0.001);
    //     let mut builder = tesselator.builder(&options, &mut geom_builder);
        
    //     builder.add_rounded_rectangle(
    //         &lyon::math::Box2D::from_origin_and_size(
    //             lyon::math::point(
    //                 layout.location.x - self.scroll_offset.x,
    //                 layout.location.y - self.scroll_offset.y,
    //             ),
    //             lyon::math::size(layout.size.width, layout.size.height),
    //         ),
    //         &lyon::path::builder::BorderRadii {
    //             top_left: self.style.border_radii.top_left,
    //             bottom_left: self.style.border_radii.bottom_left,
    //             top_right: self.style.border_radii.top_right,
    //             bottom_right: self.style.border_radii.bottom_right,
    //         },
    //         lyon::path::Winding::Positive,
    //     );
            
    //     builder.build().unwrap();
        
    //     geometry
    // }
}
