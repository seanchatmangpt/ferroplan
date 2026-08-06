//! No art department, no license to chase — every entity type gets a mesh cut
//! on the fly. A name heuristic reads the type and forges a shape for it; the
//! same type locks a stable colour. Meshes and materials are cached by (shape,
//! size) and by colour, so a graph runs on a handful of recycled handles.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IconShape {
    Circle,  // places / locations
    Truck,   // vehicles
    Box,     // packages / items
    Person,  // agents / people
    Robot,   // robots / rovers
    Machine, // machines / workstations (job-shop, factories)
    Diamond, // default
}

/// Read a type name, any case, and cast it into a shape — pattern-matching, not magic.
pub fn shape_for(ty: &str) -> IconShape {
    let t = ty.to_ascii_uppercase();
    let has = |words: &[&str]| words.iter().any(|w| t.contains(w));
    if has(&[
        "TRUCK", "CAR", "VEHICLE", "VAN", "LORRY", "BUS", "TRAIN", "SHIP", "BOAT", "PLANE",
        "AIRCRAFT", "AIRPLANE", "ROCKET",
    ]) {
        IconShape::Truck
    } else if has(&[
        "PACKAGE", "CRATE", "BOX", "PARCEL", "CARGO", "ITEM", "OBJ", "BALL", "BLOCK", "FUSE",
        "TILE", "STONE", "JOB", "TASK", "ORDER", "GOOD", "PRODUCT",
    ]) {
        IconShape::Box
    } else if has(&[
        "MACHINE",
        "MILL",
        "LATHE",
        "FORGE",
        "WORKSTATION",
        "STATION",
        "KILN",
        "ANVIL",
        "LOOM",
    ]) {
        IconShape::Machine
    } else if has(&[
        "PERSON",
        "AGENT",
        "DRIVER",
        "WORKER",
        "HUMAN",
        "PASSENGER",
        "ROBBY",
        "HOIST",
        "CREW",
    ]) {
        IconShape::Person
    } else if has(&["ROBOT", "ROVER", "BOT", "DRONE"]) {
        IconShape::Robot
    } else {
        IconShape::Diamond
    }
}

/// Colour keyed off icon category, out of the forge palette: locations run
/// purple, rigs run green, cargo runs amber, everything unclassified fades to
/// steel grey — a graph that reads by intent, not by the luck of a hash.
pub fn color_for(ty: &str) -> Color {
    use crate::palette;
    match shape_for(ty) {
        IconShape::Circle => palette::NODE_PURPLE, // places / locations
        IconShape::Truck => palette::RIG_GREEN,    // vehicles / rigs
        IconShape::Box => palette::CRATE_AMBER,    // packages / items
        IconShape::Robot => palette::CY,           // robots / rovers
        IconShape::Person | IconShape::Machine | IconShape::Diamond => palette::GREY_NODE,
    }
}

fn mesh_for(shape: IconShape, s: f32) -> Mesh {
    match shape {
        IconShape::Circle => Circle::new(s * 0.5).into(),
        IconShape::Truck => Rectangle::new(s * 0.95, s * 0.55).into(),
        IconShape::Box => Rectangle::new(s * 0.72, s * 0.72).into(),
        IconShape::Person => RegularPolygon::new(s * 0.6, 3).into(),
        IconShape::Robot => RegularPolygon::new(s * 0.58, 6).into(),
        IconShape::Machine => RegularPolygon::new(s * 0.56, 8).into(),
        IconShape::Diamond => RegularPolygon::new(s * 0.6, 4).into(),
    }
}

pub type MeshCache = HashMap<(IconShape, u32), Handle<Mesh>>;
pub type MatCache = HashMap<u32, Handle<ColorMaterial>>;

pub fn mesh_handle(
    meshes: &mut Assets<Mesh>,
    cache: &mut MeshCache,
    shape: IconShape,
    size: f32,
) -> Handle<Mesh> {
    cache
        .entry((shape, (size * 4.0) as u32))
        .or_insert_with(|| meshes.add(mesh_for(shape, size)))
        .clone()
}

pub fn mat_handle(
    materials: &mut Assets<ColorMaterial>,
    cache: &mut MatCache,
    color: Color,
) -> Handle<ColorMaterial> {
    let s = color.to_srgba();
    let key = ((s.red * 255.0) as u32) << 16
        | ((s.green * 255.0) as u32) << 8
        | ((s.blue * 255.0) as u32);
    cache
        .entry(key)
        .or_insert_with(|| materials.add(ColorMaterial::from(color)))
        .clone()
}
