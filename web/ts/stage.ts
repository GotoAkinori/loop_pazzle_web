const NS_SVG = "http://www.w3.org/2000/svg";

const MARGIN_STAGE = 10;
const LINE_LENGTH = 40;
const NUMBER_FONT_SIZE = "25";
const NUMBER_FONT_Y_ADJUST = 10;

export class Stage {
    v_line: LineItem[][] = [];
    h_line: LineItem[][] = [];
    width = 0;
    height = 0;

    public constructor(public stage: SVGElement) {
        stage.addEventListener("click", (ev) => {
            let stageRect = stage.getBoundingClientRect();
            this.click(
                ev.clientX - stageRect.left,
                ev.clientY - stageRect.top
            );
        });
        stage.addEventListener("contextmenu", (ev) => {
            ev.preventDefault();
            let stageRect = stage.getBoundingClientRect();
            this.rclick(
                ev.clientX - stageRect.left,
                ev.clientY - stageRect.top
            );
        });
    }
    public init(width: number, height: number, data: string) {
        this.stage.innerHTML = "";
        this.v_line.length = 0;
        this.h_line.length = 0;
        this.width = width;
        this.height = height;

        // V lines
        for (let r = 0; r < height; r++) {
            this.v_line.push([]);
            for (let c = 0; c < width + 1; c++) {
                this.v_line[r].push(new LineItem(this.stage, c, r, true));
            }
        }

        // H lines
        for (let r = 0; r < height + 1; r++) {
            this.h_line.push([]);
            for (let c = 0; c < width; c++) {
                this.h_line[r].push(new LineItem(this.stage, c, r, false));
            }
        }

        // numbers
        let numberStrings = data.split("|");
        for (let r = 0; r < height; r++) {
            for (let c = 0; c < width; c++) {
                if (numberStrings[r][c] != " ") {
                    let text = document.createElementNS(NS_SVG, "text");
                    text.innerHTML = numberStrings[r][c];
                    text.setAttribute("font-size", NUMBER_FONT_SIZE);
                    text.setAttribute("x", (MARGIN_STAGE + (c + 0.5) * LINE_LENGTH).toString());
                    text.setAttribute("y", (MARGIN_STAGE + (r + 0.5) * LINE_LENGTH + NUMBER_FONT_Y_ADJUST).toString());
                    text.setAttribute("text-anchor", "middle");
                    this.stage.append(text);
                }
            }
        }
    }

    private getClosestLine(x: number, y: number): {
        fx: number,
        fy: number,
        isVirtical: boolean,
        inFrame: boolean
    } {
        let fx = Math.floor((x - MARGIN_STAGE) / LINE_LENGTH);
        let rx = x - fx * LINE_LENGTH - MARGIN_STAGE;
        let fy = Math.floor((y - MARGIN_STAGE) / LINE_LENGTH);
        let ry = y - fy * LINE_LENGTH - MARGIN_STAGE;
        let isVirtical: boolean;

        if (rx < ry && rx + ry < LINE_LENGTH) {
            isVirtical = true;
        } else if (rx + ry < LINE_LENGTH) {
            isVirtical = false;
        } else if (rx < ry) {
            isVirtical = false;
            fy++;
        } else {
            isVirtical = true;
            fx++;
        }

        return {
            fx: fx,
            fy: fy,
            isVirtical: isVirtical,
            inFrame:
                (isVirtical && fx >= 0 && fx < this.width + 1 && fy >= 0 && fy < this.height) ||
                (!isVirtical && fx >= 0 && fx < this.width && fy >= 0 && fy < this.height + 1)
        }
    }

    public click(x: number, y: number) {
        let {
            fx, fy, isVirtical, inFrame
        } = this.getClosestLine(x, y);

        if (inFrame) {
            if (isVirtical) {
                this.v_line[fy][fx].click();
            } else {
                this.h_line[fy][fx].click();
            }
        }
    }

    public rclick(x: number, y: number) {
        let {
            fx, fy, isVirtical, inFrame
        } = this.getClosestLine(x, y);

        if (inFrame) {
            if (isVirtical) {
                this.v_line[fy][fx].rclick();
            } else {
                this.h_line[fy][fx].rclick();
            }
        }
    }
}

export class LineItem {
    public type: "none" | "x" | "line" = "none";
    public line: SVGLineElement;

    public constructor(public stage: SVGElement, x: number, y: number, public isVirtical: boolean) {
        this.line = document.createElementNS(NS_SVG, "line");
        stage.append(this.line);
        if (this.isVirtical) {
            this.line.setAttribute("x1", (MARGIN_STAGE + x * LINE_LENGTH).toString());
            this.line.setAttribute("y1", (MARGIN_STAGE + y * LINE_LENGTH).toString());
            this.line.setAttribute("x2", (MARGIN_STAGE + x * LINE_LENGTH).toString());
            this.line.setAttribute("y2", (MARGIN_STAGE + (y + 1) * LINE_LENGTH).toString());
        } else {
            this.line.setAttribute("x1", (MARGIN_STAGE + x * LINE_LENGTH).toString());
            this.line.setAttribute("y1", (MARGIN_STAGE + y * LINE_LENGTH).toString());
            this.line.setAttribute("x2", (MARGIN_STAGE + (x + 1) * LINE_LENGTH).toString());
            this.line.setAttribute("y2", (MARGIN_STAGE + y * LINE_LENGTH).toString());
        }
        this.redraw();
    }

    public click() {
        if (this.type == "line") {
            this.type = "none";
        } else {
            this.type = "line";
        }

        this.redraw();
    }
    public rclick() {
        if (this.type == "x") {
            this.type = "none";
        } else {
            this.type = "x";
        }

        this.redraw();
    }

    public redraw() {
        this.line.classList.remove("none");
        this.line.classList.remove("line");
        this.line.classList.remove("x");
        this.line.classList.add(this.type);
    }
}
