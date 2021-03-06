use std::any::Any;

use crate::{core::AnyView, view_bump::ViewBump, widget_host::WidgetHost};

pub trait AppDyn {
    fn update(&mut self, msg: Box<dyn Any>);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

pub trait Application: 'static {
    type Msg;
    fn update(&mut self, msg: Self::Msg);
    fn view<'a>(&'a self) -> AnyView<'a>;
}

impl<Msg, A> AppDyn for A
where
    A: Application<Msg = Msg>,
    Msg: 'static,
{
    fn update(&mut self, msg: Box<dyn Any>) {
        match msg.downcast::<Msg>() {
            Ok(msg) => self.update(*msg),
            Err(msg) => eprintln!("Unknown Message: {:?}", msg),
        }
    }

    fn view<'a>(&'a self) -> AnyView<'a> {
        self.view()
    }
}

pub struct AppHolder {
    app: Box<dyn AppDyn>,
    host: WidgetHost,
}

impl AppHolder {
    pub fn new(app: Box<dyn AppDyn>) -> Self {
        ViewBump::init();
        let widget = app.view().build();
        ViewBump::reset();
        let host = WidgetHost::new(widget);

        Self { app, host }
    }

    pub fn with_host<R>(&mut self, f: impl FnOnce(&mut WidgetHost) -> R) -> R {
        f(&mut self.host)
    }

    pub fn update(&mut self, msg: Box<dyn Any>) {
        self.app.update(msg);
        ViewBump::init();
        let next_view = self.app.view();
        self.host.update(next_view);
        ViewBump::reset();
    }
}
