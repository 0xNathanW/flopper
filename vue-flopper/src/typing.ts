
export type Results = {
    currentPlayer: "oop" | "ip" | "chance" | "terminal";
    numActions: number;
    empty: number;
    eqrBase: number[];
    weights: number[][];
    normaliser: number[][];
    equity: number[][];
    ev: number[][];
    eqr: number[][];
    strategy: number[];
    actionEv: number[];
};

export type ChanceReport = {
    currentPlayer: "oop" | "ip" | "terminal";
    numActions: number;
    status: number[];
    combos: number[][];
    equity: number[][];
    ev: number[][];
    eqr: number[][];
    strategy: number[];
};

export const resultPanelList = [
    "basics",
    "graphs",
    "compare",
    "chance",
];

export type ResultsPanel = (typeof resultPanelList)[number];

export const playerBasicsList = ["auto", "oop", "ip"] as const;
export const playerChanceList = ["auto", "oop", "ip"] as const;
export const barHeightList = ["normalised", "absolute", "full"] as const;
export const suitList = ["grouped", "individual"] as const;
export const strategyList = ["show", "none"] as const;
export const contentBasicsList = ["default", "eq", "ev", "eqr"] as const;
export const contentGraphsList = ["eq", "ev", "eqr"] as const;
export const chartChanceList = [
  "strategy-combos",
  "strategy",
  "eq",
  "ev",
  "eqr",
] as const;

export type ResultsOpts = {
    playerBasics: (typeof playerBasicsList)[number];
    playerChance: (typeof playerChanceList)[number];
    barHeight: (typeof barHeightList)[number];
    suit: (typeof suitList)[number];
    strategy: (typeof strategyList)[number];
    contentBasics: (typeof contentBasicsList)[number];
    contentGraphs: (typeof contentGraphsList)[number];
    chartChance: (typeof chartChanceList)[number];
};

export type HoverContent = {
    name: string;
    idxs: number[];
};

export type TableMode = "basics" | "graphs" | "chance";

export type ActionNode = RootNode | ChanceNode | PlayerNode | TerminalNode;

export type RootNode = {
    type: 'root',
    idx: 0;
    player: "flop" | "turn" | "river";
    selectedIdx: -1;
    board: number[];
    pot: number;
    stack: number;
};

export type ChanceNode = {
    type: 'chance',
    idx: number;
    player: "turn" | "river";
    selectedIdx: number;
    prevPlayer: "oop" | "ip";
    cards: {
        card: number;
        selected: boolean;
        dead: boolean;
    }[];
    pot: number;
    stack: number;
};

export type PlayerNode = {
    type: 'player',
    idx: number;
    player: "oop" | "ip";
    selectedIdx: number;
    actions: {
        idx: number;
        name: string;
        amount: string;
        selected: boolean;
        colour: string;
    }[];
};

export type TerminalNode = {
    type: 'terminal',
    idx: number;
    player: "end";
    selectedIdx: -1;
    prevPlayer: "oop" | "ip";
    equityOOP: number;
    pot: number;
};