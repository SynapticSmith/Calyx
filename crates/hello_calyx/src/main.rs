//! A hands-on example demonstrating Calyx's core concepts.
//!
//! Calyx stores *constellations* — one input measured through many frozen
//! *lenses*, each creating a separate typed slot-vector. This example mocks
//! the basic usage using the core trait boundaries to show how these concepts
//! come together in a developer's perspective.

use std::collections::BTreeMap;
use calyx_core::{
    Input, Modality, SlotVector, SlotShape, LensId, CxId, VaultId, LedgerRef,
    Constellation, InputRef, CxFlags, Anchor, AnchorKind, AnchorValue
};
use calyx_core::traits::{Lens, VaultStore};

// ── Mock implementations of the Core Traits ──────────────────────────────

/// A simple mock Dense Lens that just outputs the length of the input bytes.
struct MockDenseLens;
impl Lens for MockDenseLens {
    fn id(&self) -> LensId {
        LensId::from_bytes([1; 16])
    }

    fn shape(&self) -> SlotShape {
        SlotShape::Dense(1)
    }

    fn modality(&self) -> Modality {
        Modality::Text
    }

    fn measure(&self, input: &Input) -> calyx_core::Result<SlotVector> {
        Ok(SlotVector::Dense {
            dim: 1,
            data: vec![input.bytes.len() as f32],
        })
    }
}

/// A mock store that just keeps the last inserted constellation in memory.
#[derive(Default)]
struct MockVaultStore {
    last_constellation: std::sync::Mutex<Option<Constellation>>,
}

impl VaultStore for MockVaultStore {
    fn put(&self, constellation: Constellation) -> calyx_core::Result<CxId> {
        let id = constellation.cx_id;
        *self.last_constellation.lock().unwrap() = Some(constellation);
        Ok(id)
    }

    fn get(&self, id: CxId, _snapshot: u64) -> calyx_core::Result<Constellation> {
        self.last_constellation
            .lock()
            .unwrap()
            .clone()
            .filter(|cx| cx.cx_id == id)
            // Error handling could be expanded, mocking with a simple error for now
            .ok_or_else(|| calyx_core::CalyxError::stale_derived("Not found in mock store"))
    }

    fn anchor(&self, _id: CxId, _anchor: Anchor) -> calyx_core::Result<()> {
        Ok(())
    }

    fn snapshot(&self) -> u64 {
        1
    }
}

fn main() -> calyx_core::Result<()> {
    println!("🌟 Welcome to Calyx: Hello World Example 🌟\n");

    // 1. Create a Vault Store (Mocked)
    // In a real app, this would be an `Aster` embedded LSM storage engine.
    let store = MockVaultStore::default();
    println!("✅ Initialized Storage Engine.");

    // 2. Define an Input
    // We are going to feed a piece of text to our system.
    let text = "Calyx is an association-native database.";
    let input = Input::new(Modality::Text, text.as_bytes().to_vec());
    println!("✅ Created Input (Modality: Text, Length: {} bytes).", text.len());

    // 3. Measure through a Lens
    // A lens is an embedder or feature extractor.
    // Here we pass our input through a simple mock lens.
    let lens = MockDenseLens;
    let slot_vector = lens.measure(&input)?;
    println!("✅ Measured Input. Slot Vector Output: {:?}", slot_vector);

    // 4. Assemble a Constellation
    // In a full run, we would pass the input through multiple lenses (a panel),
    // and keep the slots separated. Then we anchor the result.
    let cx_id = CxId::from_bytes([42; 16]); // arbitrary unique ID for this example
    let vault_id = "01ARZ3NDEKTSV4RRFFQ69G5FAV".parse::<VaultId>().unwrap();

    let mut slots = BTreeMap::new();
    slots.insert(calyx_core::SlotId::new(1), slot_vector);

    let constellation = Constellation {
        cx_id,
        vault_id,
        panel_version: 1,
        created_at: 1000,
        input_ref: InputRef {
            hash: [0; 32], // Mock hash
            pointer: None,
            redacted: false,
        },
        modality: Modality::Text,
        slots,
        scalars: BTreeMap::new(),
        metadata: BTreeMap::new(),
        // Calyx grounds data natively. We attach anchors to show consequences.
        anchors: vec![Anchor {
            kind: AnchorKind::Reward,
            value: AnchorValue::Number(1.0),
            source: "example".to_string(),
            observed_at: 1000,
            confidence: 1.0,
        }],
        provenance: LedgerRef {
            seq: 1,
            hash: [0; 32],
        },
        flags: CxFlags::default(),
    };

    // 5. Store the Constellation
    // We put it into our simulated Aster vault.
    let saved_id = store.put(constellation)?;
    println!("✅ Stored Constellation with ID: {:?}", saved_id);

    // 6. Retrieve it
    let retrieved = store.get(saved_id, 1)?;
    println!("\n🔍 Retrieved Constellation:");
    println!("   Modality: {:?}", retrieved.modality);
    println!("   Anchors attached: {}", retrieved.anchors.len());
    println!("   Number of Lenses (Slots): {}", retrieved.slots.len());

    println!("\n🎉 Successfully completed Calyx Hello World pipeline!");
    Ok(())
}
