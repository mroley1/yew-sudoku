use std::vec;
use array2d::{Array2D, Error};
use weblog::console_log;

use yew::prelude::*;

// trunk serve --open


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
    x: usize,
    y: usize,
    value: usize,
    potential: Vec<usize>,
}

#[derive(Properties, PartialEq)]
struct CellProps {
    items: Vec<Cell>,
    on_click: Callback<Cell>,
}

#[function_component(Grid)]
fn item_list(CellProps { items, on_click }: &CellProps) -> Html {
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
            <li key={item.x} onclick={on_item_select}>{item.value.clone()}</li>
        }
    }).collect()
}

impl Cell {
    fn new(x: usize, y: usize) -> Cell {
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

enum BoardAction {
    Set,
}

struct Board {
    grid: Array2D<Cell>,
    solved: bool,
}

impl Default for Board {
    fn default() -> Self {
        let mut long_vec = vec![];
        for i in 0..=80 {
            let x = i % 9;
            let y = i / 9;
            long_vec.push(Cell::new(x, y));
    }
        Self {
            grid: Array2D::from_row_major(&long_vec, 9, 9).unwrap(),
            solved: false,
        }
    }
}

impl Reducible for Board {
    type Action = BoardAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut next_grid = match action {
            BoardAction::Set => self.grid.clone(),
        };
        
        next_grid.get_mut(0, 0).unwrap().value = 4;
        
        Self {grid: next_grid, solved: self.solved}.into()
    }
}

impl Board {
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        return self.grid.get(y, x).expect("invalid get").clone();
    }
    fn set_val(&mut self, x: usize, y: usize, value: usize) {
        self.grid.get_mut(y, x).unwrap().value = value;
    }
}


#[function_component(App)]
fn app() -> Html {
    

    //let grid = get_clean_board();


    
    let board = use_reducer(Board::default);
    
    // let on_item_select = {
    //     let selected_item = selected_item.clone();
    //     Callback::from(move |item: OlItem| {
    //         selected_item.set(Some(item))
    //     }) 
    // };
    
    // let click_cell = {
    //     console_log!(board.borrow_mut().get_cell(0, 0).value);
    //     let board = board.clone();
    //     Callback::from(move |item: Cell| {
    //         board.borrow_mut().set_val(item.x, item.y, 3);
    //     }) 
    // };
    
    let click_click = {
        let board = board.clone();
        Callback::from(move |_| board.dispatch(BoardAction::Set))
    };
    
    
    
    // let details = selected_item.as_ref().map(|item| html! {
    //     <ItemDetails item={item.clone()} />
    // });
    
    html! {
        <>
            <div style="background-color: green; width: 100%; height: 100%; display: flex; justify-content: center">
                <div style="aspect-ratio: 1; height: 100%; background-color: aqua;">
                    <ol>
                        <Grid items={board.grid.as_column_major()} on_click={click_click} />
                    </ol>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
