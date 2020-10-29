use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, &FpsText)>) {
    for (mut text, _tag) in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn resource_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    //commands.spawn(UiCameraComponents::default());

    //// Background
    //let bg_0 = commands
    //.spawn(NodeComponents {
    //style: Style {
    //size: bevy::prelude::Size::new(Val::Px(2000.0), Val::Px(2000.0)),
    //..Default::default()
    //},
    //material: materials.add(Color::rgb(0.05, 0.05, 0.7).into()),
    //..Default::default()
    //})
    //.current_entity()
    //.unwrap();

    // Other node
    let background = commands
        .spawn(NodeComponents {
            style: Style {
                size: bevy::prelude::Size::new(Val::Px(150.0), Val::Px(100.0)),
                align_self: AlignSelf::Center,
                position: Rect {
                    left: Val::Px(1000.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.05, 0.05, 0.7).into()),
            ..Default::default()
        })
        //.with(Parent(bg_0))
        .current_entity()
        .unwrap();

    // Text
    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                //size: bevy::prelude::Size::new(Val::Px(900.0), Val::Px(1500.0)),
                //align_self: AlignSelf::Center,
                //align_self: AlignSelf::FlexEnd,
                //position: Rect {
                //top: Val::Px(1000.0),
                //left: Val::Px(1000.0),
                //..Default::default()
                //},
                ..Default::default()
            },
            text: Text {
                value: "Pause".to_string(),
                font: asset_server.load("/System/Library/Fonts/SFNS.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            //transform: Transform {
            //translation: Vec3::new(1000.0, 1000.0, 0.0),
            //..Default::default()
            //},
            ..Default::default()
        })
        //.with(FpsText);
        .with(Parent(background));
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ssssnake!".to_string(),
            width: 2000,
            height: 2000,
            ..Default::default()
        })
        //.add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(resource_setup.system())
        .add_default_plugins()
        .add_system(text_update_system.system())
        .run();
}
