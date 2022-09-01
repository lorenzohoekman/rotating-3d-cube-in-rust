use std::{thread, time::{self, Duration}, io::{stdout, Write}};

//mod cube;
//mod main_2;


static WIDTH: u32 = 160;
static HEIGHT: u32 = 44;
static BACKGROUND_ASCII_CODE: char = '.';
static DISTANCE_FROM_CAM:i32 = 100;
static K1: f32 = 40.0;
static TEN_MILLIS: Duration = time::Duration::from_millis(10);

fn calculate_x(
    i: &f32, j: &f32,  k: &f32, 
    a: &f32, b: &f32, c: &f32 
) -> f32{
    j * a.cos() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
    j * a.cos() * a.sin() + k * a.sin() * c.sin() + i * b.cos() * b.cos()
}

fn calculate_y(
    i: &f32, j: &f32,  k: &f32, 
    a: &f32, b: &f32, c: &f32 
) -> f32{
    j * a.cos() * c.cos() + k * a.sin() * c.cos() - 
    j * a.sin() * b.sin() * c.sin() + k * a.cos() * b.sin() * c.sin() - 
    i * b.cos() * c.sin()
}

fn calculate_z(
    i: &f32, j: &f32,  k: &f32, 
    a: &f32, b: &f32, c: &f32 
) -> f32{
    k * a.cos() * a.cos() - j * a.sin() * b.cos() + i * b.sin()
}


fn calculate_for_surface(
    cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: char,
    a: &f32, b: &f32, c: &f32,
    z_buffer: &mut [i32], buffer:&mut [char], horizontal_offset: &f32
){
    
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b, c) + DISTANCE_FROM_CAM as f32;

    let ooz = 1.0 / z;

    let xp = ((WIDTH / 2) as f32 + horizontal_offset + K1 * ooz * x * 2.0) as i32;
    let yp = ((HEIGHT / 2) as f32 + K1 * ooz * y) as i32;
    
    let idx: usize = (xp + yp * WIDTH as i32) as usize;
        if idx as i32 >= 0 && idx < (WIDTH * HEIGHT) as usize {
        if ooz > z_buffer[idx] as f32 {
            z_buffer[idx] = ooz as i32;
            buffer[idx] = ch as char;
        }
    }
}
fn main() {

    let (mut a, mut b, mut c):  (f32, f32, f32) = (0.0, 0.0, 0.0);
    let mut buffer: Vec<char> = vec![0 as char; HEIGHT as usize * WIDTH as usize];
    let mut z_buffer: Vec<i32> = vec![0; HEIGHT as usize * WIDTH as usize];

    let increment_speed = 0.6;

    // Clear the terminal
    print!("\x1b[2j");

    loop{

        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0);

        let mut cube_width = 20.0;
        let mut horizontal_offset = -2.0 * cube_width;
        let mut cube_x = -cube_width;
        let mut cube_y = -cube_width;

        // First cube
        while cube_x < cube_width{
            while cube_y < cube_width{

                calculate_for_surface(
                    &cube_x, &cube_y, &-cube_width, '@', 
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_width, &cube_y, &cube_x, '$',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_width, &cube_y, &-cube_x, '~',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_x, &cube_y, &cube_width, '#',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &-cube_width, &-cube_y, ';',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &cube_width, &cube_y, '+',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );

                cube_y += increment_speed;
            }
            cube_x  += increment_speed
        }

        cube_width = 10.0;
        horizontal_offset =  1.0 * cube_width;

        // Second cube
        while cube_x < cube_width{
            while cube_y < cube_width{

                calculate_for_surface(
                    &cube_x, &cube_y, &-cube_width, '@', 
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_width, &cube_y, &cube_x, '$',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_width, &cube_y, &-cube_x, '~',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_x, &cube_y, &cube_width, '#',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &-cube_width, &-cube_y, ';',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &cube_width, &cube_y, '+',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );

                cube_y += increment_speed;
            }
            cube_x  += increment_speed
        }

        cube_width = 5.0;
        horizontal_offset =  8.0 * cube_width;

        // Third cube 
        while cube_x < cube_width{
            while cube_y < cube_width{

                calculate_for_surface(
                    &cube_x, &cube_y, &-cube_width, '@', 
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_width, &cube_y, &cube_x, '$',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_width, &cube_y, &-cube_x, '~',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &-cube_x, &cube_y, &cube_width, '#',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &-cube_width, &-cube_y, ';',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );
                calculate_for_surface(
                    &cube_x, &cube_width, &cube_y, '+',
                    &a, &b, &c, &mut z_buffer, &mut buffer, &horizontal_offset
                );

                cube_y += increment_speed;
            }
            cube_x  += increment_speed
        }

        print!("\x1b[H");

        for k in 0..(WIDTH * HEIGHT) as i32{



            let c: char = if (k % WIDTH as i32) != 0 {

                let k_convert = char::from_u32(*&k as u32).unwrap();

                buffer[k_convert as usize] as char
            } else  {
                10 as char
            };
            print!("{}", c);
            stdout().flush().unwrap();
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;

        thread::sleep(TEN_MILLIS);
    }
}
