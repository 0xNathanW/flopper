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
            [["50%", "50%"], ["50%", "50%"], ["50%", "50%"]], 
            [["50%", "50%"], ["50%", "50%"], ["50%", "50%"]], 
        ],
    }),

    getters: {

        rangeEmptyOOP: (s) => s.oopRange.every((w) => w === 0),
        rangeEmptyIP: (s) => s.ipRange.every((w) => w === 0),

        streetBetValidity: (s) => (street: number, player: number, raise: boolean) => {
            const r = raise ? 1 : 0;
            return verifyBetTxt(s.betSizes[player - 1][street - 1][r], raise);
        },

        isInvalid: (s) => {

            // Check if either range is empty.
            if (s.oopRange.every((w) => w === 0)) {
                return "OOP range is empty.";
            }

            if (s.ipRange.every((w) => w === 0)) {
                return "IP range is empty.";
            }

            // Check we have at least 3 board cards.
            if (s.board.length < 3) {
                return "Board must have at least 3 cards.";
            }

            for (let street = 1; street <= 3; street++) {
                for (let player = 1; player <= 2; player++) {
                    for (let raise = 0; raise <= 1; raise++) {
                        if (verifyBetTxt(s.betSizes[player - 1][street - 1][raise], raise === 1).validity !== 1) {
                            return `Invalid bet size on street ${street} for player ${player}.`;
                        }
                    }
                }
            }

            if (s.startingPot <= 0) {
                return "Starting pot must be positive.";
            }

            if (s.effectiveStack <= 0) {
                return "Effective stack must be positive.";
            }

            if (s.rake < 0 || s.rake > 100) {
                return "Rake must be between 0% and 100%.";
            }

            if (s.rakeCap < 0) {
                return "Rake cap must be positive.";
            }

            if (s.addAllInThreshold < 0) {
                return "Add all-in threshold must be positive.";
            }

            if (s.forceAllInThreshold < 0) {
                return "Force all-in threshold must be positive.";
            }

            return "";
        },

        configHash: (s) => {
            
            const text = JSON.stringify({
                oopRange: s.oopRange,
                ipRange: s.ipRange,
                board: s.board,
                betSizes: s.betSizes,
            });

            let hash = 0;
            for (let i = 0; i < text.length; i++) {
                const chr = text.charCodeAt(i);
                hash = ((hash << 5) - hash) + chr;
                hash |= 0;
            }

            return Math.abs(hash);
        },
    },

    actions: {
        
        // Ranges
        setWeight(idx: number, weight: number, oop: boolean): void {
            oop ? this.oopRange[idx] = weight : this.ipRange[idx] = weight;
        },

        clearRange(oop: boolean): void {
            oop ? this.oopRange.fill(0) : this.ipRange.fill(0);
        },

        clearBoard(): void {
            this.board = [];
        },

        // Board
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

        // Bet sizes
        copyBets(): void {
            this.betSizes[1] = this.betSizes[0].map((street) => [...street]);
        },
        
    },
});

export type MainPanel = "results" | "config" | "settings";
export type ConfigPanel = "rangeOOP" | "rangeIP" | "treeConfig" | "board" | "run" | "preview";
export type ConfigModal = "none" | "rangeOOP" | "rangeIP";

const lightThemes = [
    "light", "cupcake", "bumblebee", 
    "emerald", "corporate", "retro", 
    "cyberpunk", "valentine", "garden",
    "lofi", "pastel", "fantasy", 
    "wireframe", "cmyk", "autumn", 
    "acid", "lemonade", "winter",
]

export const useStore = defineStore("app", {
    state: () => ({
        theme: "dark",
        mainPanel: "config" as MainPanel,
        configPanel: "rangeOOP" as ConfigPanel,
        configModal: "none" as ConfigModal,
        treeHash: 0,
        solverRunning: false,
        solverPaused: false,
        solverFinished: false,
        solverError: false,
    }),

    getters: {
        solverRun: (s) => {
            return (
                s.solverRunning ||
                s.solverPaused ||
                s.solverFinished ||
                s.solverError
            );
        },
        contrastText: (s) => {
            return lightThemes.includes(s.theme) ? "black" : "white";
        }
    },
});