#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(incorrect-ident-case)]
#![allow(non-snake-case)]

// use std::thread::sleep;
// use std::time::Duration;
use std::f32::consts::PI;

#[derive(Debug)]
struct Screen {
    width: u32,
    height: u32,
    size: u32,
    buffer: Vec<char>,
}

impl Screen {
    fn new(width: u32, height: u32) -> Self {
        let size = width * height;
        let buffer: Vec<char> = vec![' '; size as usize];
        return Self {
            width,
            height,
            size,
            buffer,
        };
    }

    fn set_pixel(self: &mut Self, x: u32, y: u32, value: char) {
        self.buffer[(self.width * y + x) as usize] = value;
    }

    fn clear_buffer(self: &mut Self) {
        self.buffer = vec!['.'; self.size as usize]
    }
}

fn main() {
    let mut screen: Screen = Screen::new(100, 50);

    const R1: f32 = 1.;
    const R2: f32 = 2.;

    const THETA_SPACING: f32 = 0.07;
    const PHI_SPACING: f32 = 0.02;

    let X_OFFSET: f32 = screen.width as f32 / 2.;
    let Y_OFFSET: f32 = screen.height as f32 / 2.;

    const FRAME_RATE: u8 = 60;
    const FRAME_DELAY: f32 = 1000. / (FRAME_RATE as f32);
    const SHADES: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    let mut A: f32 = 0.;
    let mut B: f32 = 0.;
    const d_A: f32 = 0.02;
    const d_B: f32 = 0.01;

    const K2: f32 = 5.; // Distance from donut to viewer
    let K1: f32 = 25.;

    //    const shades: [char; 64] = ['.', '\'', '`', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'Y', 'U', 'J', 'C', 'L', 'Q', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

    print!("\x1b[2J");

    loop {
        screen.clear_buffer();

        // Set screen buffer
        let mut phi: f32 = 0.;
        while phi < 2. * PI {
            // Donut shape
            phi += PHI_SPACING;

            let mut theta: f32 = 0.;
            while theta < 2. * PI {
                // Small circle cross section
                theta += THETA_SPACING;

                let x: f32 = (R2 + R1 * theta.cos())
                    * (B.cos() * phi.cos() + A.sin() * B.sin() * phi.sin())
                    - R1 * A.cos() * B.sin() * theta.sin();

                let y: f32 = (R2 + R1 * theta.cos())
                    * (phi.cos() * B.sin() - B.cos() * A.sin() * phi.sin())
                    + R1 * A.cos() * B.cos() * theta.sin();

                let z: f32 =
                    A.cos() * (R2 + R1 * theta.cos()) * phi.sin() + R1 * A.sin() * theta.sin();

                let x_mapped: f32 = K1 * x / (K2 + z) + X_OFFSET;
                let y_mapped: f32 = K1 * y / (K2 + z) + Y_OFFSET;

                let L: f32 = phi.cos() * theta.cos() * B.cos() - A.cos() * theta.cos() * phi.sin() + B.cos() * (A.cos() * theta.sin() - theta.cos() * A.sin() * phi.sin());
                    

                if (x_mapped >= 0.)
                    && (y_mapped >= 0.)
                    && (x_mapped < screen.width as f32)
                    && (y_mapped < screen.height as f32)
                {
                    screen.set_pixel(x_mapped as u32, y_mapped as u32, '*');
                }
            }
        }

        print!("\x1b[H");

        // Printing screen
        for i in 0..screen.size {
            if i % screen.width == 0 {
                print!("\n");
            }

            print!("{}", screen.buffer[i as usize]);

            // let x = i % screen.width;
            // let y = i / screen.width;

            // let ch = (((x + y) as f32) / ((screen.width + screen.height) as f32)) * (SHADES.len() as f32);
            // print!("{}", SHADES[ch as usize]);
        }

        // Change angles
        A += d_A;
        if A > 2. * PI {
            A = 0.;
        }

        B += d_B;
        if B > 2. * PI {
            B = 0.;
        }
        // Breakq;
    }
}
