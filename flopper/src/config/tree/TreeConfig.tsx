import "./TreeConfig.css"
import { TreeConfigValues } from "../Config"
import BetSizes from "./BetSizes"

export default function TreeConfig(
    { treeConfig, setTreeConfig }: 
    { treeConfig: TreeConfigValues, setTreeConfig: (treeConfig: TreeConfigValues) => void }
) {

    return (
        <div id="tree-config">
            <h1>Tree Config</h1>
            <BaseConfig treeConfig={treeConfig} setTreeConfig={setTreeConfig}/>
            <h1>Bet Sizes</h1>
            <BetSizes />
        </div>
    )
}

function BaseConfig(
    { treeConfig, setTreeConfig }:
    { treeConfig: TreeConfigValues, setTreeConfig: (treeConfig: TreeConfigValues) => void }
) {

    const handleChangeStartingPot = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            startingPot: parseInt(e.target.value)
        })
    }
    
    const handleChangeEffectiveStack = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            effectiveStack: parseInt(e.target.value)
        })
    }

    const handleChangeRake = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            rake: Number(e.target.value)
        })
    }

    const handleChangeRakeCap = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            rakeCap: parseInt(e.target.value)
        })
    }

    return (
        <div id="base-config">
            <p className="config-label">Starting Pot Size:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.startingPot}
                onChange={handleChangeStartingPot}
            />

            <p className="config-label">Rake %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                max="100"
                step="1"
                value={treeConfig.rake}
                onChange={handleChangeRake}
            />

            <p className="config-label">Effective Stack:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.effectiveStack}
                onChange={handleChangeEffectiveStack}
            />

            <p className="config-label">Rake Cap:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.rakeCap}
                onChange={handleChangeRakeCap}
            />
        </div>
    )
}