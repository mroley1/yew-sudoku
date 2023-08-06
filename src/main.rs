use std::{vec, borrow::BorrowMut};
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
    fn push(&mut self, value: usize) {
        let mut i = 0;
        for b in self.data.iter() {
            if *b {i = i + 1};
        }
        if i < 8 {
            self.data[value] = true;
        }
    }
    fn pop(&mut self, value: usize) {
        self.data[value] = false;
    }
    fn get_vec(self) -> Vec<usize> {
        let mut potential: Vec<usize> = vec![];
        for i in 0..=8 {
            if self.data[i] {
                potential.push(i);
            }
        }
        potential
    }
    fn has(self, value: usize) -> bool {
        self.data[value]
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
    html! {
        <div class="grid">
            {
                (*board_handler).grid.iter().map(|row| {
                    row.iter().map(|cell| {
                        
                        let click = {
                            let cell_click = cell_click.clone();
                            let mut board: Board = **board_handler;
                            let this_cell = board.grid[cell.x][cell.y].borrow_mut();
                            
                            if this_cell.potential.has(3) {
                                this_cell.value = this_cell.value + 1;
                            }
                            
                            //board.grid[cell.x][cell.y].value = board.grid[cell.x][cell.y].value + 1;
                            this_cell.potential.push(3);
                            this_cell.potential.push(6);
                            this_cell.potential.push(5);
                            this_cell.potential.push(3);
                            
                            
                            Callback::from(move |_| {cell_click.emit(board)})
                        };
                        
                        let grid_col = 1 + (cell.x * 2) - cell.x / 9;
                        let grid_row = 1 + (cell.y * 2);
                        
                        let style = format!("grid-column: {grid_col}; grid-row: {grid_row};");
                        html! {
                            <div class="cell" style={style.clone()} key={format!("{}{}", cell.x, cell.y)} onclick={click}>
                                <div class="highlighter"></div>
                                if cell.value != 0 {
                                    <div class="value">{cell.value}</div>
                                } else {
                                    <div class="potential">
                                        <CellPotential potential={cell.potential}/>
                                        // {
                                        //     cell.potential.get_vec().iter().map(|item| {
                                        //         html! {
                                        //             <div key={*item}>{item}</div>
                                        //         }
                                        //     }).collect::<Html>()
                                        // }
                                    </div>
                                }
                            </div>
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
            <MajorLines />
            <MinorLines />
        </div>
    }
}

#[function_component(MajorLines)]
fn major_lines() -> Html {
    html! {
        <>
            <div class="grid_line_major_horizontal" style="grid-row: 6;"></div>
            <div class="grid_line_major_horizontal" style="grid-row: 12;"></div>
            <div class="grid_line_major_vertical" style="grid-column: 6;"></div>
            <div class="grid_line_major_vertical" style="grid-column: 12;"></div>
        </>
    }
}

#[function_component(MinorLines)]
fn minor_lines() -> Html {
    let minor_edges: [usize; 6] = [2, 4, 8, 10, 14, 16];
    let minor_fuller: [usize; 9] = [1, 3, 5, 7, 9, 11, 13, 15, 17];
    html! {
        <>
            {
                minor_edges.iter().map(|col| {
                    minor_fuller.iter().map(|row| {
                        let style = format!("grid-row: {row}; grid-column: {col}");
                        html! {
                            <div key={format!("{}{}", col, row)} class="grid_line_minor_vertical" style={style}></div>
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
            {
                minor_fuller.iter().map(|col| {
                    minor_edges.iter().map(|row| {
                        let style = format!("grid-row: {row}; grid-column: {col}");
                        html! {
                            <div key={format!("{}{}", col, row)} class="grid_line_minor_horizontal" style={style}></div>
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct CellPotentialProps {
    potential: PotentialVec,
}

#[function_component(CellPotential)]
fn cell_potential(CellPotentialProps { potential }: &CellPotentialProps) -> Html {
    let mut top_row: Vec<usize> = vec![];
    let mut mid_row: Vec<usize> = vec![];
    let mut bot_row: Vec<usize> = vec![];
    let mut i = 0;
    for item in (0..=8).rev() {
        console_log!(item);
        if potential.has(item) {
            i = i+1;
            match i {
                1..=2 => bot_row.push(item),
                3..=6 => mid_row.push(item),
                7..=8 => top_row.push(item),
                _ => panic!(),
            }
        }
    }
    html! {
        <div style="height: 100%; display: grid; grid-template-rows: 3fr 3fr 3fr">
            <div style=" width: 100%; display: flex; justify-content: center; flex-direction: row-reverse;">
                {
                    top_row.iter().map(|item| {
                        html! {
                            <div key={*item} style="font-size: 20pt;">{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div style=" width: 100%; display: flex; justify-content: center; flex-direction: row-reverse;">
                {
                    mid_row.iter().map(|item| {
                        html! {
                            <div key={*item} style="font-size: 20pt;">{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div style=" width: 100%; display: flex; justify-content: center; flex-direction: row-reverse;">
                {
                    bot_row.iter().map(|item| {
                        html! {
                            <div key={*item} style="font-size: 20pt;">{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
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
            <div style="width: 100%; height: 100%; display: flex; justify-content: center">
                <div style="aspect-ratio: 1; height: 100%;">
                    <GridElement board_handler={board_handler} cell_click={board_plus_one} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
