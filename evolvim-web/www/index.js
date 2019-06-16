import { Universe } from "evolvim-web";
import { memory } from "evolvim-web/evolvim_web_bg"

const info = document.getElementById("evolvim-info");
const canvas = document.getElementById("evolvim-canvas");
const ctx = canvas.getContext("2d");
const universe = Universe.new();
const TILE_WIDTH = 10;
const width = universe.width();
const height = universe.height();
console.log(universe.width(), universe.height());

canvas.width = width * TILE_WIDTH;
canvas.height = height * TILE_WIDTH;

const drawTiles = () => {
    ctx.beginPath();

    for (let x = 0; x < width; ++x) {
        for (let y = 0; y < height; ++y) {
            const h = universe.tile_colour_hue(x, y);
            const s = universe.tile_colour_saturation(x, y);
            const b = universe.tile_colour_brightness(x, y);
            const a = universe.tile_colour_alpha(x, y);

            ctx.fillStyle = `hsla(${h * 360}, ${s * 100}%, ${b * 100}%, ${a})`;
            ctx.fillRect(
                x * TILE_WIDTH,
                y * TILE_WIDTH,
                TILE_WIDTH,
                TILE_WIDTH
            );
        }
    }

    ctx.stroke();
};

const renderLoop = () => {
    universe.update();
    info.textContent =
`time: ${universe.time()}
season: ${universe.season()}
width: ${universe.width()}
height: ${universe.height()}
creatures: ${universe.count_creatures()}
`;
    drawTiles();
    requestAnimationFrame(renderLoop);
}

renderLoop();
