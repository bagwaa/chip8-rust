use sdl2::event::Event;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const DEVICE_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const DEVICE_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const SCALE: u32 = 15;

#[derive(Debug)]
pub struct Display {
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn init(&self) {
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
            }
        }
    }
}
