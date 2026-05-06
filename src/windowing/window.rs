use std::sync::Arc;

use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp,
    wgt::CommandEncoderDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes, WindowId},
};

use crate::renderer::RenderContext;

use super::input::InputState;

pub struct ActiveWindow {
    pub window: Arc<Window>,
    pub input: InputState,
    pub renderer: RenderContext,
}

pub struct WindowState {
    active: Option<ActiveWindow>,
    title: String,
    width: u32,
    height: u32,
}

impl WindowState {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            active: None,
            title: title.into(),
            width,
            height,
        }
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.active.as_ref().expect("window not yet created").window
    }

    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.active.as_mut().expect("window not yet created").input
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().expect("failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(&mut self).expect("event loop error");
    }
}

impl ApplicationHandler for WindowState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.active.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_visible(false);

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("failed to create window"),
        );

        let renderer = pollster::block_on(RenderContext::new(window.clone()));

        window.set_visible(true);

        self.active = Some(ActiveWindow {
            window,
            input: InputState::default(),
            renderer,
        })
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(active) = self.active.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(keycode),
                        state,
                        ..
                    },
                ..
            } => match state {
                ElementState::Pressed => active.input.press(keycode),
                ElementState::Released => active.input.release(keycode),
            },
            WindowEvent::RedrawRequested => {
                // Game loop
                active.input.tick();

                if let Some((output, view)) = active.renderer.current_frame() {
                    let mut encoder =
                        active
                            .renderer
                            .device
                            .create_command_encoder(&CommandEncoderDescriptor {
                                label: Some("frame_encoder"),
                            });
                    {
                        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("clear_pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: Operations {
                                    load: wgpu::LoadOp::Clear(Color {
                                        r: 0.1,
                                        g: 0.1,
                                        b: 0.15,
                                        a: 1.0,
                                    }),
                                    store: StoreOp::Store,
                                },
                                depth_slice: None,
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                            multiview_mask: None,
                        });
                    }

                    active
                        .renderer
                        .queue
                        .submit(std::iter::once(encoder.finish()));
                    output.present();
                }

                active.window.request_redraw();
            }
            WindowEvent::Resized(_new_size) => {}

            _ => {}
        }
    }
}
