# signal-chain-integration

Signal chain crates for OpenShell fleet integration. Provides OpenShell-compatible wrappers around SuperInstance consensus and trust primitives.

## Architecture

```
signal-chain-integration/
└── crates/openshell-holonomy-consensus/
    ├── src/
    │   ├── lib.rs              # Re-exports from holonomy-consensus
    │   ├── consensus.rs        # Cycle-based consensus protocol
    │   ├── constraints.rs      # Constraint satisfaction for fleet coordination
    │   ├── encoding.rs         # GL(9) intent encoding
    │   ├── cohomology.rs       # Topological invariants for trust verification
    │   ├── trust_lifecycle.rs  # Trust pool lifecycle management
    │   ├── lifecycle.rs        # Tile lifecycle (submit → gate → store)
    │   ├── zhc_gl9.rs         # ZHC-GL(9) matrix operations (917 lines)
    │   └── benchmarks.rs       # Performance benchmarks
    └── Cargo.toml
```

## Usage

```rust
use openshell_holonomy_consensus::{
    HolonomyConsensus, TrustPool, Vector48, Pythagorean48,
    ConsensusResult, EmergenceDetector,
};

// Create a trust pool with Pythagorean48 encoding
let pool = TrustPool::new(8);

// Run cycle-based consensus (no voting, no CRDTs)
let result: ConsensusResult = pool.consensus(&proposals);

// Detect emergent behavior via topological invariants
let detector = EmergenceDetector::new(pool.bounds());
if detector.check(&result) {
    println!("Emergence detected: {:?}", detector.classify(&result));
}
```

## Key Concepts

- **Zero-Holonomy Consensus (ZHC)**: Agreement protocol using GL(9) intent alignment instead of voting
- **Pythagorean48**: 48-directional trust encoding for fleet members
- **Emergence Detection**: Topological invariants detect when fleet behavior exceeds individual agent capabilities

## Related Crates

- [holonomy-consensus](https://github.com/SuperInstance/holonomy-consensus) — Core consensus implementation
- [openshell-pythagorean48](https://github.com/SuperInstance/openshell-pythagorean48) — Trust vector encoding
- [cocapn-glue-core](https://github.com/SuperInstance/cocapn-glue-core) — Wire protocol for fleet communication

## License

Apache-2.0
