mod chip8;

use sdl2::event::Event;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const DEVICE_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const DEVICE_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const SCALE: u32 = 15;

use chip8::cpu::Cpu;

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
        }
    }
}
