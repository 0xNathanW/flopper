import { invoke } from "@tauri-apps/api"
import { ChanceReport, Results } from "./typing";

export const osName = async (): Promise<'windows' | 'macos' | 'linux'> => {
    return await invoke("os_name");
};

export const memory = async (): Promise<number[]> => {
    return await invoke("memory");
};

export const setNumThreads = async (numThreads: number) => {
    return await invoke("set_num_threads", { num: numThreads });
};

// Action Tree
export const buildActionTree = async (
    board: number[],
    startingPot: number,
    effectiveStack: number,
    rake: number,
    rakeCap: number,
    addAllInThreshold: number,
    forceAllInThreshold: number,

    oopBetsFlop: string,
    oopRaisesFlop: string,
    oopBetsTurn: string,
    oopRaisesTurn: string,
    oopBetsRiver: string,
    oopRaisesRiver: string,

    ipBetsFlop: string,
    ipRaisesFlop: string,
    ipBetsTurn: string,
    ipRaisesTurn: string,
    ipBetsRiver: string,
    ipRaisesRiver: string,
): Promise<boolean> => {
    return await invoke("build_action_tree", {
        board,
        startingPot,
        effectiveStack,
        rake,
        rakeCap,
        addAllInThreshold,
        forceAllInThreshold,

        oopBetsFlop,
        oopRaisesFlop,
        oopBetsTurn,
        oopRaisesTurn,
        oopBetsRiver,
        oopRaisesRiver,

        ipBetsFlop,
        ipRaisesFlop,
        ipBetsTurn,
        ipRaisesTurn,
        ipBetsRiver,
        ipRaisesRiver,
    });
};

export const numNodesActionTree = async (): Promise<number> => {
    return await invoke("num_nodes_action_tree");
};

export const toRootActionTree = async () => {
    return await invoke("to_root_action_tree");
};

export const getActionsActionTree = async (): Promise<string[]> => {
    return await invoke("get_actions_action_tree");
};

export const playActionTree = async (action: string): Promise<number> => {
    return await invoke("play_action_tree", { action });
};

export const totalBetAmountActionTree = async (): Promise<[number, number]> => {
    return await invoke("total_bet_amount_action_tree");
};

export const isTerminalNodeActionTree = async (): Promise<boolean> => {
    return await invoke("is_terminal_node_action_tree");
};

export const isChanceNodeActionTree = async (): Promise<boolean> => {
    return await invoke("is_chance_node_action_tree");
};

export const applyHistoryActionTree = async (history: string[]) => {
    return await invoke("apply_history_action_tree", { history });
};

// Game Tree
export const buidGameTree = async (
    board: number[],
    rangeOop: number[],
    rangeIp: number[],
    startingPot: number,
    effectiveStack: number,
    rake: number,
    rakeCap: number,
    addAllInThreshold: number,
    forceAllInThreshold: number,

    oopBetsFlop: string,
    oopRaisesFlop: string,
    oopBetsTurn: string,
    oopRaisesTurn: string,
    oopBetsRiver: string,
    oopRaisesRiver: string,

    ipBetsFlop: string,
    ipRaisesFlop: string,
    ipBetsTurn: string,
    ipRaisesTurn: string,
    ipBetsRiver: string,
    ipRaisesRiver: string,
): Promise<string | null> => {
    return await invoke("build_game_tree", {
        board,
        rangeOop,
        rangeIp,
        startingPot,
        effectiveStack,
        rake,
        rakeCap,
        addAllInThreshold,
        forceAllInThreshold,

        oopBetsFlop,
        oopRaisesFlop,
        oopBetsTurn,
        oopRaisesTurn,
        oopBetsRiver,
        oopRaisesRiver,

        ipBetsFlop,
        ipRaisesFlop,
        ipBetsTurn,
        ipRaisesTurn,
        ipBetsRiver,
        ipRaisesRiver,
})};

export const solveStepGame = async (i: number) => {
    return await invoke("solve_step_game", { i });
};

export const handsGame = async (): Promise<number[][]> => {
    return await invoke("hands_game");
};

export const finaliseGame = async () => {
    return await invoke("finalise_game");
};

export const exploitabilityGame = async (): Promise<number> => {
    return await invoke("exploitability_game");
};

export const memoryUsageGame = async (): Promise<number[]> => {
    return await invoke("memory_usage_game");
};

export const allocateMemoryGame = async (compression: boolean) => {
    return await invoke("allocate_memory_game", { compression });
};

export const applyHistoryGame = async (history: number[]) => {
    return await invoke("apply_history_game", { history });
};

export const totalBetAmountGame = async (append: number[]): Promise<number[]> => {
    return await invoke("total_bet_amount_game", { append });
};

export const possibleCardsGame = async (): Promise<bigint> => {
    return BigInt(await invoke("possible_cards_game"));
};

type ResultsGame = {
    current_player: "oop" | "ip" | "chance" | "terminal";
    num_actions: number;
    empty: number;
    eqr_base: number[];
    weights: number[][];
    normaliser: number[][];
    equity: number[][];
    ev: number[][];
    eqr: number[][];
    strategy: number[];
    action_ev: number[];
};

export const resultsGame = async (): Promise<Results> => {
    const r: ResultsGame = await invoke("results_game");
    return {
        currentPlayer: r.current_player,
        numActions: r.num_actions,
        empty: r.empty,
        eqrBase: r.eqr_base,
        weights: r.weights,
        normaliser: r.normaliser,
        equity: r.equity,
        ev: r.ev,
        eqr: r.eqr,
        strategy: r.strategy,
        actionEv: r.action_ev,
    };
};

export const actionsAfterGame = async (append: number[]): Promise<string[]> => {
    return await invoke("actions_after_game", { append });
};

type ChanceReportGame = {
    num_actions: number;
    status: number[];
    combos: number[][];
    equity: number[][];
    ev: number[][];
    eqr: number[][];
    strategy: number[];
};

export const chanceReportGame = async (
    append: number[],
    currentPlayer: "oop" | "ip" | "terminal",
    numActions: number,
): Promise<ChanceReport> => {
    
    const report: ChanceReportGame = await invoke("chance_report_game", { append, numActions });
    return {
        currentPlayer,
        numActions,
        status: report.status,
        combos: report.combos,
        equity: report.equity,
        ev: report.ev,
        eqr: report.eqr,
        strategy: report.strategy,
    };
};