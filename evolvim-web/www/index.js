import { Universe } from "evolvim-web";

const info = document.getElementById("evolvim-info");
const universe = Universe.new();
console.log(universe.width(), universe.height());

const renderLoop = () => {
    universe.update();
    info.textContent =
`time: ${universe.time()}
season: ${universe.season()}
width: ${universe.width()}
height: ${universe.height()}
creatures: ${universe.count_creatures()}
`;
    requestAnimationFrame(renderLoop);
}

renderLoop();
