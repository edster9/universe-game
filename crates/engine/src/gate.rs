//! The gate: the one place every change to the world passes through. It
//! applies a set of changes all together or not at all, checks that mass and
//! credits are conserved and the world is still well formed, and logs what
//! happened. See docs/ideas/world-engine.md, "Conservation is the core's job".

use std::fmt;

use crate::intent::Intent;
use crate::units::Credits;
use crate::world::{EntityId, World};

/// The only kinds of change the world allows. Laws propose them; only the
/// gate applies them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    /// Put `entity` in or on `to`: a place, a person, a container.
    Move { entity: EntityId, to: EntityId },
    /// Move credits from one wallet to another.
    Transfer {
        from: EntityId,
        to: EntityId,
        amount: Credits,
    },
}

/// One accepted action, as recorded by the gate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    pub seq: u64,
    pub actor: EntityId,
    pub intent: Intent,
    pub changes: Vec<Change>,
}

/// Why the gate refused a set of changes. The laws should have refused first,
/// so a fault means a law has a bug.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    UnknownEntity(EntityId),
    NotLocated(EntityId),
    IntoItself(EntityId),
    NoWallet(EntityId),
    SameWallet(EntityId),
    Insufficient { from: EntityId, amount: Credits },
    Overflow { to: EntityId, amount: Credits },
    NotConserved(&'static str),
    Invariant(String),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::UnknownEntity(id) => write!(f, "no entity {id:?}"),
            Fault::NotLocated(id) => write!(f, "{id:?} isn't anywhere, so it can't move"),
            Fault::IntoItself(id) => write!(f, "{id:?} can't be put inside itself"),
            Fault::NoWallet(id) => write!(f, "{id:?} has no wallet"),
            Fault::SameWallet(id) => write!(f, "{id:?} can't pay itself"),
            Fault::Insufficient { from, amount } => write!(f, "{from:?} doesn't have {amount}"),
            Fault::Overflow { to, amount } => write!(f, "{to:?} can't hold {amount} more"),
            Fault::NotConserved(what) => write!(f, "total {what} would change"),
            Fault::Invariant(why) => write!(f, "the world would be broken: {why}"),
        }
    }
}

impl std::error::Error for Fault {}

impl World {
    /// Applies `changes` on behalf of `actor`, all together or not at all.
    /// On success the action is logged. On any fault, the world is left
    /// exactly as it was.
    pub fn apply(
        &mut self,
        actor: EntityId,
        intent: Intent,
        changes: Vec<Change>,
    ) -> Result<(), Fault> {
        let mass_before = self.total_mass();
        let credits_before = self.total_credits();

        let mut undo = Vec::with_capacity(changes.len());
        for change in &changes {
            match self.apply_one(change) {
                Ok(inverse) => undo.push(inverse),
                Err(fault) => {
                    self.roll_back(undo);
                    return Err(fault);
                }
            }
        }

        let check = if self.total_mass() != mass_before {
            Err(Fault::NotConserved("mass"))
        } else if self.total_credits() != credits_before {
            Err(Fault::NotConserved("credits"))
        } else {
            self.check_invariants().map_err(Fault::Invariant)
        };
        if let Err(fault) = check {
            self.roll_back(undo);
            return Err(fault);
        }

        let seq = self.log.len() as u64;
        self.log.push(LogEntry {
            seq,
            actor,
            intent,
            changes,
        });
        Ok(())
    }

    /// Applies one change and returns the change that would undo it.
    fn apply_one(&mut self, change: &Change) -> Result<Change, Fault> {
        match *change {
            Change::Move { entity, to } => {
                for id in [entity, to] {
                    if !self.exists(id) {
                        return Err(Fault::UnknownEntity(id));
                    }
                }
                if entity == to {
                    return Err(Fault::IntoItself(entity));
                }
                let from = self.location(entity).ok_or(Fault::NotLocated(entity))?;
                self.locations.insert(entity, to);
                Ok(Change::Move { entity, to: from })
            }
            Change::Transfer { from, to, amount } => {
                if from == to {
                    return Err(Fault::SameWallet(from));
                }
                let payer = self.wallet(from).ok_or(Fault::NoWallet(from))?;
                let payee = self.wallet(to).ok_or(Fault::NoWallet(to))?;
                let payer_after = payer
                    .checked_sub(amount)
                    .ok_or(Fault::Insufficient { from, amount })?;
                let payee_after = payee
                    .checked_add(amount)
                    .ok_or(Fault::Overflow { to, amount })?;
                self.wallets.insert(from, payer_after);
                self.wallets.insert(to, payee_after);
                Ok(Change::Transfer {
                    from: to,
                    to: from,
                    amount,
                })
            }
        }
    }

    fn roll_back(&mut self, undo: Vec<Change>) {
        for change in undo.into_iter().rev() {
            self.apply_one(&change)
                .expect("undoing a change that was just applied cannot fail");
        }
    }
}
