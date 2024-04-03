// Entity Component System (ECS)
//
// ECS store "component" data (heatlh, position, velocity, etc.)
// in "struct of arrays" format that helps maximize memory access efficiency
// and iteration performance compared to an "array of structs". This can help
// for efficiency, but it can _really_ help with data ownership issues that
// Rust helps you identify in your code.
//
// Let's take the example of simulating 100 ping pong balls that have a
// size, position, and elasticity. Rather than storing an array of 100
// of structs describing these "components" (e.g., `Vec<Ball>`),
// we store 3 arrays for all ball entities, one for each component
// (e.g., `size: Vec<u8>`, `position: Vec<[u8, 2]>`, `elasticity: Vec<f16>`).
//
// Systems in an ECS operate on the component data, for example updating
// the position of each ping pong ball according to the velocity and amount
// of time elapsed.
//
// Entities in an ECS are simply unique identifiers that are used to lookup
// the component information for that entity. This ECS uses the `slotmap`
// crate to store this information, which uses "generational indexes"
// for entity identifiers. A generational index stores both the "key"
// into the data storage (e.g., the index into a vector) and the "version"
// of the index. This helps solve two issues,
//
//  1. If an Entity is removed from the world we increment the version of our
//     index, helping identify if an Entity that might be in our memory is now
//     "dead" (removed from the ECS)
//  2. Because we can identify "dead" entities, we can reuse the same storage slots
//     for new entities without causing confusion. This helps keep our storage small
//     and dense without need for compacting because we can reuse slots.
//
// The approach, desired features, user experience, and more are inspired by the following,
//
// * RustConf 2018 closing keynote by Catherine West
//     * This talk does a great job framing the problem of OOP approaches and gradually builds
//       up a more and more sophisticated ECS system example.
//     * https://www.youtube.com/watch?v=P9u8x13W7UE
// * Legion ECS
//     * I really liked how the Legion ECS system queries components in the ECS
//     * Legion's "view_tuple" macro also helped me understand how macros can be
//       used to codegen for supporting arbitrary numbers of components in a query.
//     * https://github.com/amethyst/legion
// * Specs ECS
//     * I liked the user experience of their entity builder pattern
//     * This ECS stores components in a similar manner to Specs (doesn't use archetypes)
//       but (as of writing) we don't use anything like a bitmask to accelerate
//       queries when components are sparsely populated.
//     * https://github.com/amethyst/specs
// * This blog post has great visuals and benchmarks to explain how Specs and Legion
//   work and the pros/cons of their approaches (bitmask + store all components versus archetypes)
//     * https://csherratt.github.io/blog/posts/specs-and-legion/
// * This is an entire tutorial for writing an ECS!
//     * https://rust-tutorials.github.io/entity-component-scrapyard/01-introduction/introduction.html
//
mod builder;
mod core;
mod filter;
mod view;
mod world;

pub use builder::EntityBuilder;
pub use core::{Component, EntityId};
pub use filter::{EntityMapFilter, IsEntity, IsNotEntity};
pub use view::View;
pub use world::World;
