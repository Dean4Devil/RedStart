//! A simple CRUD Trait. Should be implemented by *most* data Storage types.

trait CRUD<K, V>
{
    fn create(&self, K, V);
    fn read(&self, K) -> Option<V>;
    fn update(&self, K, V);
    fn delete(&self, K);
}
