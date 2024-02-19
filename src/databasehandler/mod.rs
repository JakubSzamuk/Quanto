mod dbmacros;

pub trait DynamicDbAdapter {
    fn connect(db_address: &String) -> Self;
    // fn
}