pub trait Model
{
    fn new() -> Self;

    fn name(&self) -> &'static str;

}
