import { useAppSelector } from "../store/store"

export default function MiniRange({ oop }: { oop: boolean }) {

    let weights = useAppSelector(state => oop ? state.config.rangeOOP : state.config.rangeIP);
    const config = useAppSelector(state => state.config);

    const getWeight = (i: number, j: number) => {
        return oop ? config.rangeOOP[i * 13 + j] : config.rangeIP[i * 13 + j];
    }

    const isRangeEmpty = () => {
        for (let i = 0; i < 169; i++) {
            if (weights[i] > 0) {
                return false;
            }
        }
        return true;
    }

    const cells = () => {
        const cells: JSX.Element[] = [];
        
        for (let i = 0; i < 13; i++) {
            const row: JSX.Element[] = [];  
            
            for (let j = 0; j < 13; j++) {

                const idx = j * 13 + j;
                const weight = weights[idx];
                
                row.push(
                    <td className="relative w-4 px-0 py-0 border-[0.5px] border-primary-content" key={idx}>
                        <div className="absolute w-full h-full left-0 top-0 bg-primary">
                            <div className="absolute w-full h-full left-0 top-0 bg-bottom" style={{
                                backgroundImage: 
                                    `linear-gradient(to top, #a4cbb4, 
                                    #a4cbb4 ${getWeight(i, j)}%, 
                                    transparent ${getWeight(i, j)}%)`
                            }}></div>
                        </div>
                    </td>
                )  
            }
            cells.push(<tr key={"row" + i} className="h-4 px-0 py-0">{row}</tr>)
        }
        return cells;
    }

    return (
        <div className="flex flex-col items-start">
            <div className="flex flex-row items-center justify-between w-full">
                <h2 className="font-bold">{oop ? "OOP" : "IP"} Range</h2>
                {isRangeEmpty() ? <span className="badge badge-warning badge-md">Empty</span> : null}
            </div>
            <table className="table-fixed">
                <tbody>
                    {cells()}
                </tbody>
            </table>
        </div>
    )
}