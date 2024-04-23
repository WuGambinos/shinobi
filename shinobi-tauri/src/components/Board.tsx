import './Board.css';
import Tile from './Tile';
import { createEffect, createMemo, createSignal } from 'solid-js';
import { invoke } from "@tauri-apps/api/tauri";

const files = ["1", "2", "3", "4", "5", "6", "7", "8"];
const ranks = ["a", "b", "c", "d", "e", "f", "g", "h"];

function pieceToStr(piece: string) {
    switch (piece) {
        case 'p':
            return './assets/bP.png'

        case 'r':
            return './assets/bR.png'

        case 'n':
            return './assets/bN.png'

        case 'b':
            return './assets/bB.png'

        case 'q':
            return './assets/bQ.png'

        case 'k':
            return './assets/bK.png'

        case 'P':
            return './assets/wP.png'

        case 'R':
            return './assets/wR.png'

        case 'N':
            return './assets/wN.png'

        case 'B':
            return './assets/wB.png'

        case 'Q':
            return './assets/wQ.png'

        case 'K':
            return './assets/wK.png'

        default:
            break;

    }
}



const START_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const startPos = [
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
];

const empty_board = [
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.']

]

export default function Board() {

    const [fen, setFen] = createSignal(START_FEN);
    const [board, setBoard] = createSignal(startPos);
    const [vBoard, setVBoard] = createSignal(displayBoard());
    const memoizedVisualBoard = createMemo(() => vBoard());

    let activePiece: HTMLElement | null = null;
    let old_piece_x: number;
    let old_piece_y: number;
    let boardRef: HTMLDivElement;


    function grabPiece(e: MouseEvent) {
        const element = e.target as HTMLElement;
        const boardR = boardRef;
        if (element.classList.contains('piece-image')) {
            const x = e.clientX - 20;
            const y = e.clientY - 20;
            element.style.left = `${x}px`;
            element.style.top = `${y}px`;
            element.style.position = "absolute";
            activePiece = element;
            const pos = activePiece.getBoundingClientRect();
            const old_x = Math.round((pos.x - boardR.offsetLeft) / 50);
            const old_y = Math.round((pos.y - boardR.offsetTop) / 50);
            old_piece_x = old_x;
            old_piece_y = old_y;
        }
    }

    function movePiece(e: MouseEvent) {
        const boardR = boardRef;

        if (activePiece && boardR) {
            const minX = boardR.offsetLeft - 10;
            const minY = boardR.offsetTop - 10;
            const maxX = boardR.offsetLeft + boardR.clientWidth - 30;
            const maxY = boardR.offsetTop + boardR.clientHeight - 30;
            const x = e.clientX - 20;
            const y = e.clientY - 20;
            activePiece.style.position = "absolute";

            // X Bounds
            if (x < minX) {
                activePiece.style.left = `${minX}px`;
            } else if (x > maxX) {
                activePiece.style.left = `${maxX}px`;
            }
            else {
                activePiece.style.left = `${x}px`;
            }


            // Y Bounds
            if (y < minY) {
                activePiece.style.top = `${minY}px`;
            }
            else if (y > maxY) {
                activePiece.style.top = `${maxY}px`;
            }
            else {
                activePiece.style.top = `${y}px`;
            }


        }
    }

    function dropPiece(e: MouseEvent) {
        console.log(e);
        const boardR = boardRef;
        if (activePiece) {
            const x = Math.floor((e.clientX - boardR.offsetLeft) / 50);
            const y = Math.floor((e.clientY - boardR.offsetTop) / 50);
            let b = board();



            let temp = b[old_piece_y][old_piece_x];
            b[old_piece_y][old_piece_x] = '.';
            b[y][x] = temp;
            setBoard(b);
            setVBoard(displayBoard());
            activePiece = null;
        }
    }

    function displayBoard() {
        let visualBoard = [];
        for (let rank = 0; rank < ranks.length; rank++) {
            for (let file = 0; file < files.length; file++) {
                if (board().length == 8) {
                    const color = (rank + file) % 2 == 0;
                    let piece: string = board()[rank][file];
                    let img_src: string = pieceToStr(piece) as string;
                    visualBoard.push(<Tile dark={color} image={img_src} />);
                }
            }
        }
        return visualBoard;
    }

    async function makeMove() {
        let moves = await invoke('moves', {});
        let mv = await invoke('make_move', { mv: moves[0] });
        setBoard(await invoke('recieve_position', {}));
        console.log("MOVE MADE", mv);
        setVBoard(displayBoard());
    }

    async function resetPosition() {
        console.log("RESET POSITION");
        await invoke('reset_position', {});
        let position = await invoke('recieve_position', {});
        console.log("POSITON:", position);

        setBoard(await invoke('recieve_position', {}));
        setVBoard(displayBoard());
    }

    async function perft() {
        console.log(await invoke('get_perft', { 'depth': 3 }));
    }

    createEffect(() => {
        console.log("BOARD HAS CHANGED", board());
    });


    window.onload = () => {
        resetPosition();
    }

    async function loadFen() {
        let input = document.getElementById("fen_string") as HTMLInputElement;
        setFen(input.value);
        await invoke('load_fen', { 'fen': fen() });
        setBoard(await invoke('recieve_position', {}));
        setVBoard(displayBoard());
        console.log(fen());
    }

    async function search() {

        for (let i = 0; i < 10; i++) {
            await invoke('search', {});
            updateBoard();

            // Make it so i can see moves being made
            await sleep(500);
        }

    }

    function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async function updateBoard() {
        setBoard(await invoke('recieve_position', {}));
        setVBoard(displayBoard());
    }


    return (<div>
        <div
            onMouseMove={(e) => movePiece(e)}
            onMouseDown={(e) => grabPiece(e)}
            onMouseUp={(e) => dropPiece(e)}
            id="board"
            ref={boardRef}
        >{memoizedVisualBoard()}</div>
        <div>
            <input id="fen_string" class="fen" type="text" value="" maxLength={200}/>
            <button onClick={loadFen}>Load FEN</button>
        </div>
            <button onClick={makeMove}>Make Move</button>
        <button onClick={perft}>Perft</button>
        <button onClick={search}>Search</button>
    </div>);
}
