use crate::context::Context;
use crate::context::Component;
use crate::global_properties::GlobalPropertyContext;

crate::define_global_property!(TestProperty, usize);

struct TestPlugin {}

impl TestPlugin {
}

impl crate::context::Plugin for TestPlugin {
    type DataContainer = u32;

    fn get_data_container() -> Self::DataContainer {
        0
    }
}

impl Component for TestPlugin {
    fn init(context: &mut Context) {
        let _ = context.get_global_property_value::<TestProperty>().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::test_plugin::TestPlugin;
    use crate::context::Context;
    
    #[test]
    fn test_plugin() {
        let mut context = Context::new();
        context.add_component::<TestPlugin>();
    }
}
    
