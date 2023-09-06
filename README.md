# Simple Game Engine written in Rust with wgpu
This is the engine I am using for my graphics programs, like an upcoming machine learning visualization tool I am working on making. If you want to see how to make your own, check out [Learn Wgpu](https://sotrh.github.io/learn-wgpu/), and then you can look at my blog post here to turn it into an abstract engine that you can reuse.

## Features
- [x] GPU Rendering
- [x] Model Loading
- [ ] Textures
- [ ] Instances
- [ ] UI
- [ ] Custom Camera
- [ ] Lighting
- [ ] Normal Maps
- [ ] Mesh Construction

## How to use
I didn't really use many outside resources when writing this, but I decided to make my own version of ECS I call ESCS (Entity, Species, Class, System). A species is like a traditional entity, then you can instantiate them with the new entity, which has a species, position, and rotation. This allows for instancing and faster rendering.


## Possible features
- [ ] Terrain Generation
- [ ] Compute Shaders
- [ ] Shadows
- [ ] Physics
- [ ] ECS
- [ ] Culling

## Things I wish I could implement
- [ ] AI Pathfinding
- [ ] GUI Editor
