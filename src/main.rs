use std::thread;
use std::time::Duration;
use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::core::CorePlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gltf::GltfPlugin;
use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::ui::UiPlugin;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugin(LogPlugin::default());
    app.add_plugin(CorePlugin::default());
    app.add_plugin(TimePlugin::default());
    app.add_plugin(TransformPlugin::default());
    app.add_plugin(HierarchyPlugin::default());
    app.add_plugin(DiagnosticsPlugin::default());
    app.add_plugin(InputPlugin::default());
    app.add_plugin(WindowPlugin::default());
    app.add_plugin(AssetPlugin::default());
    app.add_plugin(ScenePlugin::default());
    app.add_plugin(WinitPlugin::default());
    app.add_plugin(RenderPlugin::default());
    app.add_plugin(CorePipelinePlugin::default());
    app.add_plugin(SpritePlugin::default());
    app.add_plugin(TextPlugin::default());
    app.add_plugin(UiPlugin::default());
    app.add_plugin(PbrPlugin::default());

    // NOTE: Load this after renderer initialization so that it knows about the supported
    // compressed texture format
    app.add_plugin(GltfPlugin::default());
    app.add_plugin(AudioPlugin::default());
    app.add_plugin(GilrsPlugin::default());
    app.add_plugin(AnimationPlugin::default());


    // Exit automatically so CI doesent hang
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(10));
        std::process::exit(0);
    });

    app.run();
}
