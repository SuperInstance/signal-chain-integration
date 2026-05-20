//! OpenShell wrapper for holonomy-consensus
//! 
//! Zero-holonomy consensus for fleet coordination — GL(9) intent alignment,
//! cycle-based trust verification, eliminates voting/CRDTs/BFT.
//!
//! Original crate: [SuperInstance/holonomy-consensus](https://github.com/SuperInstance/holonomy-consensus)

pub use holonomy_consensus::{
    cohomology, consensus, constraints, encoding, lifecycle,
    trust_lifecycle, zhc_gl9, ConsensusResult, ConstraintResult, EmergenceDetector,
    EmergenceResult, HolonomyBounds, HolonomyConsensus, LamportClock, LifecycleError,
    Pythagorean48, RetractionReason, TrustPool, TrustState, TrustTile, Vector48, sat8,
};

pub mod platform {
    pub use holonomy_consensus::zhc_gl9::GL9Matrix;
}

pub type Result<T> = std::result::Result<T, holonomy_consensus::trust_lifecycle::LifecycleError>;