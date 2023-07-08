
export const RANKS = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
export const SUITS = ["♣", "♦", "♥", "♠"];
const suitClasses = [
    "text-green-600",
    "text-blue-600",
    "text-pink-600",
    "text-black",
  ];

export function cardToIdx(card: string): number {
    return (12 - RANKS.indexOf(card[0])) * 4 + SUITS.indexOf(card[1]);
};

export function idxToCard(idx: number): string {
    const rank = RANKS[12 - Math.floor(idx / 4)];
    const suit = "♣♦♥♠"[idx % 4];
    return rank + suit;
};

const ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J","Q", "K", "A"];

export const cardText = (card: number) => {
    return {
      rank: ranks[card >>> 2],
      suit: SUITS[card & 3],
      colorClass: suitClasses[card & 3],
    };
};

export const cardPairCellIndex = (card1: number, card2: number) => {
    if (card1 > card2) [card1, card2] = [card2, card1];
    const hr = card2 >>> 2;
    const lr = card1 >>> 2;
    const hs = card2 & 3;
    const ls = card1 & 3;
    const isSuited = hs === ls;
    return {
      row: 12 - (isSuited ? hr : lr),
      col: 12 - (isSuited ? lr : hr),
      index: isSuited
        ? 3 - hs
        : hr === lr
        ? 6 - ((ls * (5 - ls)) / 2 + hs)
        : 11 - (3 * hs + ls - +(hs < ls)),
    };
};
  
export const cardPairOrder = (pair: number) => {
    let card1 = pair & 0xff;
    let card2 = pair >>> 8;
    if (card2 === 0xff) return card1;
    if (card1 > card2) [card1, card2] = [card2, card1];
    const hr = card2 >>> 2;
    const lr = card1 >>> 2;
    let hs = card2 & 3;
    let ls = card1 & 3;
    const isPair = hr === lr;
    const isSuited = hs === ls;
    if (isPair) [hs, ls] = [ls, hs];
    return ((((hr * 2 + +isPair) * 2 + +isSuited) * 16 + lr) * 4 + hs) * 4 + ls;
};

export function suitColour(suit: string): string {
    if (suit === "♦") {
        return "text-blue-700";
    } else if (suit === "♣") {
        return "text-green-700";
    } else if (suit === "♥") {
        return "text-red-700";
    } else {
        return "text-black";
    }
};

export function rgbToString(colour :{
    r: number;
    g: number;
    b: number;
}) {
    const red = colour.r.toString(16).padStart(2, "0");
    const green = colour.g.toString(16).padStart(2, "0");
    const blue = colour.b.toString(16).padStart(2, "0");
    return `#${red}${green}${blue}`;
};

export function average(vals: number[], weights: number[]) {
    let sum = 0;
    let totalWeight = 0;
    for (let i = 0; i < vals.length; ++i) {
        sum += vals[i] * weights[i];
        totalWeight += weights[i];
    }
    return sum / totalWeight;
}; 

export const toFixed1 = (value: number) => {
    if (!isFinite(value)) return (value < 0 ? "-" : "") + "∞";
    if (-0.05 < value && value < 0.05) return "0.0";
    return value.toFixed(1);
};

export const toFixed2 = (value: number) => {
    if (-0.005 < value && value < 0.005) return "0.00";
    return value.toFixed(2);
};

export const toFixed3 = (value: number) => {
    if (-0.0005 < value && value < 0.0005) return "0.000";
    return value.toFixed(3);
};

export const toFixed = [toFixed1, toFixed2, toFixed3];

export const toFixedAdaptive = (value: number) => {
    const abs = Math.abs(value);
    if (abs < 10) return toFixed3(value);
    if (abs < 100) return toFixed2(value);
    return toFixed1(value);
};

