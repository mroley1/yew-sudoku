use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{EventTarget, HtmlInputElement, HtmlElement};
use std::{vec, fmt, borrow::BorrowMut};
use weblog::console_log;

use rand::prelude::*;
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
        if i < 9 {
            self.data[value-1] = true;
        }
    }
    fn pop(&mut self, value: usize) -> bool {
        let state = self.data[value-1];
        self.data[value-1] = false;
        state
    }
    fn get_vec(self) -> Vec<usize> {
        let mut potential: Vec<usize> = vec![];
        for i in 0..=8 {
            if self.data[i] {
                potential.push(i+1);
            }
        }
        potential
    }
    fn has(self, value: usize) -> bool {
        self.data[value-1]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum HighlightState {
    OFF,
    FAST,
    SLOW,
}

impl fmt::Display for HighlightState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HighlightState::OFF => write!(f, "off"),
            HighlightState::FAST => write!(f, "fast"),
            HighlightState::SLOW => write!(f, "slow"),
        }
    }
}


#[derive(Clone, PartialEq, Copy)]
struct Cell {
    x: usize,
    y: usize,
    value: usize,
    potential: PotentialVec,
    highlight: HighlightState,
}


impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        Cell {x, y, value: 0, potential: PotentialVec::new(), highlight: HighlightState::OFF}
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
    board_set: Callback<Board>
}

fn update_board(mut board: Board, cell: Cell) -> Board {
    for row in board.grid {
        for cell in row {
            if board.grid[cell.x][cell.y].value == board.selected && board.selected != 0 {
                board.grid[cell.x][cell.y].highlight = HighlightState::SLOW;
            } else {
                board.grid[cell.x][cell.y].highlight = HighlightState::OFF;
            }
        }
    }
    
    board
}

#[function_component(GridElement)]
fn grid_element(GridProps { board_handler, board_set }: &GridProps) -> Html {
    
    let mut rng = rand::thread_rng();
    
    html! {
        <div class="grid">
            {
                (*board_handler).grid.iter().map(|row| {
                    row.iter().map(|cell| {
                        
                        let click = {
                            let board_set = board_set.clone();
                            let mut board: Board = **board_handler;
                            
                            
                            board.selected = board.selected + 1;
                            
                            board.grid[cell.x][cell.y].value = board.grid[cell.x][cell.y].value + 2;
                            
                            
                            board = update_board(board, *cell);
                            
                            Callback::from(move |_| {board_set.emit(board)})
                        };
                        
                        
                        
                        let grid_col = 1 + (cell.x * 2) - cell.x / 9;
                        let grid_row = 1 + (cell.y * 2);
                        
                        let highlight_delay: String = match cell.highlight {
                            HighlightState::OFF => String::from("0s"),
                            HighlightState::FAST => String::from("0s"),
                            HighlightState::SLOW => format!("{}s", rng.gen::<f32>()),
                        };
                        let highlight_style = format!("transition: all 1s linear; transition-delay: {highlight_delay}");
                        let main_style = format!("grid-column: {grid_col}; grid-row: {grid_row};");
                        html! {
                            <div class="cell" style={main_style.clone()} key={format!("{}{}", cell.x, cell.y)} onclick={click}>
                                <div data-highlight={format!("{}", cell.highlight)} style={highlight_style} class="highlighter"></div>
                                if cell.value != 0 {
                                    <div class="value">{cell.value}</div>
                                } else {
                                    <div class="potential">
                                        <CellPotential potential={cell.potential}/>
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
    for item in (1..=9).rev() {
        if potential.has(item) {
            i = i+1;
            match i {
                1..=2 => bot_row.push(item),
                3..=6 => mid_row.push(item),
                7..=9 => top_row.push(item),
                _ => panic!(),
            }
        }
    }
    html! {
        <div style="height: 100%; display: grid; grid-template-rows: 3fr 3fr 3fr">
            <div class="row">
                {
                    top_row.iter().map(|item| {
                        html! {
                            <div class="number" key={*item}>{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="row">
                {
                    mid_row.iter().map(|item| {
                        html! {
                            <div class="number" key={*item}>{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="row">
                {
                    bot_row.iter().map(|item| {
                        html! {
                            <div class="number" key={*item}>{item}</div>
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
    selected: usize,
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
        Self {
            grid,
            selected: 0,
            solved: false,
        }
    }
}

impl Board {
    fn to_string(self) -> String {
        let mut string: String = String::new();
        for row in self.grid {
            for cell in row {
                string.push_str(format!("{}", cell.value).as_str());
            }
            string.push_str("\n");
        }
        string.push_str(format!("{}", self.selected).as_str());
        string
    }
}


#[function_component(App)]
fn app() -> Html {
    
    let mut board_handler: UseStateHandle<Board> = use_state(|| Board::default());
    
    let click_set = {
        let board_handler = board_handler.clone();
        Callback::from(move |board:Board| {
            console_log!(board.to_string());
            board_handler.set(board)
        })
    };
    
    use_effect(move || {
        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // click_set.emit();
            console_log!(event.key());
        });
        || drop(listener)
    });
    
    html! {
        <>
            <div style="width: 100%; height: 100%; display: flex; justify-content: center">
                <div style="aspect-ratio: 1; height: 100%;">
                    <GridElement board_handler={board_handler} board_set={click_set} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
