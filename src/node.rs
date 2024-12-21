use crate::{
    Json, Xid,
    macros::instance,
    style::Style
};

#[derive(Default)]
pub(crate) struct Node {
    pub(crate) xid: Xid,
    pub(crate) layout_id: taffy::node::Node,
    
    pub(crate) parent: Option<Xid>,
    pub(crate) children: Vec<Xid>,
    
    pub(crate) scroll_offset: lyon::math::Point,
    pub(crate) hidden: bool,
    
    pub(crate) tag: String,
    pub(crate) id: Option<String>,
    pub(crate) classes: std::collections::HashSet<String>,
    pub(crate) text: Option<String>,
    
    pub(crate) attributes: std::collections::HashMap<String, Json>,
    // pub(crate) listeners: Vec<Listener>,
    // pub accessibility: accesskit::Node,
    
    pub(crate) style: Style,
    pub(crate) style_score: std::collections::HashMap<String, usize>
}

pub struct Vertex {
    pub position: [f32; 3],
    pub colour: [u8; 4]
}

impl Node {
    pub(crate) fn is_within(&self, point: lyon::math::Point) -> bool {
        // let inv_pos = self.model().inverse().transform_point3(Vec3::new(pos.x, pos.y, 0.0));
        let layout = instance!(1).layout.layout(self.layout_id).copied().unwrap();
        // TODO: account for border radius
        (point.x > layout.location.x.into())
        && (point.x < (layout.location.x + layout.size.width).into())
        && (point.y > layout.location.y.into())
        && (point.y < (layout.location.y + layout.size.height).into())
    }
    
    pub(crate) fn render(&self) -> lyon::tessellation::VertexBuffers<Vertex, u16> {
        let layout = instance!(1).layout.layout(self.layout_id).copied().unwrap();
        
        let mut geometry = lyon::tessellation::VertexBuffers::new();
        
        {
            let mut geom_builder = lyon::tessellation::geometry_builder::simple_builder(&mut geometry);
            let mut tesselator = lyon::tessellation::FillTessellator::new();
            let options = lyon::tessellation::FillOptions::tolerance(0.001);
            let mut builder = tesselator.builder(&options, &mut geom_builder);
            
            builder.add_rounded_rectangle(
                &lyon::math::Box2D::from_origin_and_size(
                    lyon::math::point(
                        layout.location.x - self.scroll_offset.x,
                        layout.location.y - self.scroll_offset.y,
                    ),
                    lyon::math::size(layout.size.width, layout.size.height),
                ),
                &lyon::path::builder::BorderRadii {
                    top_left: self.style.border_radii.top_left,
                    bottom_left: self.style.border_radii.bottom_left,
                    top_right: self.style.border_radii.top_right,
                    bottom_right: self.style.border_radii.bottom_right,
                },
                lyon::path::Winding::Positive,
            );
            
            builder.build().unwrap();
        }
        
        let mut geometry = lyon::tessellation::geometry_builder::VertexBuffers {
            indices: geometry.indices,
            vertices:
                geometry.vertices
                    .into_iter()
                    .map(|v| Vertex {
                        colour: [0, 0, 0, 0],
                        position: [v.x, v.y, self.style.z]
                    })
                    .collect()
        };
        
        if let Some(text) = self.text.as_ref() {
            let font_system = &mut instance!(1).text_resources.font_system;
            let swash_cache = &mut instance!(1).text_resources.swash_cache;
            
            let mut buffer = cosmic_text::Buffer::new(
                font_system,
                cosmic_text::Metrics::new(self.style.text.size, 20.0)
            );
            buffer.set_size(font_system, layout.size.width, layout.size.height);
            buffer.set_text(
                font_system,
                text,
                cosmic_text::Attrs::new()
            );
            buffer.shape_until_scroll(font_system);
            
            let mut text_geometry = lyon::tessellation::VertexBuffers::new();
            
            {
                let mut geom_builder = lyon::tessellation::geometry_builder::simple_builder(&mut text_geometry);
                let mut tesselator = lyon::tessellation::FillTessellator::new();
                let options = lyon::tessellation::FillOptions::tolerance(0.001);
                let mut builder = tesselator.builder(&options, &mut geom_builder);
                
                buffer.draw(font_system, swash_cache, cosmic_text::Color::rgb(255, 255, 255), |x, y, w, h, colour| {
                    builder.add_rectangle(
                        &lyon::math::Box2D::from_origin_and_size(
                            lyon::math::point(
                                layout.location.x - self.scroll_offset.x + x as f32,
                                layout.location.y - self.scroll_offset.y + y as f32,
                            ),
                            lyon::math::size(w as f32, h as f32),
                        ),
                        lyon::path::Winding::Positive,
                    );
                });
                
                builder.build().unwrap();
            }
            
            geometry.indices.extend(text_geometry.indices.into_iter().map(|idx| idx + geometry.vertices.len() as u16));
            geometry.vertices.extend(text_geometry.vertices.into_iter().map(|v| Vertex {
                position: [v.x, v.y, self.style.z],
                colour: [255, 255, 255, 0]
            }));
        }
        
        geometry
    }
}
