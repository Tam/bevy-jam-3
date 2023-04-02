use bevy::core_pipeline::prepass::ViewPrepassTextures;
use bevy::prelude::{Msaa, QueryState, With, World};
use bevy::render::extract_component::ComponentUniforms;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType};
use bevy::render::render_resource::{BindGroupDescriptor, BindGroupEntry, BindingResource, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::{DefaultImageSampler, FallbackImageMsaaCache, FallbackImagesDepth, FallbackImagesMsaa};
use bevy::render::view::{ExtractedView, ViewTarget};
use crate::vfx::post_process::post_process_pipeline::PostProcessPipeline;
use crate::vfx::post_process::post_process_settings::PostProcessSettings;

/// The post process node used for the render graph
pub struct PostProcessNode {
	// The node needs a query to gather data from the ECS in order to do its rendering,
	// but it's not a normal system so wee need to define it manually.
	query : QueryState<(
		&'static ViewTarget,
		Option<&'static ViewPrepassTextures>,
	), With<ExtractedView>>,
}

impl PostProcessNode {
	pub const IN_VIEW : &str = "view";
	pub const NAME : &str = "post_process";
	
	pub fn new (world : &mut World) -> Self {
		Self {
			query: QueryState::new(world),
		}
	}
}

impl Node for PostProcessNode {
	// This defines the input slot of the node and tells render graph what
	// we will need when running the node
	fn input(&self) -> Vec<SlotInfo> {
		// In this case we tell the graph that our node will use the view entity
		// Currently every node in bevy uses this pattern, so we can copy it.
		// This boilerplate won't be needed in bevy 0.11
		vec![SlotInfo::new(PostProcessNode::IN_VIEW, SlotType::Entity)]
	}
	
	// Run every frame before the run() method
	// the difference being that self is mutable here
	fn update(&mut self, world : &mut World) {
		// Since this is not a system we need to update the query manually.
		// (boilerplate, bevy have plans to remove this in the future, for now we just copy)
		self.query.update_archetypes(world);
	}
	
	// The main logic, where you encode draw commands for the GPU (I assume)
	// This will run on every view on which the graph is running, if you don't want the effect
	// to run on every camera, you'll need to have a marker component to identify which cameras
	// it should run on.
	fn run(
		&self,
		graph: &mut RenderGraphContext,
		render_context: &mut RenderContext,
		world: &World,
	) -> Result<(), NodeRunError> {
		// Get the entity of the view for the render graph where this node is running
		let view_entity = graph.get_input_entity(PostProcessNode::IN_VIEW)?;
		
		// We get the data we need from the world based on the view entity passed to the node.
		// The data is the query that was defined earlier in the PostProcessNode
		let Ok((
			view_target,
			prepass_textures,
		)) = self.query.get_manual(world, view_entity) else {
			return Ok(());
		};
		
		// Get the pipeline resource that contains the global data we need to create the render pipeline
		let post_process_pipeline = world.resource::<PostProcessPipeline>();
		
		// The pipeline cache is a cache of all previously created pipelines.
		// It's required to avoid creating a new pipeline each frame.
		let pipeline_cache = world.resource::<PipelineCache>();
		
		// Get the pipeline from the cache
		let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id) else {
			return Ok(());
		};
		
		// Get the settings uniform binding
		let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
		let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
			return Ok(());
		};
		
		// This will start a new "post process write", getting two texture views from
		// the view target - a source & a destination.
		// source is the current main texture
		// You must write into destination because calling post_process_write() on the
		// ViewTarget will internally flip the ViewTargets main texture to teh destination
		// texture. Failing to do so will cause the current main texture info to be lost.
		let post_process = view_target.post_process_write();
		
		
		// Get prepass textures
		// TODO: Work out how to query a SystemParam from world
		/*
		let fallback_images : &mut FallbackImagesMsaa;
		let fallback_depths : &mut FallbackImagesDepth;
		let msaa = world.resource::<Msaa>();
		
		let depth_view = match prepass_textures.and_then(|x| x.depth.as_ref()) {
			Some(texture) => &texture.default_view,
			None => {
				&fallback_depths
					.image_for_samplecount(msaa.samples())
					.texture_view
			},
		};
		
		let normal_view = match prepass_textures.and_then(|x| x.normal.as_ref()) {
			Some(texture) => &texture.default_view,
			None => {
				&fallback_images
					.image_for_samplecount(msaa.samples())
					.texture_view
			},
		};
		*/
		// FIXME: Get the above working to account for missing prepass textures
		let depth_view = &prepass_textures.unwrap().depth.as_ref().unwrap().default_view;
		let normal_view = &prepass_textures.unwrap().normal.as_ref().unwrap().default_view;
		
		// The bind_group gets created each frame.
		// Normally you'd create a bind_group in the Queue set, but this doesn't work
		// with post_process_write() because it alternates the source/destination.
		// To ensure we have the correct target we have to get it during node execution.
		let bind_group = render_context
			.render_device()
			.create_bind_group(&BindGroupDescriptor {
				label: Some("post_process_bind_group"),
				layout: &post_process_pipeline.layout,
				// It's important for this to match the BindGroupLayout defined in PostProcessPipeline
				entries: &[
					BindGroupEntry {
						binding: 0,
						// Make sure to use the source view
						resource: BindingResource::TextureView(post_process.source),
					},
					BindGroupEntry {
						binding: 1,
						resource: BindingResource::TextureView(depth_view),
					},
					BindGroupEntry {
						binding: 2,
						resource: BindingResource::TextureView(normal_view),
					},
					BindGroupEntry {
						binding: 3,
						// Use the sampler created for the pipeline
						resource: BindingResource::Sampler(&post_process_pipeline.sampler),
					},
				],
			});
		
		// Begin the render pass
		let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
			label: Some("post_process_pass"),
			color_attachments: &[Some(RenderPassColorAttachment {
				// We need to specify the post process destination view here
				// to make sure we write to the appropriate texture
				view: post_process.destination,
				resolve_target: None,
				ops: Operations::default(),
			})],
			depth_stencil_attachment: None,
		});
		
		// This is mostly just WGPU boilerplate for drawing a fullscreen triangle,
		// using the pipeline / bind_group created above
		render_pass.set_render_pipeline(pipeline);
		render_pass.set_bind_group(0, &bind_group, &[]);
		render_pass.draw(0..3, 0..1);
		
		Ok(())
	}
}
