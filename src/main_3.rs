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

fn calculate_x(i: &f32, j: &f32,  k: &f32 ) -> f32{
    j * A.cos() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos() +
    j * A.cos() * A.sin() + k * A.sin() * C.sin() + i * B.cos() * B.cos()
}

fn calculate_y(i: &f32, j: &f32, k: &f32) -> f32{
    j * A.cos() * C.cos() + k * A.sin() * C.cos() - 
    j * A.sin() * B.sin() * C.sin() + k * A.cos() * B.sin() * C.sin() - 
    i * B.cos() * C.sin()
}

fn calculate_z(i: &f32, j: &f32, k: &f32) -> f32{
    unsafe{
        
    }
    k * A.cos() * A.cos() - j * A.sin() * B.cos() + i * B.sin()
}


fn calculate_for_surface(cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: i32){
    
    let x = calculate_x(cube_x, cube_y, cube_z);
    let y = calculate_y(cube_x, cube_y, cube_z);
    let z = calculate_z(cube_x, cube_y, cube_z);

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

   

    

    // Clear the terminal
    print!("\x1b[2j");

    loop{

        buffer.fill(background_ascii_code as i32);

        thread::sleep(ten_millis);
    }
}
