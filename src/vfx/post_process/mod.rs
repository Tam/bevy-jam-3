mod post_process_settings;
mod post_process_pipeline;
mod post_process_node;

use bevy::app::App;
use bevy::core_pipeline::core_3d;
use bevy::prelude::Plugin;
use bevy::render::extract_component::{ExtractComponentPlugin, UniformComponentPlugin};
use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;
use crate::vfx::post_process::post_process_node::PostProcessNode;
use crate::vfx::post_process::post_process_pipeline::PostProcessPipeline;
pub use crate::vfx::post_process::post_process_settings::PostProcessSettings;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
	fn build(&self, app: &mut App) {
		app
			// The settings will be a component that lives in the main world but will
			// be extracted to the render world every frame.
			// This makes it possible to control the effect from the main world.
			// This plugin will take care of extracting it automatically.
			// It's important to derive [`ExtractComponent`] on [`PostProcessingSettings`] for this plugin to work correctly.
			.add_plugin(ExtractComponentPlugin::<PostProcessSettings>::default())
			// The settings will also be the data used in the shader.
			// This plugin will prepare the component for the GPU by creating a uniform buffer
			// and writing the data to that buffer every frame.
			.add_plugin(UniformComponentPlugin::<PostProcessSettings>::default())
		;
		
		// We need to get the render app from the main app
		let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
			return;
		};
		
		// Initialise the pipeline
		render_app.init_resource::<PostProcessPipeline>();
		
		// Create the node with the render world
		let node = PostProcessNode::new(&mut render_app.world);
		
		// Get the render graph for the entire app
		let mut graph = render_app.world.resource_mut::<RenderGraph>();
		
		// Get the render graph for 3D cameras/views
		let core_3d_graph = graph.get_sub_graph_mut(core_3d::graph::NAME).unwrap();
		
		// Register the post process node in the 3D render graph
		core_3d_graph.add_node(PostProcessNode::NAME, node);
		
		// A slot edge tells the render graph which input/output value should be passed to the node.
		// In this case, the view entity, which is the entity associated with the camera on which the graph is running.
		core_3d_graph.add_slot_edge(
			core_3d_graph.input_node().id,
			core_3d::graph::input::VIEW_ENTITY,
			PostProcessNode::NAME,
			PostProcessNode::IN_VIEW,
		);
		
		// Add an edge between our node and the nodes from bevy to ensure it's ordered correctly
		// Have the node run after tonemapping but before the end of the main pass
		core_3d_graph.add_node_edge(core_3d::graph::node::TONEMAPPING, PostProcessNode::NAME);
		core_3d_graph.add_node_edge(PostProcessNode::NAME, core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING);
	}
}
