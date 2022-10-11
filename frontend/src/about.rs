use yew::prelude::*;

#[function_component(About)]
pub fn about(props: &AboutProp) -> Html {
    html! {
        <div>
            <Sidebar> // TODO: replace with json
                <Bar header="Todo" content={0}/>
                <Bar header="Todo" content={1}/>
                <Bar header="Todo" content={2}/>
                <Bar header="Todo" content={3}/>
            </Sidebar>
            <ContentList content={props.content_id}>
                <Content>
                    {"Foo"}
                </Content>
                <Content>
                    {"Bar"}
                </Content>
                <Content>
                    {"Quantum mechanics is a fundamental theory in physics that provides a description of the physical properties of nature at the
                     scale of atoms and subatomic particles.[2]:1.1 It is the foundation of all quantum physics including quantum chemistry, 
                     quantum field theory, quantum technology, and quantum information science.

                    Classical physics, the collection of theories that existed before the advent of quantum mechanics, 
                    describes many aspects of nature at an ordinary (macroscopic) scale, but is not sufficient for describing them at small 
                    (atomic and subatomic) scales. Most theories in classical physics can be derived from quantum mechanics as an approximation 
                    valid at large (macroscopic) scale.[3]
                    
                    Quantum mechanics differs from classical physics in that energy, momentum, angular momentum, 
                    and other quantities of a bound system are restricted to discrete values (quantization); objects have characteristics of both 
                    particles and waves (wave-particle duality); and there are limits to how accurately the value of a physical quantity can be 
                    predicted prior to its measurement, given a complete set of initial conditions (the uncertainty principle).
                    
                    Quantum mechanics arose gradually from theories to explain observations which could not be reconciled with classical physics, 
                    such as Max Planck's solution in 1900 to the black-body radiation problem, 
                    and the correspondence between energy and frequency in Albert Einstein's 1905 paper which explained the photoelectric effect. 
                    These early attempts to understand microscopic phenomena, now known as the \"old quantum theory\", 
                    led to the full development of quantum mechanics in the mid-1920s by Niels Bohr, Erwin Schrödinger, Werner Heisenberg, Max Born, 
                    Paul Dirac and others. The modern theory is formulated in various specially developed mathematical formalisms. In one of them, 
                    a mathematical entity called the wave function provides information, in the form of probability amplitudes, 
                    about what measurements of a particle's energy, momentum, and other physical properties may yield."}
                </Content>
                <Content>
                    {"Todo"}
                </Content>
            </ContentList>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct AboutProp {
    pub content_id: u8,
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
    content: u8,
}

#[function_component(ContentList)]
fn content_list(props: &ContentListProp) -> Html {
    let content = props.children.iter().nth(props.content.into()).unwrap();
    html! {
        <div>
            { content }
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
