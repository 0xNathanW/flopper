
// Action Tree

import { invoke } from "@tauri-apps/api"

export const newActionTree = async (
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

export const numNodes = async (): Promise<number> => {
    return await invoke("num_nodes");
};

export const toRoot = async () => {
    return await invoke("to_root");
};

export const getActions = async (): Promise<string[]> => {
    return await invoke("get_actions");
};

export const play = async (action: string): Promise<number> => {
    return await invoke("play", { action });
};

export const totalBetAmount = async (): Promise<[number, number]> => {
    return await invoke("total_bet_amount");
};

export const isTerminalNode = async (): Promise<boolean> => {
    return await invoke("is_terminal_node");
};

export const isChanceNode = async (): Promise<boolean> => {
    return await invoke("is_chance_node");
};

export const applyHistory = async (history: string[]) => {
    return await invoke("apply_history", { history });
};


