pub mod color;
mod game;
pub mod math;

use game::Game;
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, MouseScrollDelta, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorGrabMode, WindowBuilder},
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Game")
            .with_visible(false)
            .build(&event_loop)?,
    );

    let mut game = pollster::block_on(Game::new(window.clone()))?;

    let mut last_time = std::time::Instant::now();
    let mut dt = std::time::Duration::ZERO;
    let ts = std::time::Duration::from_secs(1) / 100;
    let mut fixed_time = std::time::Duration::ZERO;
    event_loop.run(move |event, elwt| match event {
        Event::NewEvents(cause) => {
            match cause {
                StartCause::Init => {
                    elwt.set_control_flow(ControlFlow::Poll);
                    window.set_visible(true);
                    window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
                    window.set_cursor_visible(false);
                    last_time = std::time::Instant::now();
                }
                StartCause::Poll => {}
                _ => {}
            };

            let time = std::time::Instant::now();
            dt = time - last_time;
            last_time = time;
        }

        Event::WindowEvent { window_id, event } if window_id == window.id() && !elwt.exiting() => {
            match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    elwt.exit();
                }

                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    match game.resize(width, height) {
                        Ok(()) => {}
                        Err(error) => {
                            eprintln!("{error}");
                            eprintln!("{}", error.backtrace());
                            elwt.exit();
                        }
                    }
                }

                WindowEvent::RedrawRequested => match game.draw() {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("{error}");
                        eprintln!("{}", error.backtrace());
                        elwt.exit();
                    }
                },

                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    is_synthetic: _,
                } => match game.input(event) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("{error}");
                        eprintln!("{}", error.backtrace());
                        elwt.exit();
                    }
                },

                _ => {}
            }
        }

        Event::DeviceEvent {
            device_id: _,
            event,
        } => match event {
            DeviceEvent::MouseMotion { delta: (x, y) } => match game.mouse_input(x as _, y as _) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{error}");
                    eprintln!("{}", error.backtrace());
                    elwt.exit();
                }
            },
            DeviceEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
            } => match game.scroll(x, y) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{error}");
                    eprintln!("{}", error.backtrace());
                    elwt.exit();
                }
            },
            _ => {}
        },

        Event::AboutToWait if !elwt.exiting() => {
            match game.update(dt) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("{error}");
                    eprintln!("{}", error.backtrace());
                    elwt.exit();
                    return;
                }
            }

            fixed_time += dt;
            while fixed_time >= ts {
                match game.fixed_update(ts) {
                    Ok(()) => {}
                    Err(error) => {
                        eprintln!("{error}");
                        eprintln!("{}", error.backtrace());
                        elwt.exit();
                        return;
                    }
                }
                fixed_time -= ts;
            }

            window.request_redraw();
        }

        Event::LoopExiting => {
            window.set_visible(false);
        }

        _ => {}
    })?;

    Ok(())
}
