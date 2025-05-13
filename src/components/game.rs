use std::fmt::Display;

use dioxus::{
    dioxus_core::{DynamicNode, VText},
    prelude::*,
};
use web_sys::window;

#[derive(PartialEq)]
enum GamePlayState {
    Ready,
    Playing,
    Over,
}
#[derive(PartialEq, Copy, Clone)]
enum Player {
    X,
    O,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}
impl IntoDynNode for Player {
    fn into_dyn_node(self) -> DynamicNode {
        match self {
            Player::O => DynamicNode::Text(VText::new("O")),
            Player::X => DynamicNode::Text(VText::new("X")),
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum BoardItem {
    X,
    O,
    EMPTY,
}
impl BoardItem {
    fn from_player(player: Player) -> BoardItem {
        match player {
            Player::O => BoardItem::O,
            Player::X => BoardItem::X,
        }
    }
    fn check_who_win(board: &Vec<BoardItem>) -> Option<Player> {
        let win_patterns = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for pattern in win_patterns {
            let [a, b, c] = pattern;
            if (board[a] == board[b] && board[a] == board[c]) {
                if (board[a] != BoardItem::EMPTY) {
                    return if board[a] == BoardItem::X {
                        Some(Player::X)
                    } else {
                        Some(Player::O)
                    };
                }
            }
        }
        return None;
    }
}
impl IntoDynNode for BoardItem {
    fn into_dyn_node(self) -> DynamicNode {
        match self {
            BoardItem::O => DynamicNode::Text(VText::new("O")),
            BoardItem::X => DynamicNode::Text(VText::new("X")),
            BoardItem::EMPTY => DynamicNode::Text(VText::new("")),
        }
    }
}

#[component]
pub fn Game() -> Element {
    let mut game_state = use_signal(|| GamePlayState::Ready);
    let game_ready = *game_state.read() == GamePlayState::Ready;
    let game_board_visible = !game_ready;
    let mut next_player = use_signal(|| Player::X);
    let mut game_winner = use_signal::<Option<Player>>(|| None);
    let mut game_board = use_signal(|| vec![BoardItem::EMPTY; 9]);

    let mut make_move = move |index: usize| {
        if game_board.read()[index] != BoardItem::EMPTY
            || *game_state.read() != GamePlayState::Playing
        {
            return;
        }
        game_board.write()[index] = BoardItem::from_player(*next_player.read());
        let winner = BoardItem::check_who_win(&*game_board.read());
        if winner.is_some() {
            game_state.set(GamePlayState::Over);
            game_winner.set(winner);
            return;
        }
        if game_board
            .read()
            .iter()
            .all(|item| *item != BoardItem::EMPTY)
        {
            game_state.set(GamePlayState::Over);
            return;
        }
        next_player.set(if *next_player.read() == Player::X {
            Player::O
        } else {
            Player::X
        });
    };
    let mut reset_state = move || {
        game_state.set(GamePlayState::Ready);
        next_player.set(Player::X);
        game_board.set(vec![BoardItem::EMPTY; 9]);
    };
    use_effect(move || {
        let window = window().unwrap();
        let game_winner = *game_winner.read();
        if let Some(winner) = game_winner {
            window
                .alert_with_message(format!("{winner} wins!").as_str())
                .unwrap();
            reset_state();
        } else if *game_state.read() == GamePlayState::Over {
            window.alert_with_message("Game over!").unwrap();
            reset_state();
        }
    });

    rsx!(div {
        class: "w-full h-full flex items-center justify-center flex-col",
        if game_ready {
            button { onclick: move |_| {
                game_state.set(GamePlayState::Playing);
            }, "开始游戏"}
        }
        if game_board_visible {
            div { class: "w-5 text-left", "下一个玩家:"{*next_player.read()}}
            div {
                class: "grid grid-cols-[repeat(3,60px)] grid-rows-[repeat(3,60px)] gap-[4px]",
               {game_board.read()
                .iter()
                .enumerate()
                .map(|(index,item)|
                    rsx!(div {
                        class: "w-full h-full cursor-pointer border border-solid flex items-center justify-center text-sm",
                        onclick: move |_| {
                            make_move(index)
                        },
                        {item}
                })
               )}
         }
        }
    })
}
