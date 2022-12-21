use hangul_jaso::*;
use image::io::Reader as ImageReader;
use jaso_sdl2::*;
use sdl2::event::*;
use sdl_isometric::ascii::AsciiImage;

fn main() -> Result<(), String> {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("GFX", 800, 600).opengl().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut event_pump = context.event_pump().unwrap();
    let mut eng_font = AsciiFonts::default();
    let mut kor_font = KoreanFonts::default();

    let eng_img_font =
        ImageReader::open("assets/bitmap_fonts/ascii-light.png").unwrap().decode().unwrap();

    let han_img_font = ImageReader::open("assets/bitmap_fonts/hangul-dkby-dinaru-2.png")
        .unwrap()
        .decode()
        .unwrap();

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

    // 텍스트 생성해봄..
    let hacker_img = ImageReader::open("assets/tie.jpg").unwrap().decode().unwrap();
    let hacker_ascii = AsciiImage::new(&hacker_img, 10, 20);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0));
        canvas.clear();

        let text = "This text. 다람쥐쳇바퀴돌리고파힣";
        let mut x_target = 100;
        let mut y_target = 100;

        for c in text.to_string().chars() {
            let code = utf8_to_ucs2(&c).unwrap();
            let lang = ucs2_language(code);

            match lang {
                Languages::Ascii => {
                    draw_ascii_font(
                        &eng_font,
                        &mut canvas,
                        x_target,
                        y_target,
                        &c,
                        &(255, 150, 150, 255),
                        &(0, 0, 0, 0),
                    );
                    x_target += 8;
                }
                Languages::Hangul => {
                    draw_kor_font(
                        &kor_font,
                        &mut canvas,
                        x_target,
                        y_target,
                        &c,
                        &(0, 255, 0, 255),
                        &(0, 0, 0, 0),
                    );
                    x_target += 16;
                }

                _ => {}
            }
        }

        x_target = 160;
        y_target = 160;

        for h in 0..hacker_ascii.height {
            for c in hacker_ascii
                .ascii_image
                .get((h * hacker_ascii.width) as usize..((h + 1) * hacker_ascii.width) as usize)
                .unwrap()
                .to_string()
                .chars()
            {
                let code = utf8_to_ucs2(&c).unwrap();
                let lang = ucs2_language(code);

                match lang {
                    Languages::Ascii => {
                        draw_ascii_font(
                            &eng_font,
                            &mut canvas,
                            x_target,
                            y_target,
                            &c,
                            &(255, 255, 255, 255),
                            &(0, 0, 0, 0),
                        );
                        x_target += 8;
                    }
                    Languages::Hangul => {
                        draw_kor_font(
                            &kor_font,
                            &mut canvas,
                            x_target,
                            y_target,
                            &c,
                            &(0, 255, 0, 255),
                            &(0, 0, 0, 0),
                        );
                        x_target += 16;
                    }

                    _ => {}
                }
            }
            x_target = 160;
            y_target += 16;
        }

        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
