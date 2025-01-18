# Raytracer in Rust

Sphere Full (1000x562)             |  Sphere + Pyramid Full (1000x562)
:-------------------------:|:-------------------------:
![Screenshot 2025-01-17 181334](https://github.com/user-attachments/assets/8765761f-e6fa-482a-a848-6ca10aee75e7)  |  ![Screenshot 2025-01-18 132832](https://github.com/user-attachments/assets/d566ce82-a134-4d1c-ab5b-be774cf2b2fc)


https://github.com/user-attachments/assets/00677c01-979c-4b44-be94-6e1eff54ac57


*400 x 225*


Still trying to learn Rust and thought this would be a great way to crate (pun intended) stuff using
Rust but also learn a bit more about graphics programming. Good portion of the work learned from 
the classic [Raytracing In A Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) article.

Currently, it works pretty well, as you can see from the above photo. We have our Spheres with both diffuse and
metallic material, as well as the shadows underneath each. I have implemented the Triangle class and in a couple
of the animations below I have shown them being rotated around, however I want to also apply those materials
to the triangle to see how it turns out. 

Additionally, it is very slow to write to ppms, so perhaps I'd like to take into account some degree of 
asynchronous raytracing, where on writes, it could jump to another raytracing tasks. Another possibility
is to have several threads where each thread handles a subset of pixels in the image and async program those 
threads. Not suspecting any sort of race conditions because nothing is being mutated within the scene, but
who knows.

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


https://github.com/user-attachments/assets/361ead82-3aae-4eff-92b7-07c4f98eeaa7


4. ..... (maybe I'll do more; we'll see)

## For Converting Lump PPM to MP4
````
ffmpeg -framerate 30 -i output%03d.ppm -vf "scale=400:224" -c:v libx264 -pix_fmt yuv420p ../examples/output3.mp4
````

# Updates
1) Vecs and Rays
2) Basic raytracing (no shaders)
3) Camera animations + rotations
4) Diffuse material + Higher Resolution (years to render) --> I could try and do some async programming.
   

https://github.com/user-attachments/assets/3eb30413-b2e4-425a-bc69-7fb47deebf55


5) Colors + Lambertian Materials!


https://github.com/user-attachments/assets/aa0447b1-71fc-43d8-b13b-1f3824d31ab5

6) Metallica

![Screenshot 2025-01-17 162626](https://github.com/user-attachments/assets/96f8bfc4-644b-4593-8b47-850d6c129266)

