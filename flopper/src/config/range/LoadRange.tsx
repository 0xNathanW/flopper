import { RANKS } from "../../common";
import "./LoadRange.css";

export default function LoadRange({setWeights}: {setWeights: (weights: number[]) => void}) {

    const handleClick = (s: string) => {
        setWeights(textToRange(s));
    }

    return (
        <div id="load-range">
            <div id="tree-div">
                <h1>Load Range</h1>
                <ul className="tree">
                    <li>6-Max PokerStrategy.com
                        <summary>
                            <ul>
                                <li>
                                    <details>
                                        <summary>Open Raise - Standard</summary>
                                        <ul>
                                            <li><a onClick={() => handleClick("22+,ATs+,KQs,AJo+, KQo")}>MP2</a></li>
                                            <li><a onClick={() => handleClick("22+,ATs+,KJs+,ATo+,KJo+")}>MP3</a></li>
                                            <li><a onClick={() => handleClick("22+,A6s+,K9s+,Q9s+,J9s+,T8s+,98s,87s,76s,65s,A9o+,K9o+,Q9o+,J9o+,T9o,98o,87o")}>CO</a></li>
                                            <li><a onClick={() => handleClick("22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>BTN</a></li>
                                            <li><a onClick={() => handleClick("22+,A6s+,K9s+,Q9s+,J8s+,T9s,98s,87s,76s,ATo+,K9o+,Q9o+,J9o+")}>SB</a></li>
                                        </ul>
                                    </details>
                                </li>

                                <li>
                                    <details>
                                        <summary>Open Raise - Tight</summary>
                                        <ul>
                                            <li><a onClick={() => handleClick("66+,AJs+,AQo+")}>MP2</a></li>
                                            <li><a onClick={() => handleClick("66+,AJs+,KQs,AJo+,KQo")}>MP3</a></li>
                                            <li><a onClick={() => handleClick("22+,A7s+,K9s+,Q9s+,J9s+,T8s+,A9o+,K9o+,QTo+,J9o+")}>CO</a></li>
                                            <li><a onClick={() => handleClick("22+,A2s+,K8s+,Q8s+,J7s+,T9s,98s,87s,76s,65s,54s,A9o+,K9o+,Q9o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>BTN</a></li>
                                            <li><a onClick={() => handleClick("22+,ATs+,KTs+,QTs+,JTs,ATo+,KTo+,QTo+,JTo")}>SB</a></li>
                                        </ul>
                                    </details>
                                </li>
                                <li>
                                    <details>
                                        <summary>Open Raise - Loose</summary>
                                        <ul>
                                            <li><a onClick={() => handleClick("22+,ATs+,KJs+,QJs,JTs,T9s,98s,87s,76s,65s,ATo+,KJo+")}>MP2</a></li>
                                            <li><a onClick={() => handleClick("22+,A9s+,KJs+,QTs+,JTs,T9s,98s,87s,76s,65s,A9o+,KJo+,QJo")}>MP3</a></li>
                                            <li><a onClick={() => handleClick("22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>CO</a></li>
                                            <li><a onClick={() => handleClick("22+,A2s+,K2s+,Q2s+,J5s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K2o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>BTN</a></li>
                                            <li><a onClick={() => handleClick("22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>SB</a></li>
                                        </ul>
                                    </details>
                                </li>
                            </ul>
                        </summary>
                    </li>
                </ul>
            </div>
        </div>
    )
}

function textToRange(text: string): number[] {

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