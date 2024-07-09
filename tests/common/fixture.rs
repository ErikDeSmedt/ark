use common::anyhow::Result;

pub trait Fixture{
	fn setup(&mut self) -> Result<()>;
}

