use std::{vec, fmt::format, u8, borrow::BorrowMut};
use array2d::{Array2D, Error};
use weblog::console_log;

use yew::prelude::*;

// trunk serve --open

#[derive(Clone, Copy, PartialEq)]
struct PotentialVec {
    data: [bool; 9],
}

impl PotentialVec {
    fn new() -> PotentialVec {
        PotentialVec { data: [false; 9]}
    }
    fn push(mut self, value: usize) {
        self.data[value] = true;
    }
    fn pop(mut self, value: usize) {
        self.data[value] = false;
    }
    fn get_vec(self) -> Vec<usize> {
        let mut potential: Vec<usize> = vec![];
        for i in 0..=8 {
            if self.data[i] {
                potential.push(i);
            }
        }
        return potential
    }
}


#[derive(Clone, PartialEq, Copy)]
struct Cell {
    x: usize,
    y: usize,
    value: usize,
    potential: PotentialVec,
}


impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        Cell {x, y, value: 0, potential: PotentialVec::new()}
    }
}

#[derive(Properties, PartialEq)]
struct CellProps {
    cell: Cell,
    on_click: Callback<Cell>,
}

#[function_component(CellElement)]
fn cell_element(CellProps { cell, on_click }: &CellProps) -> Html {
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

#[derive(Properties, PartialEq)]
struct GridProps {
    board_handler: UseStateHandle<Board>,
    cell_click: Callback<Board>
}

#[function_component(GridElement)]
fn grid_element(GridProps { board_handler, cell_click }: &GridProps) -> Html {
    
    (*board_handler).grid.iter().map(|row| {
        row.iter().map(|cell| {
            
            let click = {
                let cell_click = cell_click.clone();
                let mut board: Board = **board_handler;
                board.grid[cell.x][cell.y].value = board.grid[cell.x][cell.y].value + 1;
                Callback::from(move |_| {
                    cell_click.emit(board);
                })
            };
            
            html! {
                <div key={format!("{}{}", cell.x, cell.y)} onclick={click}>{cell.value}</div>
            }
        }).collect::<Html>()
    }).collect()
    
}

#[derive(Properties, PartialEq, Copy, Clone)]
struct Board {
    grid: [[Cell; 9]; 9],
    solved: bool,
}


impl Default for Board {
    fn default() -> Self {
        let mut grid = [[Cell::new(0, 0); 9]; 9];
        for x in 0..=8 {
            for y in 0..=8 {
                grid[x][y] = Cell::new(x, y);
            }
        }
        let solved: bool = false;
        Self {
            grid,
            solved,
        }
    }
}


#[function_component(App)]
fn app() -> Html {
    
    let board_handler: UseStateHandle<Board> = use_state(|| Board::default());
    
    let board_plus_one = {
        let board_handler = board_handler.clone();
        Callback::from(move |board:Board| {
            board_handler.set(board)
        })
    };
    
    html! {
        <>
            <div style="background-color: green; width: 100%; height: 100%; display: flex; justify-content: center">
                <div style="aspect-ratio: 1; height: 100%; background-color: aqua;">
                    <GridElement board_handler={board_handler} cell_click={board_plus_one} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
