
export const RANKS = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
export const SUITS = ["♣", "♦", "♥", "♠"];

export function cardToIdx(card: string): number {
    return (12 - RANKS.indexOf(card[0])) * 4 + SUITS.indexOf(card[1]);
}

export function idxToCard(idx: number): string {
    const rank = RANKS[12 - Math.floor(idx / 4)];
    const suit = "♣♦♥♠"[idx % 4];
    return rank + suit;
}

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
}

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
}

// validity: 0 = empty, 1 = valid, 2 = invalid
export function verifyBetTxt(s: string, raise: boolean): {text: string, validity: number} {

    const betTxt = raise ? "Raise" : "Bet";

    const trimmed = s.split(",").map((bet) => {
        return bet.trim().toLowerCase();
    }).filter((bet) => {
        return !(bet === "")
    });

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
}