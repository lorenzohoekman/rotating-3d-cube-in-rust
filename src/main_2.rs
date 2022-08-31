




fn main_2(){

    let (mut a, mut b, mut c):  (f32, f32, f32);

    let x = | i: &f32, j: &f32,  k: &f32 | {
        j * a.cos() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
        j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * b.cos()
    };
    
    
}