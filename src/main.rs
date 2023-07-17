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
    fn new(x: u8, y: u8) -> Cell {
        Cell {x, y, value: 0, potential: vec![]}
    }
}

// impl Clone for Cell {
//     fn clone(&self) -> Cell {
//         Cell {x: self.x, y: self.y, value: self.value, potential: self.potential}
//     }
// }

fn get_clean_board() -> Board {
    let mut long_vec = vec![];
    for i in 0..=80 {
        let x = i % 9;
        let y = i / 9;
        long_vec.push(Cell::new(x, y));
    }
    return Board {
        grid: Array2D::from_row_major(&long_vec, 9, 9).unwrap(),
        solved: false,
    }
}

struct Board {
    grid: Array2D<Cell>,
    solved: bool,
}

impl Board {
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        return self.grid.get(y, x).expect("invalid get").clone();
    }
    fn set_val(&mut self, x: usize, y: usize, value: u8) {
        self.grid.get_mut(y, x).unwrap().value = value;
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
    

    let mut grid = get_clean_board();
    
    console_log!(grid.get_cell(1, 3).x);
    console_log!(grid.get_cell(1, 3).y);
    grid.set_val(1, 3, 4);
    console_log!(grid.get_cell(1, 3).value);
    
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
