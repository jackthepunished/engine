//! Tech Demo showcasing engine features
//!
//! Features demonstrated:
//! - Physics (Falling cube, movement)
//! - AI (Follower cube using Arrive steering)
//! - Particles (Smoke trail)
//! - UI (HUD elements)
//! - Audio (System integration)
//! - Animation (Procedural rotation)

use engine::ai::{Arrive, SteeringBehavior};
use engine::audio::AudioManager;
use engine::prelude::*;
use engine::renderer::{EmitterConfig, ParticleEmitter, UiRect};

/// Demo game with physics, AI, particles, and UI
struct DemoGame {
    camera: Camera,
    light: Light,

    // Meshes
    cube_mesh: Option<Mesh>,
    ground_mesh: Option<Mesh>,

    // Bind groups
    cube_model: Option<(wgpu::Buffer, wgpu::BindGroup)>,
    follower_model: Option<(wgpu::Buffer, wgpu::BindGroup)>,
    ground_model: Option<(wgpu::Buffer, wgpu::BindGroup)>,

    // Physics
    physics: Physics,
    cube_body: Option<RigidBodyHandle>,
    follower_body: Option<RigidBodyHandle>,

    // Particles
    emitter: Option<ParticleEmitter>,

    // Audio
    audio: Option<AudioManager>,

    // State
    camera_yaw: f32,
    camera_pitch: f32,
    show_ui: bool,
}

impl DemoGame {
    fn new() -> Self {
        Self {
            camera: Camera::look_at(Vec3::new(0.0, 5.0, 10.0), Vec3::ZERO, Vec3::Y),
            light: Light::new(Vec3::new(5.0, 10.0, 5.0)),
            cube_mesh: None,
            ground_mesh: None,
            cube_model: None,
            follower_model: None,
            ground_model: None,
            physics: Physics::new(),
            cube_body: None,
            follower_body: None,
            emitter: None,
            audio: None,
            camera_yaw: 0.0,
            camera_pitch: 0.3,
            show_ui: true,
        }
    }
}

impl Game for DemoGame {
    fn init(&mut self, ctx: &mut EngineContext) {
        log::info!("Initializing Tech Demo");

        // 1. Create Meshes
        let mut cube = Mesh::cube();
        let mut ground = Mesh::plane(20.0);

        ctx.renderer_mut().upload_mesh(&mut cube);
        ctx.renderer_mut().upload_mesh(&mut ground);

        // 2. Create Model Bind Groups
        self.cube_model = Some(ctx.renderer().create_model_bind_group(Mat4::IDENTITY));
        self.follower_model = Some(ctx.renderer().create_model_bind_group(Mat4::IDENTITY));
        self.ground_model = Some(ctx.renderer().create_model_bind_group(Mat4::IDENTITY));

        self.cube_mesh = Some(cube);
        self.ground_mesh = Some(ground);

        // 3. Setup Physics
        let ground_body = self.physics.create_static_body(Vec3::ZERO, Quat::IDENTITY);
        self.physics.add_ground_plane(ground_body);

        // Player cube
        let cube_body = self
            .physics
            .create_dynamic_body(Vec3::new(0.0, 5.0, 0.0), Quat::IDENTITY);
        self.physics
            .add_box_collider(cube_body, Vec3::splat(0.5), 1.0);
        self.cube_body = Some(cube_body);

        // Follower cube (different starting pos)
        let follower_body = self
            .physics
            .create_dynamic_body(Vec3::new(5.0, 5.0, -5.0), Quat::IDENTITY);
        self.physics
            .add_box_collider(follower_body, Vec3::splat(0.5), 1.0);
        self.follower_body = Some(follower_body);

        // 4. Setup Particles (Smoke trail)
        let config = EmitterConfig::default()
            .with_max_particles(500)
            .with_spawn_rate(50.0)
            .with_lifetime(0.5, 1.0)
            .with_size(0.1, 0.4)
            .with_colors(Vec4::new(0.8, 0.8, 0.8, 0.5), Vec4::new(0.2, 0.2, 0.2, 0.0));
        self.emitter = Some(ParticleEmitter::new(config));

        // 5. Setup Audio
        self.audio = AudioManager::new().ok();
        if let Some(audio) = &self.audio {
            log::info!(
                "Audio system active. Master volume: {}",
                audio.master_volume()
            );
        }

        self.camera.set_aspect(ctx.width(), ctx.height());
        log::info!("Tech Demo initialized");
    }

