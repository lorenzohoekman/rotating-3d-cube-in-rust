use std::{thread, time::{self, Duration}, io::{stdout, Write}};

static CUBE_WIDTH: f32 = 20.0;
static WIDTH: i32 = 160;
static HEIGHT: i32 = 44;
static BACKGROUND_ASCII_CODE: char = '.';
static DISTANCE_FROM_CAM:i32 = 100;
static K1: f32 = 40.0;
static TEN_MILLIS: Duration = time::Duration::from_millis(1000);

static mut A: f32 = 0.;
static mut B: f32 = 0.;
static mut C: f32 = 0.;

fn calculate_x(
    i: &f32, j: &f32,  k: &f32, 
    //a: &f32, b: &f32, c: &f32 
) -> f32{
    //let a = unsafe {A.clone()};
    //let b = unsafe {B.clone()};
    //let c = unsafe {C.clone()};

    unsafe {
        j * A.sin() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos() +
        j * A.cos() * C.sin() + k * A.sin() * C.sin() + i * B.cos() * B.cos()
    }

   
}

fn calculate_y(
    i: &f32, j: &f32,  k: &f32, 
    //a: &f32, b: &f32, c: &f32 
) -> f32{
    unsafe{
        j * A.cos() * C.cos() + k * A.sin() * C.cos() - 
        j * A.sin() * B.sin() * C.sin() + k * A.cos() * B.sin() * C.sin() - 
        j * B.cos() * C.sin()
    }
}

fn calculate_z(
    i: &f32, j: &f32,  k: &f32, 
    //a: &f32, b: &f32 
) -> f32{
    unsafe {
        k * A.cos() * B.cos() - j * C.sin() * B.cos() + i * B.sin()
    }
}


fn calculate_for_surface(
    cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: char,
    a: &f32, b: &f32, c: &f32,
    z_buffer: &mut [f32], buffer:&mut [char]
){
    
    let x = calculate_x(cube_x, cube_y, cube_z);
    let y = calculate_y(cube_x, cube_y, cube_z);
    let z = calculate_z(cube_x, cube_y, cube_z) + DISTANCE_FROM_CAM as f32;

    // out of zone
    let ooz = 1. / z;

    let xp = ((WIDTH / 2) as f32 +  K1 * ooz * x * 2.) as i32;
    let yp = ((HEIGHT / 2) as f32 + K1 * ooz * y) as i32;
    
    // Index
    let idx: i32 = xp + yp * WIDTH as i32 ;
    

    if idx >= 0 && idx < (WIDTH * HEIGHT) {
        
        if ooz > z_buffer[idx as usize] as f32 {
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = ch as char;
        }
    }
}
fn main() {
    
    let (mut a, mut b, mut c):  (f32, f32, f32) = (0.000000, 0.000000, 0.000000);
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
        let mut cube_x: f32 = -CUBE_WIDTH;
        let mut cube_y: f32 = -CUBE_WIDTH;

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

        unsafe {
            A += 0.05;
            B += 0.05;
            C += 0.01;
        }

        //println!("{} {} {}", &a, &b, &c);

        

        thread::sleep(TEN_MILLIS);
    }
}
