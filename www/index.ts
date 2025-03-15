import init, {World, Direction, GameStatus} from "snakegame";
import {rnd} from "./utils/rnd.js"

init().then(wasm => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 10;
    const SNAKE_SIZE = 3;
    const SNAKE_INDEX = rnd(WORLD_WIDTH * WORLD_WIDTH);
    const snakeWorld =  World.new(WORLD_WIDTH,SNAKE_INDEX,SNAKE_SIZE);

    const worldWidth = snakeWorld.width();

    const gameControlButton = document.getElementById("play-button");
    const gameStatusLabel = document.getElementById("game-status");
    const pointsLabel = document.getElementById("points");
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");

    gameControlButton.addEventListener("click", _ => {
        const status = snakeWorld.game_status();
        if(status == undefined){
            gameControlButton.textContent = "Playing..."
            snakeWorld.start_game();
            play();
        }
        else{
            location.reload();
        }

    })
    const ctx =  canvas.getContext("2d")

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

 

    function drawWorld(){
        ctx.beginPath();
            for(let x = 0; x <worldWidth+1 ;x++){
                ctx.moveTo(CELL_SIZE*x,0)
                ctx.lineTo(CELL_SIZE*x,worldWidth*CELL_SIZE);
            }

            for(let y = 0; y <worldWidth+1 ;y++){
                ctx.moveTo(0,CELL_SIZE*y)
                ctx.lineTo(worldWidth*CELL_SIZE,CELL_SIZE*y);
            }
        ctx.stroke();
    }


    document.addEventListener("keydown", (e) => {
        switch(e.code)
        {
            case "ArrowLeft":
                snakeWorld.change_snake_dir(Direction.Left);
                break;
            case "ArrowRight":
                snakeWorld.change_snake_dir(Direction.Right);
                break;
            case "ArrowUp":
                snakeWorld.change_snake_dir(Direction.Up);
                break;
            case "ArrowDown":
                snakeWorld.change_snake_dir(Direction.Down);
                break;
        }
    });

    function drawReward(){

        const idx = snakeWorld.reward_cell();

        const col = idx % worldWidth;
        const row = Math.floor(idx / worldWidth);

        ctx.beginPath();
        ctx.fillStyle ="rgb(0, 227, 11)"
        ctx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
           );
        ctx.stroke();
    }

    function drawSnake() {
        
        const snakeCellPtr = snakeWorld.snake_cells();;
        const snakeLength = snakeWorld.snake_length(); 

        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            snakeCellPtr,
            snakeLength
        );
             //console.log(snakeCells);

        snakeCells.filter((cellIndex, i) => !(i >0 && cellIndex === snakeCells[0]))
        .forEach((cellIndex,i) => {
            const col = cellIndex % worldWidth;
            const row = Math.floor(cellIndex / worldWidth);
    
            ctx.fillStyle = i === 0 ? "#7878db" : "#000000"
            ctx.beginPath();
               ctx.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
               );

        });

        ctx.stroke();
    }
    
    
    function drawGameStatus(){
        gameStatusLabel.textContent = snakeWorld.game_status_text();
        pointsLabel.textContent = snakeWorld.points().toString();
    }

    function drawDeathCell(){
        const idx = snakeWorld.death_cell();
        console.log(snakeWorld.death_cell_lifetime());
    
        if(idx)
        {
            const col = idx % worldWidth;
            const row = Math.floor(idx / worldWidth);
    
            ctx.beginPath();
            ctx.fillStyle ="rgb(255, 0, 0)";
            ctx.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
               );
            ctx.stroke();
        }
    }

    function paint() {
        drawWorld();
        drawSnake();
        drawDeathCell();
        drawReward();
        drawGameStatus();
    }

    function play() {
        const status = snakeWorld.game_status();

        if(status === GameStatus.Won || status === GameStatus.Lost)
            {
                gameControlButton.textContent = "Re-Play";
                return;
            }

        const fps = 12;
        setTimeout ( () => {
            ctx.clearRect(0,0,canvas.width,canvas.height);
            snakeWorld.step();
            paint();
            requestAnimationFrame(play);
        }, 1000/fps)
    }

    paint();
})