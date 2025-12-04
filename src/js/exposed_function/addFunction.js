(name, binding) => {
    if (globalThis[name]) {
        return;
    }

    Object.defineProperty(globalThis, name, {
        value: function(...args) {
            const thisFunction = globalThis[name];
            thisFunction.args ??= new Map();
            thisFunction.callbacks ??= new Map();

            const seq = (thisFunction.lastSeq ?? 0) + 1;
            thisFunction.lastSeq = seq;
            thisFunction.args.set(seq, args);

            globalThis[binding](
                JSON.stringify({
                    name: name,
                    seq: seq,
                })
            );

            return new Promise(function(resolveFunc, rejectFunc) {
                thisFunction.callbacks.set(seq, {
                    resolve: function(value) {
                        thisFunction.args.delete(seq);
                        resolveFunc(value);
                    },
                    reject: function(error) {
                        thisFunction.args.delete(seq);
                        rejectFunc(error);
                    }
                });
            });
        },
        enumerable: false,
    });
}