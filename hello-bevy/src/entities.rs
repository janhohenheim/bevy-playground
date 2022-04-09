use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct Person;

#[derive(Component, Debug)]
pub(crate) struct Name(pub String);
