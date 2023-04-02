use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::prelude::{AssetServer, FromWorld, Resource, World};
use bevy::render::render_resource::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType, TextureViewDimension};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;

#[derive(Resource)]
pub struct PostProcessPipeline {
	pub layout : BindGroupLayout,
	pub sampler : Sampler,
	pub pipeline_id : CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
	fn from_world(world: &mut World) -> Self {
		let render_device = world.resource::<RenderDevice>();
		
		// We need to define the bind group layout used for our pipeline
		let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some("post_process_bind_group_layout"),
			entries: &[
				// The screen texture
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				
				// Depth texture
				BindGroupLayoutEntry {
					binding: 1,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Depth,
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				
				// Normal texture
				BindGroupLayoutEntry {
					binding: 2,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Texture {
						sample_type: TextureSampleType::Float { filterable: true },
						view_dimension: TextureViewDimension::D2,
						multisampled: false,
					},
					count: None,
				},
				
				// The sampler that will be used to sample the screen
				BindGroupLayoutEntry {
					binding: 3,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Sampler(SamplerBindingType::Filtering),
					count: None,
				},
			],
		});
		
		// We can create the sampler here since it won't change at runtime and doesn't depend on the view
		let sampler = render_device.create_sampler(&SamplerDescriptor::default());
		
		// Get the shader handle
		let shader = world
			.resource::<AssetServer>()
			.load("shaders/post_process_pass.wgsl");
		
		let pipeline_id = world
			.resource_mut::<PipelineCache>()
			// This will add the pipeline to the cache and queue it's creation
			.queue_render_pipeline(RenderPipelineDescriptor {
				label: Some("post_process_pipeline".into()),
				layout: vec![layout.clone()],
				// This will setup a fullscreen triangle for the vertex state
				vertex: fullscreen_shader_vertex_state(),
				fragment: Some(FragmentState {
					shader,
					shader_defs: vec![],
					// Make sure this matches the entry point of the shader.
					// It can be anything as long as it matches here and in the shader
					entry_point: "fragment".into(),
					targets: vec![Some(ColorTargetState {
						format: TextureFormat::bevy_default(),
						blend: None,
						write_mask: ColorWrites::ALL,
					})],
				}),
				
				// All of the following properties are not important for this effect, so the defaults are used.
				// (this struct doesn't impl default because some fields can't have defaults apparently)
				primitive: PrimitiveState::default(),
				depth_stencil: None,
				multisample: MultisampleState::default(),
				push_constant_ranges: vec![],
			});
		
		Self {
			layout,
			sampler,
			pipeline_id,
		}
	}
}
