use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

//
//
// Plugin

pub struct PerfLogPlugin;

impl Plugin for PerfLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(init_system)
            .add_system(log_system)
            .add_system(log_visibility_system);
    }
}

//
//
// Components

#[derive(Component)]
struct PerfLogUI;

//
//
// Resources

struct PerfLogEnabled(bool);

//
//
// Systems

fn init_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/roboto_condensed/RobotoCondensed-Regular.ttf");

    let regular_text = TextStyle {
        font: font.clone(),
        font_size: 32.0,
        color: Color::rgba(1.0, 1.0, 1.0, 0.9),
    };

    commands
        .spawn_bundle(
            TextBundle::from_section("", regular_text.clone()).with_style(Style {
                position: UiRect {
                    bottom: Val::Px(16.0),
                    left: Val::Px(16.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(PerfLogUI);

    commands.insert_resource(PerfLogEnabled(false));
}

fn log_visibility_system(
    input: Res<Input<KeyCode>>,
    mut do_log: ResMut<PerfLogEnabled>,
    mut q: Query<&mut Visibility, With<PerfLogUI>>,
) {
    if input.just_pressed(KeyCode::Grave) {
        do_log.0 = !do_log.0;
    }

    q.for_each_mut(|mut visibility| {
        visibility.is_visible = do_log.0;
    });
}

fn log_system(diag: Res<Diagnostics>, mut q: Query<&mut Text, With<PerfLogUI>>) {
    let mut fps = 0.0;
    if let Some(fps_diag) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diag.average() {
            fps = fps_avg;
        }
    }

    q.for_each_mut(|mut text| {
        text.sections[0].value = format!("fps: {:.0}", fps);
    });
}
