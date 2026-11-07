fn hook(root: &Path) -> crate::Result<MobTypes> {
    let mut types = MobTypes::load_with_models(root, false)?;
    evanders_models::install(&mut types, false);
    Ok(types)
}
