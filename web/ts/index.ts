import { Stage } from "./stage";

let make_puzzle_web: (size_r: number, size_c: number) => string;
let make_puzzle_web2: (size_r: number, size_c: number) => string;
import("../node_modules/loop_puzzle_web/loop_puzzle_web.js").then(async (js) => {
    make_puzzle_web = (await js.default).make_puzzle_web;
    make_puzzle_web2 = (await js.default).make_puzzle_web2;
});

let stage: Stage;

async function init() {
    stage = new Stage(document.getElementById("stage") as unknown as SVGElement);

    document.getElementById("new_puzzle")!.addEventListener("click", () => {
        let width = Number((document.getElementById("stage_width") as HTMLInputElement).value);
        let height = Number((document.getElementById("stage_height") as HTMLInputElement).value);

        let puzzle_string = make_puzzle_web2(height, width);

        stage.init(width, height, puzzle_string);
    });
}

function waitLoad() {
    if (document.body) {
        init();
    }
    else {
        setTimeout(waitLoad, 10);
    }
}
waitLoad();
