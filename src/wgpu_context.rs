use wgpu::InstanceFlags;

pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub graphics_queue: (wgpu::Device, wgpu::Queue),
    pub compute_queue: (wgpu::Device, wgpu::Queue),
    pub copy_queue: (wgpu::Device, wgpu::Queue),
    // pub window: &'a Window,
}

impl WgpuContext {
    pub fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: InstanceFlags::empty(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        // let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Some(&surface)
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .unwrap();

        let graphics_queue = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("graphics"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        ))
        .unwrap();

        let compute_queue = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("compute"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        ))
        .unwrap();

        let copy_queue = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("copy"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        ))
        .unwrap();

        Self {
            instance,
            graphics_queue,
            compute_queue,
            copy_queue,
            // window,
        }
    }
}

impl Default for WgpuContext {
    fn default() -> Self {
        Self::new()
    }
}
