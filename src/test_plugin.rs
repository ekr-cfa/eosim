use crate::context::Context;
use crate::context::Component;
use crate::global_properties::GlobalPropertyContext;

crate::define_global_property!(TestProperty, usize);

#[derive(Debug)]
struct TestPlugin {
    f: usize,
}

impl TestPlugin {
    fn init2(&mut self, context: &mut Context) {
        self.f = *context.get_global_property_value::<TestProperty>().unwrap();
    }

    fn callback(context: &mut Context) {
    }
}

impl crate::context::Plugin for TestPlugin {
    type DataContainer = u32;

    fn get_data_container() -> Self::DataContainer {
        0
    }
}

impl Component for TestPlugin {
    fn init(_context: &mut Context) {
    }
}

#[cfg(test)]
mod tests {
    use crate::test_plugin::TestPlugin;
    use crate::context::Context;
    use crate::global_properties::GlobalPropertyContext;    
    use crate::test_plugin::TestProperty;
    
    #[test]
    fn test_plugin() {
        let mut context = Context::new();
        let u:usize = 10;
        context.set_global_property_value::<TestProperty>(u);
        let mut tp = TestPlugin{f: 0};
        tp.init2(&mut context);
        context.add_instance(tp);
        let tp2 = context.get_instance::<TestPlugin>().unwrap();
        println!("TP2 {:?}", tp2);
        assert_eq!(tp2.f, u);
        let _ = context.add_plan(1.0, TestPlugin::callback);
        context.execute();
    }
}
    
