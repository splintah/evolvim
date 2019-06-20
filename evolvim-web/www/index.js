import { Universe } from "evolvim-web";
import { memory } from "evolvim-web/evolvim_web_bg"

const info = document.getElementById("evolvim-info");
const canvas = document.getElementById("evolvim-canvas");
const ctx = canvas.getContext("2d");
let universe = Universe.new();
const TILE_WIDTH = 20;
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

    for (let i = 0, count = universe.count_creatures(); i < count; ++i) {
        const x = universe.creature_px(i);
        const y = universe.creature_py(i);
        const radius = universe.creature_radius(i);
        ctx.fillStyle = "#fff";
        ctx.fillRect(
            x * TILE_WIDTH,
            y * TILE_WIDTH,
            3 * radius * TILE_WIDTH,
            3 * radius * TILE_WIDTH
        );
    }

    ctx.stroke();
};

const renderLoop = () => {
    universe.update();
    universe.prepare_for_drawing();
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

const readFromFile = event => {
    let input = event.target;

    let reader = new FileReader();
    reader.onload = function(){
        let arrayBuffer = reader.result;
        let byteview = new Uint8Array(arrayBuffer);

        universe = Universe.from_bytes(byteview);
        console.log("Loaded the file!")
    };

    console.log("Loading file...");
    reader.readAsArrayBuffer(input.files[0]);
}

const readFromUrl = url => {
    var xhttp = new XMLHttpRequest();
    xhttp.responseType = "arraybuffer";

    xhttp.onreadystatechange = function() {
        if (xhttp.readyState == 4 && xhttp.status == 200) {

            let arrayBuffer = xhttp.response;
            let byteview = new Uint8Array(arrayBuffer);

            universe = Universe.from_bytes(byteview);
        }
    };

    xhttp.open("GET", url, true);
    xhttp.send();
}

document.getElementById('file-loader').addEventListener("change", readFromFile);
document.getElementById('default-file-loader').addEventListener("click", () => readFromUrl("test.bin"));
