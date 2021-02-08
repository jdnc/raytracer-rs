mod vec3;
mod ray;

type Point = vec3::Vec3;
type Color = vec3::Vec3;


fn write_color(pixel: Color) {
    const MULT: f64 = 255.999;
    let i_red = (pixel.x * MULT) as u64;
    let i_green = (pixel.y * MULT) as u64;
    let i_blue = (pixel.z * MULT) as u64;

    print!("{} {} {}\n", i_red, i_green, i_blue);
}

// generate ppm image
fn gen_ppm(width: u64, height: u64) {

    print!("P3\n{} {}\n255\n", width, height);

    for j in (1..height + 1).rev() {
        eprintln!("Scan lines remaining: {}", j);
        for i in 0..width {
            let pixel = Color{
                x: (i as f64) / ((width - 1) as f64),
                y: (j as f64) / ((height - 1) as f64),
                z: 0.25,};
            write_color(pixel);
        }
    }

}


fn main() {
  const HEIGHT: u64  = 256;
  const WIDTH: u64 = 256;

  gen_ppm(WIDTH, HEIGHT);
}
