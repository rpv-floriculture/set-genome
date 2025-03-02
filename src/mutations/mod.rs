use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{genes::Activation, genome::Genome};

pub use self::error::MutationError;

pub type MutationResult = Result<(), MutationError>;

mod add_connection;
mod add_node;
mod add_recurrent_connection;
mod change_activation;
mod change_weights;
mod error;
mod remove_connection;
mod remove_node;
mod remove_recurrent_connection;

/// Lists all possible mutations with their corresponding parameters.
///
/// Each mutation acts as a self-contained unit and has to be listed in the [`crate::Parameters::mutations`] field in order to take effect when calling [`crate::Genome::mutate_with`].
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mutations {
    /// See [`Mutations::change_weights`].
    ChangeWeights {
        chance: f32,
        percent_perturbed: f32,
        weight_cap: f32,
    },
    /// See [`Mutations::change_activation`].
    ChangeActivation {
        chance: f32,
        activation_pool: Vec<Activation>,
    },
    /// See [`Mutations::add_node`].
    AddNode {
        chance: f32,
        activation_pool: Vec<Activation>,
    },
    /// See [`Mutations::add_connection`].
    AddConnection { chance: f32 },
    /// See [`Mutations::add_recurrent_connection`].
    AddRecurrentConnection { chance: f32 },
    /// See [`Mutations::remove_node`].
    RemoveNode { chance: f32 },
    /// See [`Mutations::remove_connection`].
    RemoveConnection { chance: f32 },
    /// See [`Mutations::remove_recurrent_connection`].
    RemoveRecurrentConnection { chance: f32 },
}

impl Mutations {
    /// Mutate a [`Genome`] but respects the associate `chance` field of the [`Mutations`] enum variants.
    /// The user needs to supply some RNG manually when using this method directly.
    /// Use [`crate::Genome::mutate`] as the default API.
    pub fn mutate(&self, genome: &mut Genome, rng: &mut impl Rng) -> MutationResult {
        match self {
            &Mutations::ChangeWeights {
                chance,
                percent_perturbed,
                weight_cap,
            } => {
                if rng.gen::<f32>() < chance {
                    Self::change_weights(percent_perturbed, weight_cap, genome, rng);
                }
            }
            Mutations::AddNode {
                chance,
                activation_pool,
            } => {
                if rng.gen::<f32>() < *chance {
                    Self::add_node(activation_pool, genome, rng)
                }
            }
            &Mutations::AddConnection { chance } => {
                if rng.gen::<f32>() < chance {
                    return Self::add_connection(genome, rng);
                }
            }
            &Mutations::AddRecurrentConnection { chance } => {
                if rng.gen::<f32>() < chance {
                    return Self::add_recurrent_connection(genome, rng);
                }
            }
            Mutations::ChangeActivation {
                chance,
                activation_pool,
            } => {
                if rng.gen::<f32>() < *chance {
                    Self::change_activation(activation_pool, genome, rng)
                }
            }
            &Mutations::RemoveNode { chance } => {
                if rng.gen::<f32>() < chance {
                    return Self::remove_node(genome, rng);
                }
            }
            &Mutations::RemoveConnection { chance } => {
                if rng.gen::<f32>() < chance {
                    return Self::remove_connection(genome, rng);
                }
            }
            &Mutations::RemoveRecurrentConnection { chance } => {
                if rng.gen::<f32>() < chance {
                    return Self::remove_recurrent_connection(genome, rng);
                }
            }
        }
        Ok(())
    }
}
