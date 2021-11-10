# raytracer-in-a-weekend
<div align="center">
    A parallelized ray tracer in under 600 lines of rust inspired by the "Raytracer in a Weekend" book.
    <br></br>
    <img src="https://user-images.githubusercontent.com/64982761/141188184-c11d37f4-b0ba-4637-935e-675ddf054cc0.png"
         alt="Final scene featuring spheres of 3 different materials">
    <br>
    <p align="center">
    <em>Final scene with Lambertian, Metallic and Dielectric spheres 1200x800</em>
    </p>
    <br>
</div>

["Raytracing in one weekend"](https://github.com/RayTracing/raytracing.github.io/blob/master/books/RayTracingInOneWeekend.html)
is an introductory book on Computer Graphics where you learn graphics concepts through a
practical approach - building a working ray tracer! The book is fairly straightforward, which
is impressive because it already leads to some amazing results and renders. As someone very
curious about how computer graphics can accomplish such realistic results, i strongly
recommend this book to find out how!

Perhaps the biggest challenge i faced was that the book heavily utilizes concepts of OOP in C++
which aren't as natural / idiomatic in Rust. Nevertheless, my implementation closely resembles
the one in the book, apart from a function or two (i.e: i changed the main ray color function
from recursive to iterative for better memory performance).

## Improving the raytracer

After the ray tracer itself was ready, i used rust's [`image`](https://github.com/image-rs/image)
library to output the scenes in `.png` format and also parallelized the main pixel-sampling hot
loop with rust's amazing [`rayon`](https://github.com/rayon-rs/rayon) library.

This parallelization led to a 2x speedup (2hrs to 1hr) in my 2-core notebook, which theoretically
(as in, *untested*) is an *number of cores you have* speedup.

```rust
// Before rayon: write_color prints the pixel in PPM to stdout (piped to a file)
for i in 0..IMG_WIDTH as usize {
    let mut pixel = color::BLACK;

    for _ in 0..PIXEL_SAMPLES {
        let u = (i as f64 + rng.f64()) / (IMG_WIDTH - 1.0);
        let v = (j as f64 + rng.f64()) / (IMG_HEIGHT - 1.0);
        
        let ray = cam.gen_ray(u, v);
        pixel += Color::diffuse_ray(&ray, &world, MAX_RAY_BOUNCES);
    }

    pixel.write_color(PIXEL_SAMPLES);
}

// After using rayon: row is an iterator over a row of pixels in the image
row.par_bridge().for_each(|(i, img_pixel)| {
    let mut pixel = color::BLACK;

    let rng = fastrand::Rng::new();     // Thread local rng
    for _ in 0..PIXEL_SAMPLES as usize {
        let u = (i as f64 + rng.f64()) / (IMG_WIDTH - 1.0);
        let v = (j as f64 + rng.f64()) / (IMG_HEIGHT - 1.0);
        
        let ray = cam.gen_ray(u, v);
        pixel += world.find_ray_color(ray, MAX_RAY_BOUNCES);
    }

    *img_pixel = pixel.to_rgb(PIXEL_SAMPLES as f64);
})
```

After some profilling, one nice thing left to do would be removing all dynamic dispatch in
the code (matching on variants, rather than using objects with vtables). This will probably bring
another speedup, but may make it more difficult to follow the future books in the series,
[The next week](https://raytracing.github.io/books/RayTracingTheNextWeek.html) and 
[The rest of you life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
which i pretend to do sometime.

## Installation / Usage - trying this out for yourself
If you want to play with this code a little, it's properly commented and should be easy to
make a custom scene. The `random_scene` at the end of `main.rs` function is where the scene
you saw above was instantiated, and has the following snippet where the big spheres are
instantiated:

```rust
// Instantiate the big glass sphere.
let material = Arc::new(Dielectric::new(1.5));
let sphere = Sphere::new(0.0, 1.0, 0.0, 1.0, material);
world.add(sphere);

// Instantiate the big brown sphere.
let albedo = Color::new(0.4, 0.2, 0.1);
let material = Arc::new(Lambertian::new(albedo));
let sphere = Sphere::new(-4.0, 1.0, 0.0, 1.0, material);
world.add(sphere);

// Instantiate the big metallic sphere.
let albedo = Color::new(0.7, 0.6, 0.5);
let fuzz = 0.0;
let material = Arc::new(Metal::new(albedo, fuzz));
let sphere = Sphere::new(4.0, 1.0, 0.0, 1.0, material);
world.add(sphere);
```

Then, to run this you'll need `cargo` and `rustc`.
```bash
# You can either git clone (and change the code to make different scenes)
git clone https://github.com/gabrielvictorcf/raytracer-in-a-weekend.git
cd raytracer-in-a-weekend
cargo run --release -- <out_image_path>

# Or just install through cargo and run it (always output the same image!)
cargo install --branch main --git https://github.com/gabrielvictorcf/raytracer-in-a-weekend rtw
rtw <out_image_path>
```
