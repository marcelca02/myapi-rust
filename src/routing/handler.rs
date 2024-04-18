// Here we define the Handler trait and the Cloneable trait.

pub trait Handler: Cloneable {
    fn handle(&self) -> String;
}

trait Cloneable {
    fn clone_box(&self) -> Box<dyn Handler>;
}

impl<T: 'static + Handler + Clone> Cloneable for T {
    fn clone_box(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Box<dyn Handler> {
        self.clone_box()
    }
}



