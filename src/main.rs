use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use image::Pixel;
use sdl2::event::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::*;

type HalfFont = [u8; 16];
type FullFont = [u16; 16];

#[derive(Default)]
struct AsciiFonts {
    fonts: Vec<Vec<u32>>,
}

fn image2hex(img: &DynamicImage, x: u32, y: u32, w: u32, h: u32) -> Vec<u32> {
    let mut rows = vec![];
    for j in y..(y + h) {
        let mut cell: u32 = 0;
        for i in x..(x + w) {
            let digit: u32 = (w as i32 - i as i32 + x as i32) as u32 - 1;

            let v = if (*img).get_pixel(i, j).0[3] == 0 {
                0
            } else {
                2_u32.pow(digit)
            };
            cell = cell + v;
        }
        rows.push(cell);
    }

    rows
}

fn draw_font(
    font: &AsciiFonts,
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    x: i32,
    y: i32,
    contents: &String,
) {
    let mut texture = texture_creator
        .create_texture_target(
            texture_creator.default_pixel_format(),
            8 * contents.len() as u32,
            16,
        )
        .unwrap();

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            texture_canvas.clear();
            let mut start_pos = 0 as i16;
            for s in contents.chars() {
                for j in 0..16_i16 {
                    let row = font.fonts[s as usize][j as usize];
                    let mut rem = row;
                    for i in 0..8_i16 {
                        let exp = (8_i32 - i as i32 - 1) as u32;
                        let divider = 2_u32.pow(exp);
                        let v = rem / divider;
                        rem = rem - v * divider;

                        if v > 0 {
                            texture_canvas.pixel(start_pos + i, j, 0xFF00FFFFu32);
                        }
                    }
                }

                start_pos = start_pos + 8;
            }
        })
        .unwrap();

    canvas.copy_ex(
        &texture,
        Rect::new(0, 0, 8 * contents.len() as u32, 16),
        Rect::new(x, y, 8 * contents.len() as u32, 16),
        0.0,
        Point::new(0, 0),
        false,
        false,
    );
}

fn main() -> Result<(), String> {
    let context = init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("GFX", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = context.event_pump().unwrap();
    let mut eng_font = AsciiFonts::default();

    let img = ImageReader::open("assets/bitmap_fonts/ascii-light.png")
        .unwrap()
        .decode()
        .unwrap();

    // 영문 가로 8글자, 세로 16글자, 각 글자는 8x16
    for y in 0..8 {
        for x in 0..16 {
            let rows = image2hex(&img, x * 8, y * 16, 8, 16);
            eng_font.fonts.push(rows);
        }
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        draw_font(
            &eng_font,
            &mut canvas,
            &texture_creator,
            100,
            100,
            &"Hello World 123!@#$@#%#$^W$&".to_string(),
        );

        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
