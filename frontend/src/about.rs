use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <Sidebar>
                <Bar header="Todo" content={0}/> // TODO: replace with json
                <Bar header="Todo" content={1}/>
                <Bar header="Todo" content={2}/>
                <Bar header="Todo" content={3}/>
            </Sidebar>
            <ContentList>
                <Content>
                    {"Todo"}
                </Content>
                <Content>
                    {"Todo"}
                </Content>
                <Content>
                    {"Todo"}
                </Content>
                <Content>
                    {"Todo"}
                </Content>
            </ContentList>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SideBarProp {
    children: ChildrenWithProps<Bar>,
}

#[function_component(Sidebar)]
fn sidebar(props: &SideBarProp) -> Html {
    html! {
        { for props.children.iter() }
    }
}

#[derive(Properties, PartialEq)]
struct BarProp {
    header: String,
    content: usize,
}

#[function_component(Bar)]
fn bar(props: &BarProp) -> Html {
    html! {
        <h1>{ props.header.clone() }</h1>
    }
}

#[derive(Properties, PartialEq)]
struct ContentListProp {
    children: ChildrenWithProps<Content>,
}

#[function_component(ContentList)]
fn content_list(props: &ContentListProp) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContentProp {
    children: Children,
}

#[function_component(Content)]
fn content(props: &ContentProp) -> Html {
    html! {
        <div>
            { for props.children.iter() }
        </div>
    }
}
