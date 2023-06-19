
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