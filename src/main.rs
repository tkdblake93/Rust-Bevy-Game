use bevy::prelude::*;

fn main() {
    App::new()
	.add_plugins(DefaultPlugins) // Loads the important plugins for rendering, input, and more.
	.add_startup_system(setup) // Registers the setup system to run once at startup.
	.run(); // Starts the application.
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Spawn a 3D camera that looks at the origin (0,0,0).
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0.0, 8.0, 15.0)
			.looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

	// Spawn a point light to illuminate the scene.
	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 1000.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});

	// Spawn a cube in the scene.
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
		material: materials.add(StandardMaterial {
			base_color: Color::rgb(0.8, 0.2, 0.2),
			..default()
		}),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
}