export function textToRange(text: string): number[] {

    const weights = Array(169).fill(0);
    const elems = text
        .replace(/\s*([-:,])\s*/g, '$1')
        .split(',')
        .map(elem => elem.trim());
    
    for (let elem of elems) {
  
        if (elem.includes('+')) {
            const idx1 = RANKS.indexOf(elem[0]);
            const idx2 = RANKS.indexOf(elem[1]);
            
            // Pair+
            if (elem.length === 3) {
                for (let i = 0; i <= idx1; i++) {
                    weights[i * 13 + i] = 100;
                }
        
            // Suited+
            } else if (elem[2] === 's') {
                for (let i = idx1; i <= idx2; i++) {
                    weights[idx1 * 13 + i] = 100;
                }
            
              // Offsuit+
            } else if (elem[2] === 'o') {
                for (let i = idx1 + 1; i <= idx2; i++) {
                    weights[i * 13 + idx1] = 100;
                }
            }

        } else if (elem.includes('-')) {
            
            const split = elem.split('-');

            const idx11 = RANKS.indexOf(split[0][0]);
            const idx12 = RANKS.indexOf(split[0][1]);

            const idx21 = RANKS.indexOf(split[1][0]);
            const idx22 = RANKS.indexOf(split[1][1]);

            // Pair-Pair
            if (split[0].length === 2) {
                for (let i = idx11; i <= idx21; i++) {
                    weights[i * 13 + i] = 100;
                }
            }

            // Suited-Suited
            else if (split[0][2] === 's') {
                for (let i = idx11; i <= idx21; i++) {
                    for (let j = idx22; j <= idx12; j++) {
                        weights[i * 13 + j] = 100;
                    }
                }
            }

            // Offsuit-Offsuit
            else if (split[0][2] === 'o') {
                for (let i = idx11; i <= idx21; i++) {
                    for (let j = idx12; j <= idx22; j++) {
                        weights[i * 13 + j] = 100;
                    }
                }
            }
        } else {

            const idx1 = RANKS.indexOf(elem[0]);
            const idx2 = RANKS.indexOf(elem[1]);

            // Pair
            if (elem.length === 2) {
                weights[idx1 * 13 + idx1] = 100;
            }

            // Suited
            else if (elem[2] === 's') {
                weights[idx1 * 13 + idx2] = 100;
            }

            // Offsuit
            else if (elem[2] === 'o') {
                weights[idx2 * 13 + idx1] = 100;
            }
        }
    }
    return weights;
};

// validity: 0 = empty, 1 = valid, 2 = invalid
export function verifyBetTxt(s: string, raise: boolean): {text: string, validity: number} {

    const betTxt = raise ? "Raise" : "Bet";

    const trimmed = s.split(",").map((bet) => {
        return bet.trim().toLowerCase();
    }).filter((bet) => {
        return !(bet === "")
    });

    // No bets.
    if (trimmed.length === 0) {
        return {text: "", validity: 0};
    }

    for (let bet of trimmed) {

        if (bet === "allin" || bet === "a") {
            
        } else if (bet.includes("e")) {

            const split = bet.split("e");
            const num_streets = Number(split[0]);
            const max_pot = Number(split[1]);
            
            if (!(split[0] === "")) {
                if (isNaN(num_streets) || num_streets < 1 || num_streets > 100 || !Number.isInteger(num_streets)) {
                    return {text: "Geometric Bet: Number of streets must be an integer between 1 and 100.  Found: " + num_streets, validity: 2};
                }
            }

            if (!(split[1] === "")) {
                if (isNaN(max_pot)) {
                    return {text: "Geometric Bet: Maximum pot limit must be a number. Found: " + max_pot, validity: 2};
                }
            }

        } else {
            switch (bet[bet.length - 1]) {
                case "x":
                    if (!raise) {
                        return {text: "Scaled Bet: Can only use 'x' for raises", validity: 2};
                    }
                    const betN = Number(bet.slice(0, -1));
                    if (isNaN(betN) || betN < 0) {
                        return {text: "Scaled Bet: Must be a positive integer. Found: " + betN, validity: 2};
                    }
                    break;
                
                case "c":
                    const cN = Number(bet.slice(0, -1));
                    if (isNaN(cN) || cN < 0) {
                        return {text: "Constant Bet: Must be a positive integer. Found: " + cN, validity: 2};
                    }
                    break;
                
                case "%": 
                    const pctN = Number(bet.slice(0, -1));
                    if (isNaN(pctN) || pctN < 0) {
                        return {text: "Percentage Bet: Must be a positive integer. Found: " + pctN, validity: 2};
                    }
                    break;
                
                default:
                    return {text: `Invalid ${betTxt}: Must end in 'x', 'c', or '%' or be 'allin'/'a'. Found: ${bet[bet.length - 1]}`, validity: 2};
            }
        }
    };

    return {text: trimmed.join(", "), validity: 1};
};