const NS_SVG = "http://www.w3.org/2000/svg";

const MARGIN_STAGE = 10;
const LINE_LENGTH = 40;
const NUMBER_FONT_SIZE = "25";
const NUMBER_FONT_Y_ADJUST = 10;

export class Stage {
    private v_line: LineItem[][] = [];
    private h_line: LineItem[][] = [];
    private width = 0;
    private height = 0;

    private numberInfo?: StageNumberInfo;
    private lineInfos: StageLineInfo[] = [];
    private lineInfoIndex: number = -1;

    private static lineArroundNumber: [number, number, boolean][] = [[0, 0, true], [0, 0, false], [0, 1, true], [1, 0, false]];
    private static lineArroundPoint: [number, number, boolean][] = [[0, 0, true], [0, 0, false], [-1, 0, true], [0, -1, false]];

    private puzzleLayer: SVGGElement;
    private decorationLayer: SVGGElement;

    public constructor(public stage: SVGElement) {
        this.puzzleLayer = document.createElementNS(NS_SVG, "g");
        this.decorationLayer = document.createElementNS(NS_SVG, "g");
        stage.append(this.puzzleLayer);
        stage.append(this.decorationLayer);
    }
    public init(width: number, height: number, data: string) {
        this.puzzleLayer.innerHTML = "";
        this.decorationLayer.innerHTML = "";
        this.v_line.length = 0;
        this.h_line.length = 0;
        this.width = width;
        this.height = height;

        // V lines
        for (let r = 0; r < height; r++) {
            this.v_line.push([]);
            for (let c = 0; c < width + 1; c++) {
                this.v_line[r].push(new LineItem(this.puzzleLayer, c, r, true));
            }
        }

        // H lines
        for (let r = 0; r < height + 1; r++) {
            this.h_line.push([]);
            for (let c = 0; c < width; c++) {
                this.h_line[r].push(new LineItem(this.puzzleLayer, c, r, false));
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
                    this.puzzleLayer.append(text);
                }
            }
        }

        // save number-info
        this.numberInfo = {
            numbers: numberStrings.map(v => {
                let num_line: number[] = [];
                for (let c = 0; c < width; c++) {
                    num_line.push(v[c] == " " ? -1 : v[c].charCodeAt(0) - '0'.charCodeAt(0));
                }
                return num_line;
            })
        }

        this.lineInfos.length = 0;
        this.lineInfoIndex = 0;

