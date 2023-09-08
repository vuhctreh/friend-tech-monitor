# friend-tech-monitor

## Generate ABI example code:
```Rust
fn generate_abi() -> eyre::Result<()> {
    Abigen::new("BruhTech", "bruh-tech-abi.json")?.generate()?.write_to_file("sniper/sniper_contract.rs")?;

    Ok(())
}
```