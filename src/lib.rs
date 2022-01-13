#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
mod image;
use image::BOTAN;

struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
struct Vector {
    pub start: Point,
    pub end: Point,
}

impl Vector {
    const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

const CURSOR_SIZE: u8 = 4;
const MOUSE_CURSOR: [u8; 2] = [0b10010110, 0b01101001];
#[no_mangle]
fn start() {
    unsafe {
        //*PALETTE = [0x9775a6, 0x683a68, 0x412752, 0x2d162c];
        //*PALETTE = [0x2d162c, 0x412752, 0x683a68, 0x9775a6];
        *PALETTE = [0x000000, 0x444444, 0x888888, 0xbbbbbb];
    }
    unsafe { *DRAW_COLORS = 0x4320 }

    let factor = 3;
    for (i, pixel) in unsafe { BOTAN.iter_mut().enumerate() } {
        *pixel = (i / 20) as u16;
    }
    for y in 0..SCREEN_SIZE - 1 {
        for x in 1..SCREEN_SIZE - 1 {
            let idx: usize = index(x, y);
            unsafe {
                let r: u16 = BOTAN[idx];

                let new_r: u16 = ((r * factor) as f32 / 255.) as u16 * (255 / factor);
                let err_r: u16 = r - new_r;

                BOTAN[index(x + 1, y)] += (err_r as f32 * 7. / 16.) as u16;
                BOTAN[index(x - 1, y + 1)] += (err_r as f32 * 3. / 16.) as u16;
                BOTAN[index(x, y + 1)] += (err_r as f32 * 5. / 16.) as u16;
                BOTAN[index(x + 1, y + 1)] += (err_r as f32 * 1. / 16.) as u16;
            }
        }
    }
}

// 160x160
const CENTER: Point = Point::new(80.0, 80.0);
// rect, oval, line, text
#[no_mangle]
fn update() {
    let mouse = unsafe { (*MOUSE_X, *MOUSE_Y) };
    let mouse_pressed = unsafe { *MOUSE_BUTTONS & MOUSE_LEFT };
    let mouse_pressed_right = unsafe { *MOUSE_BUTTONS & MOUSE_RIGHT };

    if mouse_pressed != 0 {
        let offset = (CURSOR_SIZE / 2) as i16;

        let x: i32 = (mouse.0 - offset).into();
        let y: i32 = (mouse.1 - offset).into();
        blit(
            &MOUSE_CURSOR,
            x,
            y,
            CURSOR_SIZE.into(),
            CURSOR_SIZE.into(),
            BLIT_1BPP,
        );
    }

    for x in 0..SCREEN_SIZE {
        for y in 0..SCREEN_SIZE {
            unsafe {
                /*
                *DRAW_COLORS = (BOTAN[index(x, y)] * 4 / TRESHHOLD as u16)
                    + ((BOTAN[index(x, y)] * 3 / TRESHHOLD as u16) << 1)
                    + ((BOTAN[index(x, y)] * 2 / TRESHHOLD as u16) << 2)
                    + ((BOTAN[index(x, y)] * 1 / TRESHHOLD as u16) << 3);
                    */
                *DRAW_COLORS = ((BOTAN[index(x, y)]) / 85) as u16;
            }

            pixel(x, y);
        }
    }
}

fn index(x: u32, y: u32) -> usize {
    (x + y * SCREEN_SIZE) as usize
}

fn pixel(x: u32, y: u32) {
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let palette_color: u8 = (*DRAW_COLORS & 0xf) as u8;
        if palette_color == 0 {
            // Transparent
            return;
        }
        let color = (palette_color - 1) & 0b11;

        let framebuffer = FRAMEBUFFER.as_mut().expect("fb ref");

        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}

fn line2(l: Vector) {
    line(
        l.start.x as i32,
        l.start.y as i32,
        l.end.x as i32,
        l.end.y as i32,
    );
    line(
        l.start.x as i32 + 1,
        l.start.y as i32,
        l.end.x as i32 + 1,
        l.end.y as i32,
    );
    line(
        l.start.x as i32,
        l.start.y as i32 + 1,
        l.end.x as i32,
        l.end.y as i32 + 1,
    );
}

fn circle(pos: Point, radius: i32) {
    oval(
        (pos.x - radius as f32 / 2.0) as i32,
        (pos.y - radius as f32 / 2.0) as i32,
        radius.try_into().unwrap(),
        radius.try_into().unwrap(),
    );
}
