import { useState } from "react";
import PreviewTree from "./PreviewTree";

export default function RunSolver() {
    
    return (
        <div className="flex flex-col items-center w-full">
            <div className="flex flex-col items-start w-[700px]">
                <h1 className="text-3xl font-bold">Action Tree</h1>
                <div className="divider"></div>
                <PreviewTree />
            </div>
        </div>
    )
}