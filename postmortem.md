# Reflection

## Code

1. [Experimental ecs library](https://github.com/geng-engine/ecs)

Basic usage is pretty nice.

Serialization is not implemented.

Combining queries over multiple entity types could be more ergonomic.
They also require fields to have the same name, which is not great.

Mutable queries are not convenient.

Nested structofs could be useful.

**Note:** am I reinventing lenses LUL

2. Boilerplate

- Angles/rotation
- Level editor
- Render
- UI

## Git Log

### TLDR

1. Movement, collisions, lighting, visibility.
2. Health, score, editor, level, textures.
3. Visuals, level balance, polish, music.

### Day 1
16:28 - 01:38

- (16:41) Setup
- (17:21) Copypaste lighting
- Dinner
- (19:05) Obstacles
- (19:20) Attach lights to obstacles
- (20:03) Simple movement
- (21:23) Colliders with rotations via [parry2d](https://docs.rs/parry2d)
- (21:30) Collisions
- (21:42) Better controller
- (21:53) More obstacles
- (22:08) Shadows
- (00:55) Visibility indicator and coefficient
- (01:38) Small fixes

### Day 2
11:27 - 23:15

- (11:27) Update visibility indicator
- (12:40) (De)Serializing the level
- Lunch
- (13:40) Update CI
- (13:59) Waypoints
- (15:08) Obstacle path
- (15:26) Obstacles moving along the path
- (16:10) Obstacles turn towards the target
- (16:46) Player health
- (17:08) Score
- Dinner
- (18:38) More waypoints
- (20:21) Editor
- (20:28) Camera interpolation
- (20:50) Loading the level from file
- (21:02) Update level
- (22:37) Car, bike, and wall textures
- (22:59) Fix dynamic shadows
- (23:15) Waypoint texture
 
### Day 3
12:25 - 02:31

- (12:25) Lamps
- (12:41) Particles
- (12:51) Visibility indicator is now red
- Lunch
- (14:18) Props
- (14:55) Road prop
- (15:20) Background texture
- idk what I was doing at this time
- (16:08) Circle around waypoints
- Dinner and Playtesting @DaivyIsHere's game
- (18:34) Duplicate visibility indicator on top of the player
- (18:53) Arrow points towards the waypoint
- (19:48) Flickering lamp
- (20:33) Harder level
- (20:38) More damage
- (21:10) Daivy drone
- (21:29) More lights
- (21:33) Remove health regeneration
- (22:00) Low health effect
- (22:25) Bounce sfx and particles
- (22:32) Hurt sfx
- (23:02) Buildings have light normals
- (23:28) Draw buildings with a shader
- (23:33) Deliver and death sfx
- (23:44) Score particles
- (00:39) Difficulty increase over time
- (00:47) Disable respawn
- (01:13) Death screen
- (01:20) Animate waypoint arrow
- (02:22) Music
- (02:31) Deploy
