import { defineStore } from "pinia";
import { verifyBetTxt } from "./util";

export const useConfigStore = defineStore("config", {
    
    state: () => ({
        board: [] as number[],
        oopRange: Array(169).fill(0) as number[],
        ipRange: Array(169).fill(0) as number[],

        startingPot: 40,
        effectiveStack: 100,
        rake: 0,
        rakeCap: 3,
        addAllInThreshold: 0,
        forceAllInThreshold: 0,

        betSizes: [
            [["", ""], ["", ""], ["", ""]], 
            [["", ""], ["", ""], ["", ""]], 
        ],
    }),

    getters: {
        rangeEmptyOOP: (s) => s.oopRange.every((w) => w === 0),
        rangeEmptyIP: (s) => s.ipRange.every((w) => w === 0),

        streetBetValidity: (s) => (street: number, player: number, raise: boolean) => {
            const r = raise ? 1 : 0;
            return verifyBetTxt(s.betSizes[player - 1][street - 1][r], raise);
        },

        configInValid: (s) => {
        
            if (s.oopRange.every((w) => w === 0) || s.ipRange.every((w) => w === 0)) {
                return true;
            }

            if (s.board.length < 3) {
                return true;
            }

            for (let street = 1; street <= 3; street++) {
                for (let player = 1; player <= 2; player++) {
                    for (let raise = 0; raise <= 1; raise++) {
                        if (verifyBetTxt(s.betSizes[player - 1][street - 1][raise], raise === 1).validity !== 1) {
                            return true;
                        }
                    }
                }
            }
            return false;
        },
    },

    actions: {
        
        setWeight(idx: number, weight: number, oop: boolean): void {
            oop ? this.oopRange[idx] = weight : this.ipRange[idx] = weight;
        },

        clearRange(oop: boolean): void {
            oop ? this.oopRange.fill(0) : this.ipRange.fill(0);
        },

        clearBoard(): void {
            this.board = [];
        },

        setRandomBoard(n: number): void {
            let newBoard: number[] = [];
            for (let i = 0; i < n; i++) {
                let card = Math.floor(Math.random() * 52);
                while (newBoard.includes(card)) {
                    card = Math.floor(Math.random() * 52);
                }
                newBoard.push(card);
            }
            this.board = newBoard;
        },

        addToBoard(card: number): void {
            if (this.board.includes(card)) {
                this.board = this.board.filter((c) => c !== card);
            } else if (this.board.length < 5) {
                this.board.push(card);
            }
        },

        copyBets(): void {
            this.betSizes[1] = this.betSizes[0].map((street) => [...street]);
        },
        
    },
});

export type MainPanel = "config" | "results";
export type ConfigPanel = "rangeOOP" | "rangeIP" | "treeConfig" | "board" | "run";

export const useStore = defineStore("app", {
    state: () => ({
        mainPanel: "config" as MainPanel,
        configPanel: "rangeOOP" as ConfigPanel,
        treeBuilt: false,
        solverRun: false,
    }),
});