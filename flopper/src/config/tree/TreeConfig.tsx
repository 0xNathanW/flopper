import "./TreeConfig.css"
import { useAppDispatch, useAppSelector } from "../../store/store";
import {
    setStartingPot,
    setEffectiveStack,
    setRake,
    setRakeCap,
    setAddAllIn,
    setForceAllIn
} from "../../store/features/configSlice";

export default function TreeConfig() {

    return (
        <div id="tree-config">
            <h1>Tree Config</h1>
            <div id="tree-config-row">
                <BaseConfig />
            </div>
        </div>
    )
}

function BaseConfig() {

    const dispatch = useAppDispatch();
    const treeConfig = useAppSelector(state => state.config);

    return (
        <div id="base-config">
            <p className="config-label">Starting Pot Size:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.startingPot}
                onChange={(e) => dispatch(setStartingPot(Number(e.target.value)))}
            />
            
            <p className="config-label">Effective Stack:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.effectiveStack}
                onChange={(e) => dispatch(setEffectiveStack(Number(e.target.value)))}
            />

            <p className="config-label">Rake %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                max="100"
                step="1"
                value={treeConfig.rake}
                onChange={(e) => dispatch(setRake(Number(e.target.value)))}
            />

            <p className="config-label">Rake Cap:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.rakeCap}
                onChange={(e) => dispatch(setRakeCap(Number(e.target.value)))}
            />

            <p className="config-label">Add All-In Threshold %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.addAllIn}
                onChange={(e) => dispatch(setAddAllIn(Number(e.target.value)))}
            />

            <p className="config-label">Force All-In Threshold %:</p>
            <input
                className="config-input"
                type="number"
                min="0"
                value={treeConfig.forceAllIn}
                onChange={(e) => dispatch(setForceAllIn(Number(e.target.value)))}
            />
            
        </div>
    )
}
