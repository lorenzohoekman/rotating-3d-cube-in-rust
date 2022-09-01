use std::{thread, time::{self, Duration}, io::{stdout, Write}};

static CUBE_WIDTH: f32 = 20.0;
static WIDTH: i32 = 160;
static HEIGHT: i32 = 44;
static BACKGROUND_ASCII_CODE: char = ' ';
static DISTANCE_FROM_CAM:i32 = 100;
static K1: f32 = 40.0;
static TEN_MILLIS: Duration = time::Duration::from_millis(100);

fn calculate_x(
    i: &f32, j: &f32,  k: &f32, 
    a: &f32, b: &f32, c: &f32 
) -> f32{
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
    j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * b.cos()
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
    a: &f32, b: &f32 
) -> f32{
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}


fn calculate_for_surface(
    cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: char,
    a: &f32, b: &f32, c: &f32,
    z_buffer: &mut [f32], buffer:&mut [char]
){
    
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM as f32;

    // out of zone
    let ooz = 1. / z;

    let xp = ((WIDTH / 2) as f32 +  K1 * ooz * x * 2.) as i32;
    let yp = ((HEIGHT / 2) as f32 + K1 * ooz * y) as i32;
    
    // Index
    let idx: i32 = xp + yp * WIDTH as i32 ;
    

    /*  let x = -20.0;
    let z = 80.;
    let y = -20.;
    let xp = 20;
    let yp = 12;
    let idx = 1940;
    let ooz= 0.012500;  */

    if idx >= 0 && idx < (WIDTH * HEIGHT).try_into().unwrap() {
        
        if ooz > z_buffer[idx as usize] as f32 {
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = ch as char;
        }
    }
}
fn main() {

    let (mut a, mut b, mut c):  (f32, f32, f32) = (0., 0., 0.);
    let mut buffer: Vec<char> = vec![0 as char; (HEIGHT * WIDTH) as usize];
    let mut z_buffer: Vec<f32> = vec![0.; (HEIGHT * WIDTH * 4)as usize];

    let increment_speed = 0.6;

    // Clear the terminal
    print!("\x1b[2j");

    loop{
        // Fill the buffers
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.);   

        // 
        let mut cube_x = -CUBE_WIDTH;
        let mut cube_y = -CUBE_WIDTH;

        // Calculating cube
        while cube_x < CUBE_WIDTH{
            while cube_y < CUBE_WIDTH{

                calculate_for_surface(
                    &cube_x, &cube_y, &-CUBE_WIDTH, '@', 
                    &a, &b, &c, &mut z_buffer, &mut buffer
                );
                calculate_for_surface(
                    &CUBE_WIDTH, &cube_y, &cube_x, '$',
                    &a, &b, &c, &mut z_buffer, &mut buffer
                );
                calculate_for_surface(
                    &-CUBE_WIDTH, &cube_y, &-cube_x, '~',
                    &a, &b, &c, &mut z_buffer, &mut buffer
                );
                calculate_for_surface(
                    &-cube_x, &cube_y, &CUBE_WIDTH, '#',
                    &a, &b, &c, &mut z_buffer, &mut buffer
                );
                calculate_for_surface(
                    &cube_x, &-CUBE_WIDTH, &-cube_y, ';',
                    &a, &b, &c, &mut z_buffer, &mut buffer
                );
                calculate_for_surface(
                    &cube_x, &CUBE_WIDTH, &cube_y, '+',
                    &a, &b, &c, &mut z_buffer, &mut buffer
                ); 

                cube_y += increment_speed;
            }
            cube_x  += increment_speed
        }

        // Clear the terminal
        print!("\x1b[H");
        stdout().flush().unwrap();

        let mut k = 0;

        while k < WIDTH * HEIGHT {

            let c: char = if (k % WIDTH ) != 0 {
                buffer[k as usize]
            } else  {
                10 as char
            };
            print!("{}", c);
            stdout().flush().unwrap();

            k += 1;
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;

        thread::sleep(TEN_MILLIS);
    }
}
