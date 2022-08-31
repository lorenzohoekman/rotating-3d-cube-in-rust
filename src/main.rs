use std::{thread, time::{self, Duration}};

mod cube;
mod main_2;

static mut A: f32 = 0.0;
static mut B: f32 = 0.0;
static mut C: f32 = 0.0;

/* static mut X: f32 = 0.0;
static mut Y: f32 = 0.0;
static mut Z: f32 = 0.0; */

static cube_width:f32 = 20.0;
static width: f32 = 160.0;
static height: f32 = 44.0;

//static mut Z_BUFFER: Vec<i32> = vec![0; height as usize * width as usize];
//static mut BUFFER: Vec<i32> = vec![0; height as usize * width as usize];
static background_ascii_code: char = '.';
static distance_from_cam:i32 = 100;
static horizontal_offset: f32 = 0.0;
static K1: f32 = 40.0;




static ten_millis: Duration = time::Duration::from_millis(10);

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
    cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: i32,
    a: &f32, b: &f32, c: &f32,
    z_buffer: &[i32], buffer:&[i32]
){
    
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b, c);

    let ooz = 1.0 / z;

    let xp = width / 2.0 + horizontal_offset + K1 * ooz * x * 2.0;
    let yp = height / 2.0 + K1 * ooz * y;
    
    let idx: usize = (xp + yp * width) as usize;
        if idx >= 0 && idx < (width * height) as usize {
        if ooz > z_buffer[idx] as f32 {
            z_buffer[idx] = ooz as i32;
            buffer[idx] = ch;
        }
    }
}
fn main() {

    let (mut a, mut b, mut c):  (f32, f32, f32);
    let mut buffer: Vec<i32> = vec![0; height as usize * width as usize];
    let mut z_buffer: Vec<i32> = vec![0; height as usize * width as usize];

    let increment_speed = 0.6;

    // Clear the terminal
    print!("\x1b[2j");

    loop{

        buffer.fill(background_ascii_code as i32);
        z_buffer.fill(0);

        cube_width = 20.0;
        horizontal_offset = -2.0 * cube_width;
        

        // First cube
        let cube_x = -cube_width;
        let cube_y = -cube_width;

        while cube_x < cube_width{
            while cube_y < cube_width{

                calculate_for_surface(
                    &cube_x, &cube_y, &-cube_width, '@' as i32, 
                    &a, &b, &c, &z_buffer, &buffer
                );
                calculate_for_surface(
                    &cube_width, &cube_y, &cube_x, '$' as i32,
                    &a, &b, &c, &z_buffer, &buffer
                );
                calculate_for_surface(
                    -cube_width, cube_y, -cube_x, '~'
                );
                calculate_for_surface(
                    -cube_x, cube_y, cube_width, '#'
                );
                calculate_for_surface(
                    cube_x, -cube_width, -cube_y, ';'
                );
                calculate_for_surface(
                    cube_x, cube_width, cube_y, '+'
                );

                cube_y += increment_speed;
            }
            cube_x  += increment_speed
        }


        thread::sleep(ten_millis);
    }
}
