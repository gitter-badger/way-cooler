extern crate rustwlc;

use rustwlc::*;
use rustwlc::types::*;
use rustwlc::handle::{WlcView, WlcOutput};
use rustwlc::interface::*;
use rustwlc::input::{pointer, keyboard};
use rustwlc::xkb::keysyms;

fn main() {
    let interface: WlcInterface = WlcInterface::new()
        .output_resolution(output_resolution)
        .view_created(view_created)
        .view_destroyed(view_destroyed)
        .view_focus(view_focus)
        .view_request_state(view_request_state)
        //.view_request_geometry(view_request_geometry)
        .keyboard_key(keyboard_key)
        .pointer_motion(pointer_motion)
        .compositor_ready(compositor_ready);

    rustwlc::log_set_default_handler();

    let run_wlc = rustwlc::init(interface).expect("Unable to initialize wlc!");
    run_wlc();
}

/// From example.c:
/// very simple layout function
/// you probably don't want to layout certain type of windows in wm
fn render_output(output: &WlcOutput) {
    let resolution = output.get_resolution();
    let views = output.get_views();

    for view in views {
        if view.get_type() == ViewType::empty() {
            view.set_geometry(EDGE_NONE,
                              &Geometry {
                                  size: Size {
                                      w: resolution.w as u32,
                                      h: resolution.h as u32
                                  },
                                  origin: Point {
                                      x: 0,
                                      y: 0
                                  }
                              });
        }
    }
}

fn get_topmost_view(output: &WlcOutput, offset: usize) -> Option<WlcView> {
    let views = output.get_views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()].clone())
    }
}

extern fn view_created(view: WlcView) -> bool {
    let output = view.get_output();
    view.set_mask(output.get_mask());
    view.bring_to_front();
    view.focus();
    render_output(&output);
    true
}

extern fn output_resolution(output: WlcOutput, _from: &Size, _to: &Size) {
    render_output(&output);
}

extern fn view_destroyed(view: WlcView) {
    if let Some(top) = get_topmost_view(&view.get_output(), 0) {
        top.focus();
    }
    render_output(&view.get_output());
}

extern fn view_focus(current: WlcView, focused: bool) {
    current.set_state(VIEW_ACTIVATED, focused);
}

extern fn view_request_state(view: WlcView, state: ViewState, handled: bool) {
    view.set_state(state, handled);
}

extern fn keyboard_key(view: WlcView, _time: u32, mods: &KeyboardModifiers,
                       key: u32, state: KeyState) -> bool {
    use std::process::Command;
    let sym = keyboard::get_keysym_for_key(key, &mods.mods);
    if state == KeyState::Pressed && mods.mods.contains(MOD_MOD4) {
        match sym {
            // Close
            keysyms::KEY_q => view.close(),

            keysyms::KEY_Down => {
                view.send_to_back();
                if let Some(top) =
                    get_topmost_view(&view.get_output(), 0) {
                    top.focus();
                }
            },

            //keysyms::KEY_Up =>

            keysyms::KEY_Escape => terminate(),

            keysyms::KEY_Return => {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg("/usr/bin/xfce4-terminal")
                    .spawn()
                    .unwrap_or_else(|e| {
                        println!("Eror spawning child: {}", e);
                        panic!("{}", e);
                    });
            },
            _ => {}
        }
        return true;
    }
    return false;
}

extern fn pointer_motion(_view: WlcView, _time: u32, point: &Point) -> bool {
    pointer::set_position(point);
    false
}

extern fn compositor_ready() {
    println!("Preparing compositor!");
}
