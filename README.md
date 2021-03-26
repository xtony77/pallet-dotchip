# pallet-dotchip

chip money pallet for polkadot games

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-dotchip = { default-features = false, version = '1.0.0' }

[features]
default = ['std']
runtime-benchmarks = [
    ...
]
std = [
    ...
    // Add this code
    'pallet-dotchip/std'
]
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
impl pallet-dotchip::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        // Add this code
        DotChip: pallet-dotchip::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```