    fn update(&mut self, ctx: &mut EngineContext) {
        let dt = ctx.time.delta_seconds();

        if ctx.input.is_key_pressed(KeyCode::Escape) {
            ctx.quit();
            return;
        }

        // Camera control
        let rotation_speed = 2.0;
        if ctx.input.is_key_pressed(KeyCode::ArrowLeft) {
            self.camera_yaw -= rotation_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowRight) {
            self.camera_yaw += rotation_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowUp) {
            self.camera_pitch -= rotation_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowDown) {
            self.camera_pitch += rotation_speed * dt;
        }
        self.camera_pitch = self.camera_pitch.clamp(-1.4, 1.4);

        let distance = 12.0;
        self.camera.position = Vec3::new(
            distance * self.camera_yaw.cos() * self.camera_pitch.cos(),
            distance * self.camera_pitch.sin() + 3.0,
            distance * self.camera_yaw.sin() * self.camera_pitch.cos(),
        );
        self.camera.direction = (Vec3::ZERO - self.camera.position).normalize();

        // Player movement
        let force_strength = 20.0;
        let mut force = Vec3::ZERO;
        if ctx.input.is_key_pressed(KeyCode::KeyW) {
            force.z -= force_strength;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyS) {
            force.z += force_strength;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyA) {
            force.x -= force_strength;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyD) {
            force.x += force_strength;
        }

        if let Some(body) = self.cube_body {
            if force != Vec3::ZERO {
                self.physics.apply_force(body, force);

                // Spawn particles at player position
                if let (Some(pos), Some(emitter)) =
                    (self.physics.get_position(body), &mut self.emitter)
                {
                    emitter.set_position(pos);
                    emitter.start();
                }
            } else if let Some(emitter) = &mut self.emitter {
                emitter.stop();
            }
        }

        // AI Follower Logic
        if let (Some(target_body), Some(follower_body)) = (self.cube_body, self.follower_body) {
            let target_pos = self.physics.get_position(target_body).unwrap_or(Vec3::ZERO);
            let follower_pos = self
                .physics
                .get_position(follower_body)
                .unwrap_or(Vec3::ZERO);
            let follower_vel = self
                .physics
                .get_linear_velocity(follower_body)
                .unwrap_or(Vec3::ZERO);

            let arrive = Arrive::new(target_pos, 10.0, 5.0);
            let steering = arrive.calculate(follower_pos, follower_vel);

            self.physics
                .apply_force(follower_body, steering.linear * 5.0);
        }

        // Toggle UI
        if ctx.input.is_key_just_pressed(KeyCode::KeyU) {
            self.show_ui = !self.show_ui;
        }

        // Physics step
        self.physics.step(dt);

        // Update particle state
        if let Some(emitter) = &mut self.emitter {
            emitter.update(dt);
            emitter.upload(ctx.renderer().device(), ctx.renderer().queue());
        }

        // Update model transforms
        if let (Some(body), Some((buffer, _))) = (self.cube_body, &self.cube_model)
            && let (Some(pos), Some(rot)) = (
                self.physics.get_position(body),
                self.physics.get_rotation(body),
            )
        {
            ctx.renderer()
                .update_model_buffer(buffer, Mat4::from_rotation_translation(rot, pos));
        }
        if let (Some(body), Some((buffer, _))) = (self.follower_body, &self.follower_model)
            && let (Some(pos), Some(rot)) = (
                self.physics.get_position(body),
                self.physics.get_rotation(body),
            )
        {
            ctx.renderer()
                .update_model_buffer(buffer, Mat4::from_rotation_translation(rot, pos));
        }
    }

    fn render(&mut self, ctx: &mut EngineContext) {
        ctx.renderer_mut().update_camera(&self.camera);
        ctx.renderer_mut().update_light(&self.light);

        let Some(mut frame) = ctx.renderer().begin_frame() else {
            return;
        };

        {
            let mut render_pass = ctx.renderer().begin_render_pass(&mut frame);

            // 1. Draw 3D opaque
            if let (Some(mesh), Some((_, bg))) = (&self.ground_mesh, &self.ground_model) {
                ctx.renderer().draw_mesh(&mut render_pass, mesh, bg);
            }
            if let (Some(mesh), Some((_, bg))) = (&self.cube_mesh, &self.cube_model) {
                ctx.renderer().draw_mesh(&mut render_pass, mesh, bg);
            }
            // Draw follower in different color? (Currently using default material)
            if let (Some(mesh), Some((_, bg))) = (&self.cube_mesh, &self.follower_model) {
                ctx.renderer().draw_mesh(&mut render_pass, mesh, bg);
            }

            // 2. Draw Particles (Translucent)
            if let Some(emitter) = &self.emitter {
                ctx.renderer().draw_particles(&mut render_pass, emitter);
            }

            // 3. Draw UI HUD
            if self.show_ui {
                let mut ui_rects = Vec::new();

                // HUD Background
                ui_rects.push(UiRect {
                    position: [10.0, 10.0],
                    size: [240.0, 100.0],
                    color: [0.0, 0.0, 0.0, 0.7],
                });

                // Header (Electric Teal)
                ui_rects.push(UiRect {
                    position: [10.0, 10.0],
                    size: [240.0, 5.0],
                    color: [0.0, 0.9, 0.9, 1.0],
                });

                // FPS Indicator (Vibrant Green if high, Red if low)
                let fps = ctx.debug.frame_stats.fps();
                let fps_color = if fps > 30.0 {
                    [0.2, 0.9, 0.2, 1.0]
                } else {
                    [0.9, 0.2, 0.2, 1.0]
                };
                ui_rects.push(UiRect {
                    position: [20.0, 25.0],
                    size: [15.0, 15.0],
                    color: fps_color,
                });

                // Interaction Guide
                ui_rects.push(UiRect {
                    position: [10.0, ctx.height() as f32 - 40.0],
                    size: [ctx.width() as f32 - 20.0, 30.0],
                    color: [0.1, 0.1, 0.1, 0.8],
                });

                ctx.renderer().draw_ui(&mut render_pass, &ui_rects);
            }
        }

        ctx.renderer().end_frame(frame);
    }
}

fn main() {
    let config = EngineConfig::default()
        .with_title("Horizon Engine Tech Demo")
        .with_size(1280, 720)
        .with_vsync(true);

    let game = DemoGame::new();
    let engine = Engine::new(config, game);

    if let Err(e) = engine.run() {
        eprintln!("Engine error: {}", e);
    }
}
