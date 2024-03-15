#[macro_use]
extern crate glium;
extern crate winit;

mod models;
use models::*;

use glium::Surface;


fn main() {
    let event_loop = 
        winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    let (_window, display) =
        glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Physics Simulation")
        .build(&event_loop);
    

    event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::CursorMoved { position, .. } => { 
                    println!("Mouse position: {:?}x{:?}", position.x as u16, position.y as u16);
                },
                _ => (),
            },
            _ => (),
        };
    })
    .unwrap();
}

