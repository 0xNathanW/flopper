import { type } from "os";

export const RANKS: string[] = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

export type ActionNode = ActionNodeRoot | ActionNodeChance | ActionNodePlayer | ActionNodeTerminal;

export type ActionNodeRoot = {
    type: "root";
    idx: 0;
    player: "flop" | "turn" | "river";
    selectedIdx: -1;
    board: number[];
    pot: number;
    stack: number;
};

export type ActionNodeChance = {
    type: "chance";
    idx: number;
    player: "turn" | "river";
    selectedIdx: number;
    lastPlayer: "oop" | "ip";
    cards: {
        card: number;
        selecte: boolean;
        dead: boolean;
    }[];
    pot: number;
    stack: number;
};

export type ActionNodePlayer = {
    type: "player";
    idx: number;
    player: "oop" | "ip";
    selectedIdx: number;
    actions: {
        idx: number;
        name: string;
        amount: number;
        selected: boolean;
        colour: string;
    }[];
};

export type ActionNodeTerminal = {
    type: "terminal";
    idx: number;
    player: "end";
    selectedIdx: -1;
    lastPlayer: "oop" | "ip";
    equityOOP: number;
    pot: number;
};