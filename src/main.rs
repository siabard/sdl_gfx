use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use sdl2::event::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use hangul_jaso::*;

use sdl2::*;

type HalfFont = [u8; 16];
type FullFont = [u16; 16];

#[derive(Default)]
struct AsciiFonts {
    fonts: Vec<Vec<u32>>,
}

#[derive(Default)]
struct KoreanFonts {
    cho: Vec<Vec<u32>>,
    mid: Vec<Vec<u32>>,
    jong: Vec<Vec<u32>>,
}


pub fn build_jaso_bul(t: &dyn ToString) -> (Jaso, Bul) {
    let code = utf8_to_ucs2(t).unwrap();
    let jaso = build_jaso(code).unwrap();
    let bul = build_bul(&jaso);

    (jaso, bul)
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
            cell += v;
        }
        rows.push(cell);
    }

    rows
}

fn draw_kor_font(
    font: &KoreanFonts,
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    c: &char,

) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_target(
	texture_creator.default_pixel_format(),
	16,
	16,
    ).unwrap();

    let (jaso, bul) = build_jaso_bul(c);

    let cho_hex = &font.cho[(jaso.cho + bul.cho.unwrap() * 19) as usize];
    let mid_hex = &font.mid[(jaso.mid + bul.mid.unwrap() * 21) as usize];
    let jong_hex = match bul.jong {
	Some(jong) => {
	    &font.jong[(jaso.jong + jong * 28) as usize]
	}
	_ => &font.jong[0]

    };
    canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            texture_canvas.clear();

                for j in 0..16_i16 {
                    let cho = cho_hex[j as usize];
		    let mid = mid_hex[j as usize];
		    let jong = jong_hex[j as usize];
                    for i in 0..16_i16 {
                        let vc = (cho << i) & 0x8000;
			let vm = (mid << i) & 0x8000;
			let vj = (jong << i) & 0x8000;

                        if vc > 0 {
                            texture_canvas.pixel(i, j, 0xFF00FFFFu32).unwrap();
                        }
			if vm > 0 {
                            texture_canvas.pixel(i, j, 0xFF00FFFFu32).unwrap();
                        }
			if vj > 0 {
                            texture_canvas.pixel(i, j, 0xFF00FFFFu32).unwrap();
                        }
                    }
                }


    }).unwrap();

    canvas.copy_ex(
	&texture,
        Rect::new(0, 0, 16, 16),
        Rect::new(x, y, 16, 16),
        0.0,
        Point::new(0, 0),
        false,
        false,
    ).unwrap();
}

fn draw_ascii_font(
    font: &AsciiFonts,
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    contents: &char,


) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(
            texture_creator.default_pixel_format(),
            8,
            16,
        )
        .unwrap();

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            texture_canvas.clear();

                for j in 0..16_i16 {
                    let row = font.fonts[*contents as usize][j as usize];
                    for i in 0..8_i16 {
                        let v = (row << i) & 0x80;


                        if v > 0 {
                            texture_canvas.pixel(i, j, 0xFF00FFFFu32).unwrap();
                        }
                    }
                }



        })
        .unwrap();

    canvas.copy_ex(
        &texture,
        Rect::new(0, 0, 8, 16),
        Rect::new(x, y, 8, 16),
        0.0,
        Point::new(0, 0),
        false,
        false,
    ).unwrap();
}

fn main() -> Result<(), String> {
    let context = init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("GFX", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = context.event_pump().unwrap();
    let mut eng_font = AsciiFonts::default();
    let mut kor_font = KoreanFonts::default();

    let eng_img_font = ImageReader::open("assets/bitmap_fonts/ascii-light.png")
        .unwrap()
        .decode()
        .unwrap();

    let han_img_font = ImageReader::open("assets/bitmap_fonts/hangul-dkby-dinaru-2.png").unwrap().decode().unwrap();

    // 영문 가로 16글자, 세로 8글자, 각 글자는 8x16
    for y in 0..8 {
        for x in 0..16 {
            let rows = image2hex(&eng_img_font, x * 8, y * 16, 8, 16);
            eng_font.fonts.push(rows);
        }
    }

    // 한글 가로 28글자, 세로 16글자(8,4,4), 각 글자는 16x16
    // 한글 초성 8벌 : 19 : 32*19*8 = 4864
    for y in 0..8 {
	for x in 0..19 {
	    let rows = image2hex(&han_img_font, x * 16, y * 16, 16, 16);
	    kor_font.cho.push(rows);
	}
    }
    // 한글 중성 4벌 : 21 : 32*21*4 = 2688
    for y in 8..12 {
	for x in 0..21 {
	    let rows = image2hex(&han_img_font, x * 16, y * 16, 16, 16);
	    kor_font.mid.push(rows);
	}
    }
    // 한글 종성 4벌 : 28 : 32*28*4 = 3584
    for y in 12..16 {
	for x in 0..28 {
	    let rows = image2hex(&han_img_font, x * 16, y * 16, 16, 16);
	    kor_font.jong.push(rows);
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

	let text = "This text. 가나난너녀녁";
	let mut x_target = 100;
	let y_target = 100;

	for c in text.to_string().chars() {
	    let code = utf8_to_ucs2(&c).unwrap();
            let lang = ucs2_language(code);


	    match lang {
		Languages::Ascii => {
		    draw_ascii_font(&eng_font, &mut canvas, x_target, y_target, &c);
		    x_target += 8;
		},
		Languages::Hangul => {
		    draw_kor_font(&kor_font, &mut canvas, x_target, y_target, &c);
		    x_target += 16;
		},

		
		_ => {}
	    }

	}



        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
