# Raytracer in Rust...with some other stuff

Still trying to learn Rust and thought this would be a great way to crate (pun intended) stuff using
Rust but also learn a bit more about graphics programming.

## 3d Vectors (Vec 3)

I provide basic vector operation implementations, really those that I needed to follow along with
the [Raytracing In A Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) article.

## Rays

I also provide a basic ray implementation, of course used in raytracing.
At current moment of development, it is able to shoot rays out and render
a cute lil gradient.

## For Converting Lump PPM to MP4

ffmpeg -framerate 30 -i output%03d.ppm -vf "scale=400:224" -c:v libx264 -pix_fmt yuv420p ../examples/output3.mp4
