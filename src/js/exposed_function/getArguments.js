(name, seq) => {
    const thisFunction = globalThis[name];
    const args = thisFunction.args.get(seq);
    return args;
}