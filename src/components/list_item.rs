use yew::prelude::*;

pub struct ListItem {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub avatar_url: String,
    pub full_name: String,
    pub email: String,
    pub github: String,
}

impl Component for ListItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props: Props { ..props },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        fn make_mail_href(addr: &str) -> String {
            format!("mailto:{}", addr)
        }

        html! {
            <li class="developer">
                <figure class="avatar">
                    <img
                        src=&self.props.avatar_url
                        height="90"
                        width="90"
                    />
                </figure>
                <article>
                    <h3>{&self.props.full_name}</h3>
                    <div class="details">
                        <span class="field">
                            <figure>{"📧"}</figure>
                            <a href={make_mail_href(&self.props.email).as_str()}>{&self.props.email}</a>
                        </span>
                        <span class="field">
                            <figure>{"🐱"}</figure>
                            <a href={self.props.github.as_str()}>{&self.props.github}</a>
                      </span>
                    </div>
                </article>
            </li>
        }
    }
}
