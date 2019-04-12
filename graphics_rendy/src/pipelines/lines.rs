//!
//! Basic pipeline for the 2D world
//!

#![cfg_attr(
    not(any(feature = "dx12", feature = "metal", feature = "vulkan")),
    allow(unused)
)]

use std::marker::PhantomData;

use failure;
use lazy_static;

use {
    gfx_hal::Device as _,
    rendy::{
        command::{Families, QueueId, RenderPassEncoder},
        factory::{Config, Factory, ImageState},
        graph::{
            present::PresentNode, render::*, Graph, GraphBuilder, GraphContext, NodeBuffer,
            NodeImage,
        },
        memory::{Data, Dynamic},
        mesh::{AsVertex, PosTex, PosColor},
        mesh::line::{Line, LinePos},
        resource::{Buffer, BufferInfo, DescriptorSet, DescriptorSetLayout, Escape, Handle},
        shader::{Shader, ShaderKind, SourceLanguage, StaticShaderInfo},
        texture::Texture,
    },
};

lazy_static::lazy_static! {
    static ref VERTEX: StaticShaderInfo = StaticShaderInfo::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "pipelines/shaders/line.vert"),
        ShaderKind::Vertex,
        SourceLanguage::GLSL,
        "main",
    );
}

#[derive(Debug, Default)]
struct LinesPipelineDesc;

#[derive(Debug)]
struct LinesPipeline<B: gfx_hal::Backend> {
    lines_buffer: Option<Escape<Buffer<B>>>
}

impl<B, T> SimpleGraphicsPipelineDesc<B, T> for LinesPipelineDesc
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Pipeline = LinesPipeline<B>;

    fn vertices(
        &self,
    ) -> Vec<(
        Vec<gfx_hal::pso::Element<gfx_hal::format::Format>>,
        gfx_hal::pso::ElemStride,
        gfx_hal::pso::InstanceRate,
    )> {
        vec![PosColor::VERTEX.gfx_vertex_input_desc(0)]
    }

    fn load_shader_set<'a>(
        &self,
        storage: &'a mut Vec<B::ShaderModule>,
        factory: &mut Factory<B>,
        _aux: &T,
    ) -> gfx_hal::pso::GraphicsShaderSet<'a, B> {
        storage.clear();
        
        log::trace!("Load shader module '{:#?}'", *VERTEX);
        storage.push(unsafe { VERTEX.module(factory).unwrap() });
        
        gfx_hal::pso::GraphicsShaderSet {
            vertex: gfx_hal::pso::EntryPoint {
                entry: "main",
                module: &storage[0],
                specialization: gfx_hal::pso::Specialization::default(),
            },
            fragment: None,
            hull: None,
            domain: None,
            geometry: None,
        }
    }

    fn input_assembler(&self) -> gfx_hal::pso::InputAssemblerDesc {
        gfx_hal::pso::InputAssemblerDesc {
            primitive: gfx_hal::Primitive::LineList,
            primitive_restart: gfx_hal::pso::PrimitiveRestart::Disabled,
        }
    }
    
    fn build<'a>(
        self,
        _ctx: &mut GraphContext<B>,
        _factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &T,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout<B>>],
    ) -> Result<LinesPipeline<B>, failure::Error> {
        Ok(LinesPipeline {lines_buffer: None})
    }

}

impl<B, T> SimpleGraphicsPipeline<B, T> for LinesPipeline<B>
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Desc = LinesPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        _set_layouts: &[Handle<DescriptorSetLayout<B>>],
        _index: usize,
        _aux: &T,
    ) -> PrepareResult {
        if self.lines_buffer.is_none() {
            let mut vbuf = factory.create_buffer(BufferInfo{
                    size: PosColor::VERTEX.stride as u64 * 2,
                    usage: gfx_hal::buffer::Usage::VERTEX
                }, Dynamic,
            ).unwrap();

            unsafe {
                factory.upload_visible_buffer(&mut vbuf, 
                    0, 
                    &[
                        Line{
                            start: LinePos{
                                position: [0.0, -1.0].into(),
                            },
                            end: LinePos{
                                position: [0.0, 1.0].into(),
                            },
                            color: [0.0, 1.0, 0.0, 1.0].into()
                        }
                    ]
                ).unwrap();
            }
            self.lines_buffer = Some(vbuf);
        }
        PrepareResult::DrawReuse
    }

    fn draw(
        &mut self,
        _layout: &B::PipelineLayout,
        mut encoder: RenderPassEncoder<'_, B>,
        _index: usize,
        _aux: &T,
    ) {
        let vbuf = self.lines_buffer.as_ref().unwrap();
        encoder.bind_vertex_buffers(0, Some((vbuf.raw(), 0)));
        encoder.draw(0..3, 0..1);
    }

    fn dispose(self, _factory: &mut Factory<B>, _aux: &T) {}
}