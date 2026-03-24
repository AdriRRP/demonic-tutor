//! Test support for unit test entrypoints.

#[path = "unit/support/mod.rs"]
mod support;

#[path = "unit/combat/mod.rs"]
mod combat;
#[path = "unit/infrastructure/mod.rs"]
mod infrastructure;
#[path = "unit/lifecycle/mod.rs"]
mod lifecycle;
#[path = "unit/regressions/mod.rs"]
mod regressions;
#[path = "unit/resource_actions/mod.rs"]
mod resource_actions;
#[path = "unit/stack_priority/mod.rs"]
mod stack_priority;
#[path = "unit/turn_flow/mod.rs"]
mod turn_flow;
