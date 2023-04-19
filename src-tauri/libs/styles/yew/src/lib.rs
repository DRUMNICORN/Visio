use nodium_app::{NodiumRenderer, NodiumApp};
use yew::{html, Component, Html};

#[derive(Clone)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone)]
pub struct NodiumAppYew {
    _theme: Theme,
}

impl Component for NodiumAppYew {
    type Message = ();
    type Properties = ();

    fn rendered(&mut self, _ctx: &yew::Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}

    fn create(_ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        return html! {
          <div style="background-color: #000000; color: #ffffff;">
            <h1>{"Hello, world!"}</h1>
          </div>
        };
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        todo!()
    }
}


// YewRende
#[derive(Clone)]
pub struct YewRenderer {
    app: NodiumAppYew,
}
