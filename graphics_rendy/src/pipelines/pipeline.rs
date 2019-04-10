//!
//! Basic pipeline for the 2D world
//!

#![cfg_attr(
    not(any(feature = "dx12", feature = "metal", feature = "vulkan")),
    allow(unused)
)]

use std::marker::PhantomData;

use failure;

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
        mesh::{AsVertex, PosTex},
        resource::{Buffer, BufferInfo, DescriptorSet, DescriptorSetLayout, Escape, Handle},
        shader::{Shader, ShaderKind, SourceLanguage, StaticShaderInfo},
        texture::Texture,
    },
};

#[derive(Debug, Default)]
struct WorldPipelineDesc;

#[derive(Debug)]
struct WorldPipeline<B: gfx_hal::Backend> {
    pd: PhantomData<B>
}

impl<B, T> SimpleGraphicsPipelineDesc<B, T> for WorldPipelineDesc
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Pipeline = WorldPipeline<B>;

    fn depth_stencil(&self) -> Option<gfx_hal::pso::DepthStencilDesc> {
        None
    }

    fn load_shader_set<'a>(
        &self,
        storage: &'a mut Vec<B::ShaderModule>,
        _factory: &mut Factory<B>,
        _aux: &T,
    ) -> gfx_hal::pso::GraphicsShaderSet<'a, B> {
        storage.clear();
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

    fn build<'a>(
        self,
        _ctx: &mut GraphContext<B>,
        _factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &T,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout<B>>],
    ) -> Result<WorldPipeline<B>, failure::Error> {
        Ok(WorldPipeline {pd: PhantomData})
    }

}

impl<B, T> SimpleGraphicsPipeline<B, T> for WorldPipeline<B>
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Desc = WorldPipelineDesc;


    fn draw(
        &mut self,
        _layout: &B::PipelineLayout,
        mut encoder: RenderPassEncoder<'_, B>,
        _index: usize,
        _aux: &T,
    ) {

    }

    fn dispose(self, _factory: &mut Factory<B>, _aux: &T) {}
}