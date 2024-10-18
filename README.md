# Bevy first week practice

<!--toc:start-->
- [Bevy first week practice](#bevy-first-week-practice)
  - [Design document first iteration](#design-document-first-iteration)
    - [Setup scene](#setup-scene)
    - [Game loop](#game-loop)
<!--toc:end-->

## Design document first iteration

### Setup scene

Add environment to the world, with the following Components:

- [ ] Directional light
- [ ] Camera

Add controls to the world, with the following Components:

- [ ] Camera Controller
- [ ] Keyboard Input

Add UI to the world, with the following Components:

- [ ] Label, that displays the current step count
- [ ] Label, that displays sim count
- [ ] Label, that displays grid size

Add level to the world, with the following Components:

- [ ] Ground, a 3D plane
- [ ] Walls, 3D boxes that are placed around the ground

### Game loop

While step_count is larger than zero, do the following:
