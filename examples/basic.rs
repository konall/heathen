fn main() {
    let width = 640;
    let height = 320;
    
    let app = heathen::new(0, width as f32, height as f32);
    
    let el = app.create_element("h1", [("x", 3.into())], &[
        app.create_element("h2", [], &[]),
        app.create_element("h3", [], &[
            app.create_element("h4", [("a", "123".into())], &[])
        ])
    ]);
    
    el.set_id("demo");
    el.set_attribute("qwerty", "wow");
    el.set_attribute("abc", 2);
    // el.set_style(new)
    println!("{:?}", el.descendants().into_iter().map(|x| format!("{} -- {:?}", x.tag(), x.attributes())).collect::<Vec<_>>());
    println!("{:?}", app.select(r#"h1#demo[qwerty$=ow][abc^=3]"#).into_iter().map(|el| el.tag()).collect::<Vec<_>>());
    
    let app2 = heathen::new(0, 0 as f32, 0 as f32);
    let el2 = app2.create_element("p", [("zzz", "yyy".into())], &[]);
    println!("app2: {:?}", app.select(r#"h1#demo[qwerty$=ow][abc^=3]"#).into_iter().map(|el| el.tag()).collect::<Vec<_>>());
    println!("app2: {:?}", app2.select(r#"p"#).into_iter().map(|el| el.tag()).collect::<Vec<_>>());
    
    let mut window = minifb::Window::new("Test - ESC to exit", width, height, minifb::WindowOptions::default()).unwrap();
    
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let mut buffer = vec![0u32; width * height];
    
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if window.is_key_down(minifb::Key::X) {
            {
                let (vb, ib) = app.render(el);
                ib.into_iter().for_each(|idx| {
                    let vertex = &vb[idx as usize];
                    let [x, y, z] = vertex.position;
                    let pixel = &mut buffer[(32 * x as usize) + (32 * width * y as usize)];
                    
                    vertex.position[0];
                });
            }
        }
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
