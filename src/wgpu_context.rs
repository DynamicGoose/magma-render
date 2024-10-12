use wgpu::InstanceFlags;
use winit::window::Window;

pub struct WgpuContext<'a> {
    pub surface: wgpu::Surface<'a>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub graphics_queue: (wgpu::Device, wgpu::Queue),
    pub compute_queue: (wgpu::Device, wgpu::Queue),
    pub copy_queue: (wgpu::Device, wgpu::Queue),
    pub window: &'a Window,
}

impl<'a> WgpuContext<'a> {
    pub fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: InstanceFlags::empty(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let graphics_queue = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
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
                label: None,
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
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Self {
            surface,
            surface_config,
            graphics_queue,
            compute_queue,
            copy_queue,
            window,
        }
    }
}
