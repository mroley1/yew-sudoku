use std::vec;
use array2d::{Array2D, Error};
use weblog::console_log;

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


#[derive(Clone, PartialEq)]
struct Cell {
    x: u8,
    y: u8,
    value: u8,
    potential: Vec<u8>,
}


impl Cell {
    fn new() -> Cell {
        Cell {x: 0, y:0, value: 0, potential: vec![]}
    }
    fn get_x(&self) -> u8 {
        return self.x;
    }
}

// impl Clone for Cell {
//     fn clone(&self) -> Cell {
//         Cell {x: self.x, y: self.y, value: self.value, potential: self.potential}
//     }
// }

fn get_clean_board() -> Array2D<Cell> {
    let mut long_vec = vec![];
    for _ in 1..=81 {
        long_vec.push(Cell::new());
    }
    return Array2D::from_row_major(&long_vec, 9, 9).unwrap();
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
    

    let mut grid = get_clean_board();
    
    console_log!(grid.get(0, 0).unwrap().get_x());
    
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
