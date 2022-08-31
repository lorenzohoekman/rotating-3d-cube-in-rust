use crate::width;


static K1: f32 = 40.0;

struct Cube{
    // Hold values for the next frame
    a: f32,
    b: f32,
    c: f32,

    // Current coordinates
    x: f32,
    y: f32,
    z: f32,

    ooz: f32,
}

impl Cube{

    fn calculate_x(&self, i: &f32, j: &f32,  k: &f32 ) -> f32{
        let a = &self.a; let b = &self.b; let c = &self.c;

        j * a.cos() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
        j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * b.cos()
    }

    fn calculate_y(&self, i: &f32, j: &f32, k: &f32) -> f32{
        let a = &self.a; let b = &self.b; let c = &self.c;

        j * a.cos() * c.cos() + k * a.sin() * c.cos() - 
        j * a.sin() * b.sin() * c.sin() + k * a.cos() * b.sin() * c.sin() - 
        i * b.cos() * c.sin()
    }

    fn calculate_z(&self, i: &f32, j: &f32, k: &f32) -> f32{
        let a = &self.a; let b = &self.b; let c = &self.c;

        k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
    }

    pub fn calculate_for_surface(&self, cube_x: &f32, cube_y: &f32, cube_z: &f32, ch: i32){
        self.x = self.calculate_x(cube_x, cube_y, cube_z);
        self.y = self.calculate_y(cube_x, cube_y, cube_z);
        self.z = self.calculate_z(cube_x, cube_y, cube_z);

        self.ooz = 1.0 / self.z;

        let xp = width / 2 + horizontalOffset + K1 * self.ooz * self.x * 2.0;
    }


}