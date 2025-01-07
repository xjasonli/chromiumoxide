
export function executeFunction(func: Function | string, thisArg: any, args: any[]) {
    if (typeof func === "string") {
        return executeFunctionByName(func, thisArg, args);
    }
    return executeFunctionImpl(func, thisArg, args);
}

function executeFunctionImpl(func: Function, thisArg: any, args: any[]) {
    if (thisArg === undefined) {
        return func(...args);
    } else {
        return func.apply(thisArg, args);
    }
}

function executeFunctionByName(name: string, thisArg: any, args: any[]) {
    let namespaces = name.split(".");
    let funcName = namespaces.pop();
    if (funcName === undefined) {
        throw new Error("Function name is required");
    }
    let context: any = globalThis;
    for (let i = 0; i < namespaces.length; i++) {
        context = context[namespaces[i]];
        if (context === undefined) {
            throw new Error(`Namespace ${namespaces[i]} not found`);
        }
        context = context as any;
    }
    let func = context[funcName];
    if (typeof func !== "function") {
        throw new Error(`Function ${funcName} is not a function`);
    }
    return executeFunctionImpl(func, thisArg, args);
}

interface ArgumentDescriptor {
    index: number;
    kind: {
        
    }
}