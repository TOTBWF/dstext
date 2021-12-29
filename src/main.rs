extern crate sdl2;

use std::env;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::rotozoom::RotozoomSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn run(msg : &str) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window("Dark Souls", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let font_bytes = include_bytes!("../resources/EBGaramond-Regular.ttf");
    let font_rw = sdl2::rwops::RWops::from_bytes(font_bytes)?;
    let font = ttf_context.load_font_from_rwops(font_rw, 64)?;

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(msg)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();


    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;


    // Play the dark souls death sound.
    let _audio_subsys = sdl_context.audio()?;
    let frequency = 44_100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3);
    sdl2::mixer::allocate_channels(1);
    let music_buffer = include_bytes!("../resources/ds_death.mp3");
    let music = sdl2::mixer::Music::from_static_bytes(music_buffer)?;
    music.play(1)?;

    let mut scale = 1.0;
    'mainloop: loop {
	if scale < 1.5 {
	    scale += 0.0001;
	}
	let texture = texture_creator
	    .create_texture_from_surface(&surface.zoom(scale, scale, true)?)
	    .map_err(|e| e.to_string())?;
	let TextureQuery { width, height, .. } = texture.query();
	let target = get_centered_rect(
	    width,
	    height,
	    SCREEN_WIDTH - padding,
	    SCREEN_HEIGHT - padding,
	);
	canvas.clear();
	canvas.copy(&texture, None, Some(target))?;
	canvas.present();
	for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
	run("Argument Forgotten")?;
	Ok(())
    } else {
	let msg = &args[1];
	run(msg)?;
	Ok(())
    }
}
