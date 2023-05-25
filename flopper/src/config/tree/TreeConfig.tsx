import "./TreeConfig.css"
import { TreeConfigValues } from "../Config"

export default function TreeConfig(
    { treeConfig, setTreeConfig }: 
    { treeConfig: TreeConfigValues, setTreeConfig: (treeConfig: TreeConfigValues) => void }
) {

    return (
        <div id="tree-config">
            <h1>Tree Config</h1>
            <div id="tree-config-row">
                <BaseConfig treeConfig={treeConfig} setTreeConfig={setTreeConfig}/>
            </div>
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

    const handleChangeAddAllIn = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            addAllIn: parseInt(e.target.value)
        })
    }

    const handleChangeForceAllIn = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTreeConfig({
            ...treeConfig,
            forceAllIn: parseInt(e.target.value)
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

            <p className="config-label">Add All-In Threshold %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.addAllIn}
                onChange={handleChangeAddAllIn}
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

            <p className="config-label">Force All-In Threshold %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.forceAllIn}
                onChange={handleChangeForceAllIn}
            />
            
        </div>
    )
}
