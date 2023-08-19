use ash::vk;
use bytemuck::offset_of;
use drowsed_math::{FVec2, FVec4, FVec3};
use yum_mocha::rendering::mesh::Vertex;

#[repr(C, align(16))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex2DRGBA {
    pub rgba: FVec4,
    pub pos: FVec2,
}
impl Vertex for Vertex2DRGBA {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Self, rgba) as u32,
        };
        let attr1 = vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, pos) as u32,
        };
        let attributes = vec![attr, attr1];
        attributes
    }
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }
}


#[repr(C, align(16))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Point2DRGBA {
    pub rgba: FVec4,
    pub pos: FVec2,
    pub size: f32,
}
impl Vertex for Point2DRGBA {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Self, rgba) as u32,
        };
        let attr1 = vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, pos) as u32,
        };
        let attr2 = vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(Self, size) as u32,
        };
        let attributes = vec![attr, attr1, attr2];
        attributes
    }
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }
}



#[repr(C, align(16))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vertex2DUV {
    pub pos: FVec2,
    pub uv: FVec2,
}
impl Vertex for Vertex2DUV {
    fn attribute_description() -> Vec<vk::VertexInputAttributeDescription> {
        let attr = vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(Self, pos) as u32,
        };
        let attr1 = vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Self, uv) as u32,
        };
        let attributes = vec![attr, attr1];
        attributes
    }
    fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }
    }
}