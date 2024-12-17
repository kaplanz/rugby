use rugby::core::dmg;

uniffi::setup_scaffolding!();

#[uniffi::export]
#[derive(uniffi::Record)]
pub struct GameBoy {
    pub(crate) inner: dmg::GameBoy,
}

#[uniffi::export]
pub fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}
