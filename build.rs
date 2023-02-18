use shadow_rs::SdResult;

fn main() -> SdResult<()> {
    shadow_rs::new().map_err(|err| err.to_string())?;
    Ok(())
}
