use nova_snark::PublicParams;

use crate::{G1, G2, C1, C2};

pub fn println_constraints_summary(pp: &PublicParams<G1, G2, C1, C2>) {
    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );
}