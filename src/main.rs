extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use std::sync::Arc;

use vulkano::instance::{Instance, InstanceExtensions, ApplicationInfo, Version};
use vulkano::instance::debug::{DebugCallback, MessageType};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::LogicalSize
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: &[&str] = &[
    "VK_LAYER_LUNARG_standard_validation"
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

#[allow(unused)]
struct HelloTriangleApplication {
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,
    event_loop: EventLoop<()>,
}

impl HelloTriangleApplication {
    pub fn initialize() -> Self {
        let instance = Self::create_instance();
        let debug_callback = Self::setup_debug_callback(&instance);
        let event_loop = Self::init_window();

        Self {
            instance,
            debug_callback,
            event_loop,
        }
    }

    fn create_instance() -> Arc<Instance> {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("Hello triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0}),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0}),
        };

        let required_extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &required_extensions, None)
            .expect("failed to create a Vulkan instance")
    }

    fn init_window() -> EventLoop<()> {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_inner_size(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&event_loop);
        event_loop
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS {
            return None;
        }

        let msg_types = MessageType {
            error: true,
            warning: true,
            performance_warning: true,
            information: true,
            debug: true,
        };
        DebugCallback::new(&instance, msg_types, |msg| {
            println!("validation layer: {:?}", msg.description);
        }).ok()
    }

    fn main_loop(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    *control_flow = ControlFlow::Exit
                },
                Event::MainEventsCleared => {
                    //Application update code.
                },
                Event::RedrawRequested(_) => {
                    //Redraw...
                },
                _ => ()
            }
        });
        // self.event_loop.poll_events(|ev| {
        //     if let Event::WindowEvent {event: WindowEvent::CloseRequested, .. } = ev {
        //         done = true
        //     }
        // });
    }
}

fn main() {
    let mut app = HelloTriangleApplication::initialize();
    //app.main_loop();
}
