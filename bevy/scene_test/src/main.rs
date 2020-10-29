use bevy::{prelude::*, type_registry::TypeRegistry};
use std::fs;

fn resource_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(UiCameraComponents::default());

    commands
        .spawn(NodeComponents {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(2000.0), Val::Px(2000.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                    ..Default::default()
                },
                text: Text {
                    value: "Pause".to_string(),
                    font: asset_server.load("/System/Library/Fonts/SFNS.ttf"),
                    style: TextStyle {
                        font_size: 200.0,
                        color: Color::WHITE,
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn save_scene_system(world: &mut World, resources: &mut Resources) {
    let keyboard_input = resources.get::<Input<KeyCode>>().unwrap();
    if keyboard_input.just_pressed(KeyCode::S) {
        let type_registry = resources.get::<TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(&world, &type_registry.component.read());
        let ron_scene = scene.serialize_ron(&type_registry.property.read()).unwrap();
        //dbg!(ron_scene);

        //let path = "scene.scn";
        //let mut outfile = File::create(path)?;
        //write!(output, ron_scene)?;
        fs::write("scene.scn", ron_scene).unwrap();
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ssssnake!".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        .add_startup_system(resource_setup.system())
        .add_system(save_scene_system.thread_local_system())
        .add_default_plugins()
        .run();
}
