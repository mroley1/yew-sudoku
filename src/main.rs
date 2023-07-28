use std::vec;
use array2d::{Array2D, Error};
use weblog::console_log;

use yew::prelude::*;

// trunk serve --open

#[derive(Clone, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    value: usize,
    potential: Vec<usize>,
}


impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        Cell {x, y, value: 0, potential: vec![]}
    }
}

#[derive(Properties, PartialEq)]
struct CellProps {
    cell: Cell,
    on_click: Callback<Cell>,
}

#[function_component(CellElement)]
fn cell_element(CellProps { cell, on_click}: &CellProps) -> Html {
    let on_cell_select = {
        let on_click = on_click.clone();
        let cell = cell.clone();
        Callback::from(move |_| {
            on_click.emit(cell.clone())
        })
    };
    html! {
        <div onclick={on_cell_select}>{cell.value}</div>
    }
}

#[function_component(GridElement)]
fn grid_element() -> Html {
    let board = get_clean_board();
    let grid = board.grid;
    grid.rows_iter().map(|row| {
        row.map(|cell| {
            let on_cell_click = {
                Callback::from(move |cell: Cell| {
                    console_log!(format!("{}{}", cell.x, cell.y));
                })
            };
            
            let html = html! {
                <CellElement key={format!("{}{}", cell.x, cell.y)} on_click={on_cell_click} cell={cell.clone()} />
            };
            html
        }).collect::<Html>()
        
    }).collect()
}


struct Board {
    grid: Array2D<Cell>,
    solved: bool,
}

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


// impl Default for Board {
//     fn default() -> Self {
//         let mut long_vec = vec![];
//         for i in 0..=80 {
//             let x = i % 9;
//             let y = i / 9;
//             long_vec.push(Cell::new(x, y));
//     }
//         Self {
//             grid: Array2D::from_row_major(&long_vec, 9, 9).unwrap(),
//             solved: false,
//         }
//     }
// }

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
    
    
    
    
    // let details = selected_item.as_ref().map(|item| html! {
    //     <ItemDetails item={item.clone()} />
    // });
    
    html! {
        <>
            <div style="background-color: green; width: 100%; height: 100%; display: flex; justify-content: center">
                <div style="aspect-ratio: 1; height: 100%; background-color: aqua;">
                    <GridElement />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
