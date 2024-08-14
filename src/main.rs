mod chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::event::Event;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const DEVICE_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const DEVICE_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const SCALE: u32 = 15;

use chip8::cpu::Cpu;

fn draw_screen(emu: &Cpu, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();
    // Now set draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert our 1D array's index into a 2D (x,y) position
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn main() {
    // Create a new CPU
    let mut cpu = Cpu::new();

    // Load ROM into memory from file specified as command line argument
    let rom_path = std::env::args().nth(1).expect("ROM file not found");

    // Load ROM file into RAM starting at 0x200
    cpu.load_rom(&rom_path);

    // Run the Emulator loop
    // cpu.run();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8 Emulator", DEVICE_WIDTH, DEVICE_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'gameloop;
            }

            cpu.tick();
            draw_screen(&cpu, &mut canvas);
        }
    }
}
