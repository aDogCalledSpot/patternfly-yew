use crate::{Button, Divider, Icon, Position};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub toggle: Option<Html>,
    #[prop_or_default]
    pub children: ChildrenRenderer<AppLauncherChildVariant>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub position: Position,
}

pub enum Msg {
    Toggle,
}

pub struct AppLauncher {
    props: Props,
    link: ComponentLink<Self>,

    expanded: bool,
}

impl Component for AppLauncher {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            expanded: false,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-app-launcher");
        let mut menu_classes = Classes::from("pf-c-app-launcher__menu");

        match self.props.position {
            Position::Left => {}
            Position::Right => menu_classes.push("pf-m-align-right"),
            Position::Top => classes.push("pf-m-top"),
        }

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        let onclick = self.link.callback(|_| Msg::Toggle);

        return html! {
            <nav class=classes>
                <Button
                    class="pf-c-app-launcher__toggle"
                    r#type="button"
                    disabled=self.props.disabled
                    onclick=onclick
                    >
                    { self.render_trigger() }
                </Button>
                <ul class=menu_classes hidden=!self.expanded>
                    { for self.props.children.iter() }
                </ul>
            </nav>
        };
    }
}

impl AppLauncher {
    fn render_trigger(&self) -> Html {
        if let Some(toggle) = &self.props.toggle {
            toggle.clone()
        } else {
            Icon::Th.into()
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum AppLauncherChild {
    Item(<AppLauncherItem as Component>::Properties),
    Divider(<Divider as Component>::Properties),
}

impl From<AppLauncherItemProps> for AppLauncherChild {
    fn from(props: AppLauncherItemProps) -> Self {
        AppLauncherChild::Item(props)
    }
}

impl From<()> for AppLauncherChild {
    fn from(_: ()) -> Self {
        AppLauncherChild::Divider(())
    }
}

#[derive(PartialEq, Clone)]
pub struct AppLauncherChildVariant {
    props: AppLauncherChild,
}

impl<CHILD> From<VChild<CHILD>> for AppLauncherChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<AppLauncherChild>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl Into<Html> for AppLauncherChildVariant {
    fn into(self) -> Html {
        match self.props {
            AppLauncherChild::Item(props) => {
                VComp::new::<AppLauncherItem>(props, NodeRef::default(), None).into()
            }
            AppLauncherChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct AppLauncherItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    #[prop_or_default]
    pub external: bool,
}

#[derive(Clone, PartialEq)]
pub struct AppLauncherItem {
    props: AppLauncherItemProps,
}

impl Component for AppLauncherItem {
    type Message = ();
    type Properties = AppLauncherItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let action = if let Some(onclick) = &self.props.onclick {
            html! {
                <Button
                    class="pf-c-app-launcher__menu-item"
                    onclick=onclick.clone().reform(|_|{})
                    >
                    { for self.props.children.iter() }
                </Button>
            }
        } else {
            let mut classes = Classes::from("pf-c-app-launcher__menu-item");

            let target = if self.props.external {
                classes.push("pf-m-external");
                "_blank"
            } else {
                ""
            };

            html! {
                <a
                    class=classes
                    target=target
                    href=self.props.href.clone()>

                { for self.props.children.iter() }

                { if self.props.external {html!{
                    <>
                    <span class="pf-c-app-launcher__menu-item-external-icon">
                        { Icon::ExternalLinkAlt }
                    </span>
                    <span class="pf-screen-reader">{"(opens new window)"}</span>
                    </>
                }} else {html!{}} }

                </a>
            }
        };

        return html! {
            <li>{action}</li>
        };
    }
}
