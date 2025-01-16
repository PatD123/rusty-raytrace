# Raytracer in Rust...with some other stuff

Still trying to learn Rust and thought this would be a great way to crate (pun intended) stuff using
Rust but also learn a bit more about graphics programming. Good portion of the work learned from 
the classic [Raytracing In A Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) article.

Currently, it works pretty decently as you can see from below, but obviously it barely has any shaders, it
doesn't take lighting into effect. These will be worked on later. But this was supposed to be a very barebones 
attempt to get Raytracing working with Rust.

## Features
### 3d Vectors (Vec 3)
Support for basic vector implementations, including dot product, cross product, rotations (just used for 
camera animations). Additionally, I didn't feel like making Vectors any different than Points (could've done
homegenous coordinates), but they were practically the same so for example, when defining rays, the origin of
the ray is a Vec3 and the direction is also a Vec3.

### Rays
I also provide a basic ray implementation, of course used in raytracing.
At current moment of development, it is able to shoot rays out and render objects in our scene. 
As stated above, created from the Vec3 struct.

### Camera
Supports camera animation and rendering.


https://github.com/user-attachments/assets/8f268893-1ed7-460f-aa4a-1d4242a6dc44



https://github.com/user-attachments/assets/17686b5b-6eda-450e-8af1-f2754e946358

For both above, the camera rotates around the sphere. There's not that many objects nor has 
the best lighting to provide an explicit perspective. But it is rotating around the sphere.

### Shapes
1. Spheres
2. Triangles [ray-triangle insided-ness](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html)
3. ..... (maybe I'll do more; we'll see)

## For Converting Lump PPM to MP4

ffmpeg -framerate 30 -i output%03d.ppm -vf "scale=400:224" -c:v libx264 -pix_fmt yuv420p ../examples/output3.mp4
