use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct OlItem {
    id: u8,
    content: String,
    styles: String,
}

#[derive(Properties, PartialEq)]
struct ItemListProps {
    items: Vec<OlItem>,
    on_click: Callback<OlItem>,
}

#[function_component(ItemList)]
fn item_list(ItemListProps { items, on_click }: &ItemListProps) -> Html {
    let on_click = on_click.clone();
    items.iter().map(|item| {
        let on_item_select = {
            let on_click = on_click.clone();
            let item = item.clone();
            Callback::from(move |_| {
                on_click.emit(item.clone())
            })
        };
        
        html! {
            <li key={item.id} style={item.styles.clone()} onclick={on_item_select}>{item.content.clone()}</li>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
struct ItemDetailsProps {
    item: OlItem,
}

#[function_component(ItemDetails)]
fn item_details(ItemDetailsProps { item }: &ItemDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ item.id.clone() }</h3>
            <h4>{ item.content.clone() }</h4>
        </div>
    }
}


#[function_component(App)]
fn app() -> Html {
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
    
    let selected_item = use_state(|| None);
    
    let on_item_select = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: OlItem| {
            selected_item.set(Some(item))
        }) 
    };
    
    let details = selected_item.as_ref().map(|item| html! {
        <ItemDetails item={item.clone()} />
    });
    
    html! {
        <>
            <ol>
                <ItemList items={items} on_click={on_item_select.clone()} />
            </ol>
            {for details}
            <img src="https://assets.mroley.dev/discordBG.jpg" alt="picture" />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
