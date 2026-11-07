fn hook(root: &Path) -> crate::Result<MobTypes> {
    let mut types = MobTypes::load_with_models(root, true)?;
    evanders_models::install(&mut types, true);
    Ok(types)
}
