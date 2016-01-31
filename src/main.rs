extern crate sdl2;
extern crate sdl2_image;
extern crate time;

//includes
mod constants;
mod math;
mod drawable;
mod actor;
mod world;
mod playerinput;
mod projectile;
mod player;
mod orb;
mod controller;
mod spawner;

//use sdl2_image::{self, LoadTexture, INIT_PNG, INIT_JPG};
use sdl2_image::LoadTexture;
use std::path::Path;
use sdl2::rect::{Point};
use constants::*;
use sdl2::pixels;

static mut running: bool = true;
static mut left_mouse_down: bool = false;

fn handle_event(event: sdl2::event::Event) {
	use sdl2::event::Event;
	use sdl2::keyboard::Keycode;
	match event {
		Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => unsafe {
				running = false;
		},
		Event::MouseButtonDown {mouse_btn: sdl2::mouse::Mouse::Left, ..} => unsafe {
			left_mouse_down = true;
		},
		_ => {
		}
	}
}

fn main() {
	let sdl_context = sdl2::init().ok().expect("Failed to create context.");
	let video_sys = sdl_context.video().ok().expect("Failed to create video context.");
	let _image_context = sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);
	let window = video_sys.window("Rituler", WIN_WIDTH as u32, WIN_HEIGHT as u32).build().ok().expect("Failed to create window.");
	let mut renderer = window.renderer().build().ok().expect("Failed to create renderer.");
	
	// the "textureManager"
	let mut textures : Vec < sdl2::render::Texture > = Vec::new();
	
	//load resources
	textures.push(renderer.load_texture(&Path::new("img/mage.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/golem.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/spider1.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/spider2.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/spider3.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/unicorn.png")).unwrap());
	//ground layer
	textures.push(renderer.load_texture(&Path::new("img/grass.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/sand.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/golem_altar.png")).unwrap());
	// Projectiles
	textures.push(renderer.load_texture(&Path::new("img/projectile.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/broken_soul.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/weak_soul.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/strong_soul.png")).unwrap());
	textures.push(renderer.load_texture(&Path::new("img/nothing.png")).unwrap());
	//test
	let default_builder = projectile::ProjectileBuilder{m_texture: &textures[9], m_speed: 8.0, m_damage: 10.0, m_life_time: 300};
	let mob_proj_builder = projectile::ProjectileBuilder{m_texture: &textures[13], m_speed: 8.0, m_damage: 20.0, m_life_time: 20};
	
	let player = player::Player::new( math::Vector{x : 10.0, y : 10.0}, &textures[0], &default_builder,
											vec![&textures[10], &textures[11], &textures[12]]);

	
	let mut world = world::World::new(vec![&textures[6], &textures[7], &textures[8]],
									  vec![&textures[10], &textures[11], &textures[12]], player);
									  
	//spawners
	let actor_builder = spawner::ActorBuilder::new(&textures[2], 50.0, &mob_proj_builder, 1.5);
	world.m_spawners.push(spawner::Spawner{m_actor_builder : actor_builder, m_location: math::Vector{x : 0.0, y : 0.0}, m_cool_down: 2, m_cool_down_max: 240, m_wants_to_spawn : false});
	//the ai
	let controller = controller::Controller{m_speed : 4.0};
	world.m_controllers.push(controller);
	
	while unsafe{running} {
		let mut time = time::precise_time_ns();
	
		let mut event_pump = sdl_context.event_pump().unwrap();
		
		// Handle all sdl events.
		unsafe{left_mouse_down = false;}
		for event in event_pump.poll_iter() {
			handle_event(event);
		}
		playerinput::handle_player_input(&sdl_context, &event_pump.keyboard_state(), &mut world, unsafe{left_mouse_down});
		
		// The camera is always attached to the player which is entity 0
		{
			let player_pos = &world.m_player.m_actor.m_sprite.m_location;
			let cam_pos = Point::new(player_pos.x as i32 - WIN_WIDTH/2, player_pos.y as i32 - WIN_HEIGHT/2);
		
		
			renderer.set_draw_color(pixels::Color::RGB(0,0,0));
			renderer.clear();
			(&world as &drawable::Drawable).draw(&mut renderer, &cam_pos);
		}
		(&mut world as &mut actor::Dynamic).process();
		renderer.present();
		
		//cap frame time
		time = time::precise_time_ns() - time;
		//16666666
		if time < 16666666 { std::thread::sleep(std::time::Duration::from_millis((16666666 - time)/1000000));}
	}
}