use std::time::Instant;

use ash::vk;
use calligraphy::Absatz;
use drowsed_math::{TransformQuaternion3D, FVec4, FVec3, FVec2, Transform2D};
use seraphine::{rendering::RenderingSystem, world::PushData3D, camera::Camera};
use shax::font::atlas::FontAtlas;
use structures::{GlobalUBO, Curve};
use vertex::{Vertex2DRGBA, Point2DRGBA, Vertex2DUV};
use winit::{dpi::LogicalSize, window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, event::WindowEvent};
use yum_mocha::{model::vertex::GlobalDebugVertex, device::{QueueFamilyIndices, DeviceQueueFlags}, input::input_state::GlobalInputState, pipelines::{graphics::{self, GraphicsPipelineBuilder}, create_shader_module}, descriptors::DescriptorWriter, rendering::mesh::Vertex, buffer};

use crate::calligraphy::{SchriftartGlipheEingang, Kalligraphie};
mod vertex;
mod calligraphy;
mod structures;
fn main() {
    // Font Initialization
    let data = std::fs::read("arial.ttf").unwrap();
    let vector: Vec<char> = "abcy".chars().collect();
    // let atlas = FontAtlas::new(vector, &data, 0.035);
    
    // Vulkan Object Initialization
    let event_loop = EventLoop::new();
    let window = std::sync::Arc::new(WindowBuilder::new()
        .with_title("Angelic")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop).unwrap()
    );
    let device = std::sync::Arc::new(yum_mocha::device::LogicalDeviceBuilder::new()
    .set_window(window.clone())
    .add_swapchain_extension()
    .check_queue_support(DeviceQueueFlags::GRAPHICS)
    .build((|prop, physical_device, surface, funcs|{
        let mut indices = QueueFamilyIndices::default();
        let mut queueinfo: Vec<(u32, u32, vk::CommandPoolCreateFlags)> = vec![];
        let mut i = 0;
        for family in prop {
            if family.queue_count > 0 && (family.queue_flags & vk::QueueFlags::GRAPHICS) == vk::QueueFlags::GRAPHICS {
                indices.graphics = Some(i);
            }
            let present_support = unsafe { ash::extensions::khr::Surface::get_physical_device_surface_support(funcs, *physical_device, i, surface.unwrap()).unwrap() };
            if family.queue_count > 0 && present_support {
                indices.surface = Some(i);
            }
            if indices.graphics.is_some() && indices.surface.is_some() {
                queueinfo.push((i, 1, vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER));
                break;
            }
            i += 1;
        }
        queueinfo
    })));
    let mut atlas = Kalligraphie::new(device.clone(), vector, &data, 0.055);
    let eingang = atlas.get_entry('a');
    let mut paragraph = Absatz::new();
    let font_idx = paragraph.add_font(device.clone(), &atlas);
    paragraph.push_str("string", font_idx);
    let letters = paragraph.iter().map(|info|{
        
    });

    let mut render_system = RenderingSystem::<PushData3D>::new(device.clone(),yum_mocha::device::WindowOption::Winit(window.clone()), vk::CommandBufferLevel::PRIMARY);

    let descriptor_pool = yum_mocha::descriptors::DescriptorPoolBuilder::new(device.clone())
    .set_max_sets(4)
    // * .add_pool_size(vk::DescriptorType::UNIFORM_BUFFER, 2)
    // * .add_pool_size(vk::DescriptorType::STORAGE_BUFFER, 2)
    .add_pool_size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 2)
    .build();
    let descriptor_layout = yum_mocha::descriptors::DescriptorLayoutBuilder::new(device.clone())
    // * .add_binding(0, vk::DescriptorType::UNIFORM_BUFFER, 1, vk::ShaderStageFlags::ALL_GRAPHICS)
    // * .add_binding(1, vk::DescriptorType::STORAGE_BUFFER, 1, vk::ShaderStageFlags::FRAGMENT)
    .add_binding(0, vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 1, vk::ShaderStageFlags::FRAGMENT)
    .build();
    let layout = yum_mocha::pipelines::layout::PipelineLayoutBuilder::new()
        .add_descriptor_layout(descriptor_layout.layout)
        .add_push_constant::<PushData3D>(vk::ShaderStageFlags::ALL_GRAPHICS)
        .build(device.clone());
    let graphics = {
        let vertex = create_shader_module(&device.device, "./shaders/vertex2uv.vert.spv");
        let fragment = create_shader_module(&device.device, "./shaders/vertex2uv.frag.spv");
        let stages1 = vec![
            vk::PipelineShaderStageCreateInfo {
            module: vertex,
            stage: vk::ShaderStageFlags::VERTEX,
            p_name: "main\0".as_ptr() as *const i8,
            ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
            module: fragment,
            stage: vk::ShaderStageFlags::FRAGMENT,
            p_name: "main\0".as_ptr() as *const i8,
            ..Default::default()
            }
        ];
        let vertex2 = create_shader_module(&device.device, "./shaders/points.vert.spv");
        let fragment2 = create_shader_module(&device.device, "./shaders/points.frag.spv");
        let stages2 = vec![
            vk::PipelineShaderStageCreateInfo {
            module: vertex2,
            stage: vk::ShaderStageFlags::VERTEX,
            p_name: "main\0".as_ptr() as *const i8,
            ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
            module: fragment2,
            stage: vk::ShaderStageFlags::FRAGMENT,
            p_name: "main\0".as_ptr() as *const i8,
            ..Default::default()
            }
        ];
        
            let binding = Vertex2DUV::binding_description();
            let attribute = Vertex2DUV::attribute_description();
            let binding2 = Point2DRGBA::binding_description();
            let attribute2 = Point2DRGBA::attribute_description();
            GraphicsPipelineBuilder::new()
            .add_unique_shader_module(vertex)
            .add_unique_shader_module(fragment)
            .add_shader_stage(stages1.clone())
            .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR])
            .pipeline_layout(layout.get_layout())
            .render_pass(render_system.get_render_pass())
            .subpass(0)
            .rasterization(vk::PolygonMode::FILL, vk::CullModeFlags::NONE, 1.0)
            .vertex_input_state(&binding, &attribute) // Vertex2DRGBA
            .input_assembly_state(vk::PrimitiveTopology::TRIANGLE_LIST)
            .push_info()
            .vertex_input_state(&binding2, &attribute2) // Vertex2DRGBA
            .rasterization(vk::PolygonMode::POINT, vk::CullModeFlags::NONE, 1.0)
            .input_assembly_state(vk::PrimitiveTopology::POINT_LIST)
            .add_shader_stage(stages2)
            .push_info()
            .build(device.clone(), vk::PipelineCache::null())
    };
    // Seraphine Object Code
    let global_input = GlobalInputState::new();
    let mut world = seraphine::world::World::<PushData3D, Transform2D>::new();
    let mut scene = seraphine::world::scene::Scene::<PushData3D>::new();
    let mut resized = false;
    let mut current_time = Instant::now();
    let mut delta_time = 0.0;
    let mut camera = Camera::default();
    let layouts = [descriptor_layout.layout, descriptor_layout.layout].as_ptr();
    // Descriptor Sets Startup Code
    // * let curves = Curve::curve_setup();

    let sets = descriptor_pool.allocate(device.clone(), layouts, 2);
    // * let window_size = window.inner_size();
    // * let ubo_data = GlobalUBO {
    // *     resolution: FVec2::new(window_size.width as f32, window_size.height as f32),
    // * };
    // * let global_ubo = yum_mocha::buffer::raw::Buffer::from_value(device.clone(), vk::BufferUsageFlags::UNIFORM_BUFFER, vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, &ubo_data);
    // * let storage_length = 8 + curves.len() * std::mem::size_of::<Curve>();
    // * let mut storage = yum_mocha::buffer::raw::Buffer::<Curve>::new(
    // *     device.clone(), 
    // *     storage_length, 
    // *     vk::BufferUsageFlags::STORAGE_BUFFER, 
    // *     vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
    // * );
    // * storage.mapping(device.clone(), storage_length, 0);
    // * unsafe { storage.write_value::<i32>(0, &(curves.len() as i32)) };
    // * unsafe { storage.write_vec(8, &curves) };
    let font_texture = buffer::img::ImageTexture::from_vec(device.clone(), vk::Format::R8G8B8A8_UINT, atlas.schriftart_atlas.bmp.width() as u32, atlas.schriftart_atlas.bmp.height() as u32, &atlas.schriftart_atlas.bmp.as_byte_slice().to_vec());
    for i in 0..2 {
        // * let info = vk::DescriptorBufferInfo {
        // *     buffer: global_ubo.buffer,
        // *     offset: 0,
        // *     range: global_ubo.capacity_in_bytes() as u64
        // * };
        // * let info2 = vk::DescriptorBufferInfo {
        // *     buffer: storage.buffer,
        // *     offset: 0,
        // *     range: storage_length as u64
        // * };
        let info = font_texture.get_info(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let writer = DescriptorWriter::new()
            .add_image_buffer(sets[i], 1, 0, 0, &info);
        // * .add_uniform_buffer(sets[i], 1, 0, 0, &info)
        // * .add_storage_buffer(sets[i], 1, 1, 0, &info2);
        writer.write(device.clone());
    }
    // Object Setup
    world.push_object(0, Transform2D::default());
    // Mesh Loading
    // let (bmin, bmax) = atlas.place_letter('a');
    let square = seraphine::model::Model::<Vertex2DUV>::new(vec![
        Vertex2DUV {
            // pos: FVec2::new(bmin.x, bmax.y)* 0.035,//* 0.032,
            // uv: FVec2::new(min.x, max.y),
            pos: eingang.pos_rahmen.top_left,
            uv: eingang.brief_rahmen.top_left
        },
        Vertex2DUV {
            // pos: FVec2::new(bmax.x, bmin.y)* 0.035,// * 0.032,
            // uv: FVec2::new(max.x , min.y),
            pos: eingang.pos_rahmen.bottom_left,
            uv: eingang.brief_rahmen.bottom_left
        },
        Vertex2DUV {
            // pos: FVec2::from(bmin)* 0.035,//* 0.032,
            // uv: min,
            pos: eingang.pos_rahmen.bottom_right,
            uv: eingang.brief_rahmen.bottom_right
        },
        Vertex2DUV {
            // pos: FVec2::from(bmax)* 0.035,//* 0.032,
            // uv: max,
            pos: eingang.pos_rahmen.top_right,
            uv: eingang.brief_rahmen.top_right
        },
    ], vec![0, 2, 1, 0, 3, 1]);
    // println!("{:?}\n{:?}\n", min, max);
    // Mesh Setup
    let squareidx = render_system.push_mesh(0, &square, vk::ShaderStageFlags::ALL_GRAPHICS);
    // Scene Setup
    scene.add_group(seraphine::world::scene::RenderGroupType::Dynamic, graphics.pipelines[0], vk::ShaderStageFlags::ALL_GRAPHICS, vec![(0, Some(squareidx))]);
    let mut time = 0.0;
    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { 
                window_id: _, 
                event: WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ },
            } => {
                let mut inputlock = global_input.lock().unwrap();
                inputlock.poll(input);
            }
            winit::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } 
            if window_id == window.id() => *control_flow = ControlFlow::Exit,
            winit::event::Event::WindowEvent { window_id: _, event: WindowEvent::Resized(_) } => {
                // let aspect = render_system.get_aspect_ratio();
                // camera.set_perspective_projection(0.872665, aspect, 0.1, 150.0);
                resized = true;
            }
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                // let triangle = world.objects.get_mut(&0).unwrap();
                // triangle.translation = FVec2::new(1.0, -time);
                let new_time = Instant::now();
                let mut frame = render_system.start_frame(device.clone(), layout.get_layout());
                let obj0 = world.objects.get_mut(&0).unwrap();
                frame.begin();
                scene.set_descriptor_set(0, vec![sets[frame.get_current_frame()]]);
                let lock = global_input.lock().unwrap();
                if lock.is_pressed(winit::event::VirtualKeyCode::W) {
                    obj0.translation.y += 1.0 * delta_time;
                }
                frame.render(&scene, &world, &camera);
                frame.end(&mut resized);
                delta_time = (new_time - current_time).as_secs_f32();
                current_time = new_time;
                time += delta_time;
            }
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
