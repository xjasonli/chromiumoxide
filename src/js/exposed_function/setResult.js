(name, seq, value, errmsg) => {
    const thisFunction = globalThis[name];
    const callback = thisFunction.callbacks.get(seq);
    if (errmsg !== undefined && errmsg !== null) {
        let error = new Error(errmsg);
        callback.reject(error);
    } else {
        callback.resolve(value);
    }
}