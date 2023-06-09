import { Stage } from "./stage";

let make_puzzle_web: (size_r: number, size_c: number) => string;
let make_puzzle_web2: (size_r: number, size_c: number) => string;
import("../node_modules/loop_puzzle_web/loop_puzzle_web.js").then(async (js) => {
    make_puzzle_web = (await js.default).make_puzzle_web;
    make_puzzle_web2 = (await js.default).make_puzzle_web2;
});

let stage: Stage;

async function init() {
    let stageSvg = document.getElementById("stage") as unknown as SVGElement;
    stage = new Stage(stageSvg);

    let preventClick = false;
    stageSvg.addEventListener("click", (ev) => {
        if (!preventClick) {
            let stageRect = stageSvg.getBoundingClientRect();
            stage.click(
                ev.clientX - stageRect.left,
                ev.clientY - stageRect.top
            );
        } else {
            preventClick = false;
        }
    });

    stageSvg.addEventListener("contextmenu", (ev) => {
        ev.preventDefault();
        if (!preventClick) {
            let stageRect = stageSvg.getBoundingClientRect();
            stage.rclick(
                ev.clientX - stageRect.left,
                ev.clientY - stageRect.top
            );
        } else {
            preventClick = false;
        }
    });

    document.addEventListener("keydown", (ev) => {
        // svg element won't be active. so insteadly check if "body is active".
        if (document.activeElement == document.body) {
            ev.preventDefault();
            if (ev.ctrlKey) {
                if (ev.code == "KeyZ") {
                    stage.loadLineInfo(stage.lineInfoIndex - 1);
                } else if (ev.code == "KeyY") {
                    stage.loadLineInfo(stage.lineInfoIndex + 1);
                }
            }
        }
    });

    { // scroll
        let dragging = false;
        let dragX = 0;
        let dragY = 0;
        let scrollX = 0;
        let scrollY = 0;
        stageSvg.addEventListener("mousedown", (ev) => {
            dragging = true;
            dragX = ev.clientX;
            dragY = ev.clientY;
            scrollX = stage.scrollX;
            scrollY = stage.scrollY;
        });
        window.addEventListener("mousemove", (ev) => {
            if (dragging) {
                preventClick = true;
                stage.scroll(ev.clientX - dragX + scrollX, ev.clientY - dragY + scrollY);
            }
        });
        window.addEventListener("mouseup", () => {
            dragging = false;
        })
    }

    { // scale
        stageSvg.addEventListener("wheel", (ev) => {
            ev.preventDefault();
            let stageRect = stageSvg.getBoundingClientRect();
            let cx = ev.clientX - stageRect.left;
            let cy = ev.clientY - stageRect.top;

            if (ev.deltaY > 0) {
                stage.scaleUp(cx, cy);
            } else {
                stage.scaleDown(cx, cy);
            }
        });
    }

    document.getElementById("new_puzzle")!.addEventListener("click", () => {
        let width = Number((document.getElementById("stage_width") as HTMLInputElement).value);
        let height = Number((document.getElementById("stage_height") as HTMLInputElement).value);

        let puzzle_string = make_puzzle_web2(height, width);

        stage.init(width, height, puzzle_string);
    });
    document.getElementById("undo")!.addEventListener("click", () => {
        stage.loadLineInfo(stage.lineInfoIndex - 1);
    });
    document.getElementById("redo")!.addEventListener("click", () => {
        stage.loadLineInfo(stage.lineInfoIndex + 1);
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
