
// generate ppm image
fn gen_ppm(width: u64, height: u64) {

    print!("P3\n{} {}\n255\n", width, height);

    for j in (1..height + 1).rev() {
        for i in 0..width {
            let red = (i as f64) / ((width - 1) as f64);
            let green = (j as f64) / ((height - 1) as f64);
            let blue = 0.25;
    
            const MULT: f64 = 255.999;
            let i_red = (red * MULT) as u64;
            let i_green = (green * MULT) as u64;
            let i_blue = (blue * MULT) as u64;

            print!("{} {} {}\n", i_red, i_green, i_blue);
        }
    }

}


fn main() {
  const HEIGHT: u64  = 256;
  const WIDTH: u64 = 256;

  gen_ppm(WIDTH, HEIGHT);
}
