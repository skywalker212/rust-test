use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
#[macro_use]
extern crate specs_derive;

struct State {
    ecs: World
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        for (position, renderable) in (&positions, &renderables).join() {
            ctx.set(position.x, position.y, renderable.fg, renderable.bg, renderable.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination = xy_id(pos.x + delta_x, pos.y + delta_y);
        if map[destination] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}

pub fn xy_id(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];
    for x in 0..80 {
        map[xy_id(x, 0)] = TileType::Wall;
        map[xy_id(x, 49)] = TileType:: Wall;
    }
    for y in 0..50 {
        map[xy_id(0, y)] = TileType::Wall;
        map[xy_id(79, y)] = TileType::Wall;
    }

    let mut bless_rng = rltk::RandomNumberGenerator::new();

    for x in 0..80 {
        for y in 0..50 {
            let binary = bless_rng.roll_dice(1, 5);
            let index = xy_id(x, y);
            if binary == 3 {
                if x != 40 && y != 25 {
                    map[index] = TileType::Wall;
                } 
            }
        }
    }

    map
}

fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "TARS", "resources");
    let mut gs =  State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position {x: 40, y: 25})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs);
}