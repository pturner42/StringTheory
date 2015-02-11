extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate rustnet;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::{Event, poll_event};
//use sdl2::event::poll_event;
//use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi;
use collections::vec;


mod object;
mod shader;
mod camera;
mod matrix;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    let window_x = 800;
    let window_y = 600;

    let port: u16 = 1231;

    let mut connected = true;

    if !rustnet::init_client("127.0.0.1", port) {
        println!("Unable to connect to server on port {}", port);
        connected = false;
    } else {
        println!("Connected!");
    }

    
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCompatibilityProfile as i32);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 2);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    let window = match Window::new("rust-sdl2: Video", WindowPos::PosCentered, WindowPos::PosCentered, window_x, window_y, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("faid to create window: {}", err)
    };

    // MUST ASSIGN RESULT THIS TO A VARIABLE
    // Otherwise, it gets deleted or is optimized out or something
    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });

    //unsafe { gl::Disable(gl::CULL_FACE); }
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }

    let obj = object::new(-0.5, -0.5, -1.5,    0.5, 0.5, -2.5,    0.8, 0.9, 0.4);
    let obj2 = object::new(0.5, 0.5, -1.5,     1.5, 1.5, -2.5,    1.0, 0.4, 0.2);
    //let floor = object::new(-0.5, -0.5, -1.5, 0.5, 0.5, -2.5,     0.4, 0.9, 0.4);
    let floor = object::new(-5.5, -4.5, 5.5,  5.5, -4.0, -5.5,     0.4, 0.9, 0.4);
    let aspect_ratio = window_x as f32 / window_y as f32;
    let mut camera = camera::new(60.0f32, aspect_ratio, 1.0f32, 100.0f32);
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut z = 0.0f32;
    //camera.view.setScale(1.0, 1.0, 1.0);

    let mut sent = false;

    loop {
        match poll_event() {
            Event::Quit{..} => break,
            Event::KeyDown{keycode: key, ..} => {
                if key == KeyCode::Escape { break; }
                if key == KeyCode::Up { z = -1.0f32; }
                if key == KeyCode::Down { z = 1.0f32; }
                if key == KeyCode::Z { y = -1.0f32; }
                if key == KeyCode::X { y = 1.0f32; }
                if key == KeyCode::Left { x = -1.0f32; }
                if key == KeyCode::Right { x = 1.0f32; }
                //if key == KeyCode::Up { z = z - 1.0f32; }
                //if key == KeyCode::Down { z = z + 1.0f32; }
                //if key == KeyCode::Z { y = y - 1.0f32; }
                //if key == KeyCode::X { y = y + 1.0f32; }
                //if key == KeyCode::Left { x = x + 1.0f32; }
                //if key == KeyCode::Right { x = x - 1.0f32; }
                if key == KeyCode::D { camera.horizontal_angle +=  -0.10f32; }
                if key == KeyCode::A { camera.horizontal_angle +=  0.10f32; }
                if key == KeyCode::S { camera.vertical_angle +=  -0.10f32; }
                if key == KeyCode::W { camera.vertical_angle +=  0.10f32; }
            }
            _ => {}
        }

        camera.translate(x, y, z);
        //camera.set_translation(x, y, z);
        camera.update_view_projection();
        x = 0.0f32;
        y = 0.0f32;
        z = 0.0f32;
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        obj.draw(&camera);
        obj2.draw(&camera);
        floor.draw(&camera);
        //obj3.draw(&camera);
        window.gl_swap_window();
        if connected {
            if rustnet::check_sockets(){
                if !rustnet::read_server_socket(can_handle, user_defined){
                    println!("Lost server connection.");
                    break;;
                }
            }

            if !sent {
                rustnet::clear_buffer();
                rustnet::write_byte(1);
                rustnet::write_byte(5);
                rustnet::send_ts_message();
                sent = true;
            }
        }
    }
    sdl2::quit();
}

fn user_defined(msg_id: u8) -> u32 {
    0
}

fn can_handle(msg_id: u8, buffer_size: u32) -> bool {
    true
}
