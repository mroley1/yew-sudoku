use yew::prelude::*;



#[function_component(App)]
fn app() -> Html {
    
    #[derive(Clone, PartialEq)]
    struct OlItem {
        id: u8,
        content: String,
        styles: String,
    }
    
    #[derive(Properties, PartialEq)]
    struct ItemListProps {
        items: Vec<OlItem>,
    }
    
    #[function_component(ItemList)]
    fn item_list(ItemListProps { items }: &ItemListProps) -> Html {
        items.iter().map(|item| html! {
            <li key={item.id} style={item.styles.clone()}>{item.content.clone()}</li>
        }).collect()
    }
    
    let items = vec![
        OlItem {
            id: 1,
            content: String::from("red"),
            styles: String::from("color: red"),
        },
        OlItem {
            id: 2,
            content: String::from("green"),
            styles: String::from("color: green; transform: rotate(-30deg);"),
        },
        OlItem {
            id: 3,
            content: String::from("yeller"),
            styles: String::from("color: goldenrod"),
        },
        OlItem {
            id: 4,
            content: String::from("blue"),
            styles: String::from("color: blue"),
        },
    ];
    
    html! {
        <>
            <ol>
                <ItemList items={items} />
            </ol>
            <img src="https://assets.mroley.dev/discordBG.jpg" alt="picture" />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
