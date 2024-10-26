import './Board.css';
import Tile from './Tile';
import { createEffect, createMemo, createSignal, onMount } from 'solid-js';
import init, { ClientEngine } from './../../shinobi_web.js';

const files = ["window 1", "2", "3", "4", "5", "6", "7", "8"];
const ranks = ["a", "b", "c", "d", "e", "f", "g", "h"];

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


interface Props {
    engine: ClientEngine
}

export default function Board({ engine }: Props) {

    const [fen, setFen] = createSignal(START_FEN);
    const [board, setBoard] = createSignal(startPos);

    // Create a memoized visual board that reacts to changes in the board state
    const memoizedVisualBoard = createMemo(() => {
        return ranks.map((_, rank) => (
            files.map((_, file) => {
                const color = (rank + file) % 2 === 0;
                let piece: string = board()[rank][file];
                let img_src: string = pieceToStr(piece) as string;
                return <Tile dark={color} image={img_src} />;
            })
        ));
    });

    let activePiece: HTMLElement | null = null;
    let old_piece_x: number;
    let old_piece_y: number;
    let boardRef: HTMLDivElement;

    console.log("ENGINE: " + engine);

    // PIECE MOVEMENT
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
            activePiece = null;
        }
    }

    function updateBoard() {
        setBoard(engine.recieve_position());
    }

    function makeMove() {
        let moves = engine.moves();
        let mv = moves[0];
        engine.make_move(mv);
        setBoard(engine.recieve_position());
        console.log("MOVE MADE", mv);
    }

    function resetPosition() {
        console.log("RESET POSITION");
        engine.reset_position();
        setBoard(engine.recieve_position());
    }

    createEffect(() => {
        console.log("BOARD HAS CHANGED", board());
    });

    onMount(() => resetPosition());

    function loadFen() {
        let input = document.getElementById("fen_string") as HTMLInputElement;
        setFen(input.value);
        engine.load_fen(fen());
        let pos = engine.recieve_position();
        console.log("RECEIVE", pos);
        setBoard(pos);
    }

    async function search() {
        for (let i = 0; i < 10; i++) {
            let best_mv = engine.search();
            if (best_mv != null) {
                engine.make_move(best_mv);
                updateBoard();
            }

            // Make it so i can see moves being made
            await sleep(500);
        }
    }

    function perft(depth: number) {
        console.log("NODES: " + engine.start_perft(depth));
        console.log(board());
    }

    function sleep(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
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
            <input id="fen_string" class="fen" type="text" value="" maxLength={200} />
            <button onClick={loadFen}>Load FEN</button>
        </div>
        <button onClick={makeMove}>Make Move</button>
        <button onClick={(_e) => perft(4)}>Perft</button>
        <button onClick={search}>Search</button>
        <button onClick={resetPosition}>Reset Position</button>
    </div>);
}