        this.saveLineInfo();
        this.assist();
        this.saveLineInfo();
    }

    private getClosestLine(x: number, y: number): {
        fx: number,
        fy: number,
        isVirtical: boolean,
        inFrame: boolean
    } {
        let stagePosX = (x - this.scrollX) / this.scale;
        let stagePosY = (y - this.scrollY) / this.scale;

        let fx = Math.floor((stagePosX - MARGIN_STAGE) / LINE_LENGTH);
        let rx = stagePosX - fx * LINE_LENGTH - MARGIN_STAGE;
        let fy = Math.floor((stagePosY - MARGIN_STAGE) / LINE_LENGTH);
        let ry = stagePosY - fy * LINE_LENGTH - MARGIN_STAGE;
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
            this.assist();
            this.saveLineInfo();

            if (this.checkCleared()) {
                this.complete();
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
            this.assist();
            this.saveLineInfo();
        }
    }

    public saveLineInfo() {
        this.lineInfos.length = this.lineInfoIndex + 1;
        this.lineInfos.push({
            h_line: this.h_line.map(u => u.map(v => {
                switch (v.getLineType()) {
                    case "line": return 1;
                    case "x": return 0;
                    case "none": return -1;
                }
            })),
            v_line: this.v_line.map(u => u.map(v => {
                switch (v.getLineType()) {
                    case "line": return 1;
                    case "x": return 0;
                    case "none": return -1;
                }
            }))
        });

        this.lineInfoIndex++;
    }

    public loadLineInfo(index: number) {
        if (index < 0 || index >= this.lineInfos.length) {
            return;
        }

        this.lineInfoIndex = index;
        let lineInfo = this.lineInfos[this.lineInfoIndex];
        function lineTypeToInfo(v: number) {
            switch (v) {
                case -1: return "none";
                case 0: return "x";
                case 1: return "line";
                default: return "none";
            }
        };

        this.v_line.map((v, r) => v.map((v, c) => {
            v.setLineType(lineTypeToInfo(lineInfo.v_line[r][c]));
        }));
        this.h_line.map((v, r) => v.map((v, c) => {
            v.setLineType(lineTypeToInfo(lineInfo.h_line[r][c]));
        }));
    }

    public onKey(ev: KeyboardEvent) {
        // svg element won't be active. so insteadly check if "body is active".
        if (document.activeElement == document.body) {
            ev.preventDefault();
            if (ev.ctrlKey) {
                if (ev.code == "KeyZ") {
                    this.loadLineInfo(this.lineInfoIndex - 1);
                } else if (ev.code == "KeyY") {
                    this.loadLineInfo(this.lineInfoIndex + 1);
                }
            }
        }
    }

    public scrollX: number = 0;
    public scrollY: number = 0;
    public scale: number = 1;
    public setDisplay() {
        this.puzzleLayer.setAttribute("transform", `translate(${this.scrollX} ${this.scrollY}) scale(${this.scale})`);
    }

    public scroll(x: number, y: number) {
        this.scrollX = x;
        this.scrollY = y;
        this.setDisplay();
    }

    public changeScale(scale: number, cx: number, cy: number) {
        // (cx - scrollx) / scale = (cx - new_scrollx) / new_scale
        // (cy - scrolly) / scale = (cy - new_scrolly) / new_scale
        this.scrollX = cx - (cx - this.scrollX) * scale / this.scale;
        this.scrollY = cy - (cy - this.scrollY) * scale / this.scale;
        this.scale = scale;
        this.setDisplay();
    }

    private getNumber(r: number, c: number) {
        return this.numberInfo?.numbers[r][c] ?? -1;
    }
    private getVLineType(r: number, c: number) {
        if (r < 0 || r >= this.height || c < 0 || c >= this.width + 1) {
            return "x";
        }
        return this.v_line[r][c].getLineType() ?? -1;
    }
    private getHLineType(r: number, c: number) {
        if (r < 0 || r >= this.height + 1 || c < 0 || c >= this.width) {
            return "x";
        }
        return this.h_line[r][c].getLineType() ?? -1;
    }
    private getLineType(r: number, c: number, isVirtical: boolean) {
        if (isVirtical) {
            return this.getVLineType(r, c);
        } else {
            return this.getHLineType(r, c);
        }
    }
    private setVLineType(r: number, c: number, type: "none" | "x" | "line") {
        this.v_line[r][c].setLineType(type);
    }
    private setHLineType(r: number, c: number, type: "none" | "x" | "line") {
        this.h_line[r][c].setLineType(type);
    }
    private setLineType(r: number, c: number, isVirtical: boolean, type: "none" | "x" | "line") {
        if (isVirtical) {
            return this.setVLineType(r, c, type);
        } else {
            return this.setHLineType(r, c, type);
        }
    }
    private *iterateAllNumbers(): Generator<[number, number]> {
        for (let r = 0; r < this.height; r++) {
            for (let c = 0; c < this.width; c++) {
                yield [r, c];
            }
        }
    }
    private *iterateAllPoints(): Generator<[number, number]> {
        for (let r = 0; r < this.height + 1; r++) {
            for (let c = 0; c < this.width + 1; c++) {
                yield [r, c];
            }
        }
    }
    private *iterateAllLines(): Generator<[number, number, boolean]> {
        for (let r = 0; r < this.height; r++) {
            for (let c = 0; c < this.width + 1; c++) {
                yield [r, c, true];
            }
        }
        for (let r = 0; r < this.height + 1; r++) {
            for (let c = 0; c < this.width; c++) {
                yield [r, c, false];
            }
        }
    }

    private assist() {
        let changed: boolean = false;

        while (true) {
            changed = false;

            // numbers
            for (let [r, c] of this.iterateAllNumbers()) {
                let num = this.getNumber(r, c);

                if (num >= 0) {
                    let countLine = 0;
                    let countNoLine = 0;
                    for (let [dr, dc, isVirtical] of Stage.lineArroundNumber) {
                        if (this.getLineType(r + dr, c + dc, isVirtical) == "line") {
                            countLine++;
                        } else if (this.getLineType(r + dr, c + dc, isVirtical) == "x") {
                            countNoLine++;
                        }
                    }

                    if (countLine + countNoLine == 4) {
                        continue;
                    } else if (countLine == num) {
                        for (let [dr, dc, isVirtical] of Stage.lineArroundNumber) {
                            if (this.getLineType(r + dr, c + dc, isVirtical) == "none") {
                                this.setLineType(r + dr, c + dc, isVirtical, "x");
                            }
                        }
                        changed = true;
                    } else if (countNoLine + num == 4) {
                        for (let [dr, dc, isVirtical] of Stage.lineArroundNumber) {
                            if (this.getLineType(r + dr, c + dc, isVirtical) == "none") {
                                this.setLineType(r + dr, c + dc, isVirtical, "line");
                            }
                        }
                        changed = true;
                    }
                }
            }

            // points
            for (let [r, c] of this.iterateAllPoints()) {
                let countLine = 0;
                let countNoLine = 0;
                for (let [dr, dc, isVirtical] of Stage.lineArroundPoint) {
                    if (this.getLineType(r + dr, c + dc, isVirtical) == "line") {
                        countLine++;
                    } else if (this.getLineType(r + dr, c + dc, isVirtical) == "x") {
                        countNoLine++;
                    }
                }

                if (countLine + countNoLine == 4) {
                    continue;
                } else if (countLine == 2 || countNoLine == 3) {
                    for (let [dr, dc, isVirtical] of Stage.lineArroundPoint) {
                        if (this.getLineType(r + dr, c + dc, isVirtical) == "none") {
                            this.setLineType(r + dr, c + dc, isVirtical, "x");
                        }
                    }
                    changed = true;
                } else if (countNoLine == 2 && countLine == 1) {
                    for (let [dr, dc, isVirtical] of Stage.lineArroundPoint) {
                        if (this.getLineType(r + dr, c + dc, isVirtical) == "none") {
                            this.setLineType(r + dr, c + dc, isVirtical, "line");
                        }
                    }
                    changed = true;
                }
            }

            if (!changed) {
                break;
            }
        }
    }

    private checkCleared(): boolean {
        // check number
        {
            for (let [r, c] of this.iterateAllNumbers()) {
                let num = this.getNumber(r, c);
                if (num >= 0) {
                    let countLine = 0;
                    for (let [dr, dc, isVirtical] of Stage.lineArroundNumber) {
                        if (this.getLineType(r + dr, c + dc, isVirtical) == "line") {
                            countLine++;
                        }
                    }
                    if (num != countLine) {
                        return false;
                    }
                }
            }
        }

        // check points
        {
            for (let [r, c] of this.iterateAllNumbers()) {
                let countLine = 0;
                for (let [dr, dc, isVirtical] of Stage.lineArroundPoint) {
                    if (this.getLineType(r + dr, c + dc, isVirtical) == "line") {
                        countLine++;
                    }
                }
                if (countLine == 1 || countLine == 3 || countLine == 4) {
                    return false;
                }
            }
        }

        // check global loop
        {
            let groupPoints: number[][] = [];
            let joinGroup = (num1: number, num2: number) => {
                if (num1 == num2) {
                    return;
                }
                for (let [r, c] of this.iterateAllPoints()) {
                    if (groupPoints[r][c] == num2) {
                        groupPoints[r][c] = num1;
                    }
                }
            }

            for (let r = 0; r < this.height + 1; r++) {
                groupPoints.push([]);
                for (let c = 0; c < this.width + 1; c++) {
                    groupPoints[r].push(-1);
                }
            }
            for (let [r, c] of this.iterateAllPoints()) {
                let countLine = 0;
                for (let [dr, dc, isVirtical] of Stage.lineArroundPoint) {
                    if (this.getLineType(r + dr, c + dc, isVirtical) == "line") {
                        countLine++;
                    }
                }
                if (countLine == 0) {
                    groupPoints[r][c] = -1;
                } else if (countLine == 2) {
                    groupPoints[r][c] = r * (this.width + 1) + c;
                }
            }

            for (let [r, c, isVirtical] of this.iterateAllLines()) {
                if (this.getLineType(r, c, isVirtical) == "line") {
                    if (isVirtical) {
                        joinGroup(groupPoints[r][c], groupPoints[r + 1][c]);
                    } else {
                        joinGroup(groupPoints[r][c], groupPoints[r][c + 1]);
                    }
                }
            }

            let group = -1;
            for (let [r, c] of this.iterateAllPoints()) {
                if (groupPoints[r][c] == -1 || groupPoints[r][c] == group) {
                    continue;
                } else if (group == -1) {
                    group = groupPoints[r][c];
                    continue;
                } else {
                    return false;
                }
            }
            if (group == -1) {
                return false;
            }
        }

        return true;
    }

    private completeText?: SVGTextElement;
    private complete() {
        if (this.completeText) {
            this.completeText.remove();
        }

        this.completeText = document.createElementNS(NS_SVG, "text");
        let stageRect = this.stage.getBoundingClientRect();

        this.completeText.innerHTML = "COMPLETE";
        this.completeText.setAttribute("x", (stageRect.width / 2).toString());
        this.completeText.setAttribute("y", (stageRect.height / 2).toString());
        this.completeText.setAttribute("transform", `rotate(-30 ${stageRect.width / 2}, ${stageRect.height / 2})`);
        this.completeText.classList.add("complete");
        this.decorationLayer.append(this.completeText);
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

    public getLineType() {
        return this.type;
    }
    public setLineType(type: "none" | "x" | "line") {
        this.type = type;
        this.redraw();
    }
}

type StageLineInfo = {
    h_line: number[][];
    v_line: number[][];
}
type StageNumberInfo = {
    numbers: number[][];
}

