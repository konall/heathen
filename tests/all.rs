#[test]
fn fx() {
    let engine = wasmi::Engine::default();
    
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))
            (func (export "hello")
                (call $host_hello (i32.const 3))
            )
        )
    "#;
    let wasm = wat::parse_str(&wat).unwrap();
    
    let module = wasmi::Module::new(&engine, &mut &wasm[..]).unwrap();
    let mut store = wasmi::Store::new(&engine, ());
    
    let host_fx = wasmi::Func::wrap(&mut store, |param: i32| println!("hi! #{param}#"));
    
    let mut linker = wasmi::Linker::new(&engine);
    linker.define("host", "hello", host_fx).unwrap();
    
    let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();
    let fx = instance.get_func(&store, "hello").unwrap();
    fx.call(&mut store, &[], &mut []).unwrap();
    
    println!("{:?}", heathen::Document::root().attributes());
    
    heathen::Document::root().set_attribute("test", r#"{ "z": ["abc", 1, 2] }"#);
    
    println!("{:?}", heathen::Document::root().attributes());
    
    println!("{:?}", heathen::Document::select("#hello").into_iter().map(|el| format!("{} - {:?}", el.tag(), el.attributes())).collect::<Vec<_>>());
    
    
    
    // let width = 640;
    // let height = 320;
    
    // let mut buffer: Vec<u32> = vec![0; width * height];
    
    // let mut window = minifb::Window::new(
    //     "Test - ESC to exit",
    //     width,
    //     height,
    //     minifb::WindowOptions::default(),
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });
    
    // // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    // while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
    //     for i in buffer.iter_mut() {
    //         *i = 0; // write something more funny here!
    //     }
        
    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&buffer, width, height)
    //         .unwrap();
    // }
}
