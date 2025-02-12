 (...$chromiumoxideEvaluatorArguments$) => {
    const $chromiumoxideEvaluatorContext$ = {
        resultSchema: $chromiumoxideEvaluatorArguments$.pop(),
        awaitPromise: $chromiumoxideEvaluatorArguments$.pop(),
        exprThis: $chromiumoxideEvaluatorArguments$.pop(),
        funcThis: $chromiumoxideEvaluatorArguments$.pop(),
        descriptors: $chromiumoxideEvaluatorArguments$.pop(),
        specials: $chromiumoxideEvaluatorArguments$,
    };

    $chromiumoxideEvaluatorContext$.func = (
        function(){return (__EXPR_FUNC__);}
    ).call($chromiumoxideEvaluatorContext$.exprThis);

    $chromiumoxideEvaluatorContext$.exprs = (
        function(){return [__EXPR_LIST__];}
    ).call($chromiumoxideEvaluatorContext$.exprThis);

    const mergeSpecials = (() => {
        function mergeSpecials(descriptors, specials, expressions) {
            let offset = 0;
            for (let i = 0; i < descriptors.length; i++) {
                let descriptor = descriptors[i];
                for (let j = 0; j < descriptor.paths.length; j++) {
                    descriptor.value = mergeSpecial(
                        descriptor.value,
                        descriptor.paths[j],
                        specials[offset + j],
                        expressions
                    );
                    offset += 1;
                }
            }
            return descriptors.map(descriptor => descriptor.value);
        };

        function mergeSpecial(value, path, special, expressions) {
            if (path.length === 0) {
                if (
                    typeof special === 'object' ||
                    typeof special === 'function' ||
                    typeof special === 'symbol' ||
                    typeof special === 'bigint'
                ) {
                    return special;
                } else if (typeof special === 'number') {
                    return expressions[special];
                }
                throw new Error('Unsupported special value: ' + special);
            }

            const segment = path[0];
            if (typeof segment === 'string' && typeof value !== 'object') {
                value = {};
            } else if (typeof segment === 'number' && !isArray(value)) {
                value = [];
            }
            value[segment] = mergeSpecial(value[segment], path.slice(1), special, expressions);
            return value;
        };

        return mergeSpecials;
    })();

    const splitSpecials = (() => {
        // get the remote type of `schema`
        //
        // returns:
        // - the remote type of `schema`
        // - null if `schema` is not a remote object
        //
        const getSchemaSpecialType = (schema) => {
            const JS_REMOTE_KEY = '$chromiumoxide::js::remote';
            const JS_BIGINT_KEY = '$chromiumoxide::js::bigint';
            const JS_UNDEFINED_KEY = '$chromiumoxide::js::undefined';

            if (schema.properties && schema.properties.hasOwnProperty(JS_REMOTE_KEY)) {
                let default_allowlist = ["object", "function", "symbol"];
                let allowlist = schema.properties[JS_REMOTE_KEY].properties.type.enum;
                if (!allowlist) {
                    allowlist = default_allowlist;
                }
                return allowlist;
            }
            if (schema.properties && schema.properties.hasOwnProperty(JS_BIGINT_KEY)) {
                return ['bigint'];
            }
            if (schema.properties && schema.properties.hasOwnProperty(JS_UNDEFINED_KEY)) {
                return ['undefined'];
            }
            return null;
        }

        // get the remote type of `value`
        //
        // returns:
        // - the remote type of `value`
        // - null if `value` is not a remote object
        //
        const getObjectSpecialType = (value) => {
            if (typeof value === 'function') {
                return 'function';
            }
            if (typeof value === 'symbol') {
                return 'symbol';
            }
            if (typeof value === 'object' && value !== null) {
                return 'object';
            }
            if (typeof value === 'bigint') {
                return 'bigint';
            }
            if (typeof value === 'undefined') {
                return 'undefined';
            }
            return null;
        }

        // cross-realm compatible way to check if `value` is an array
        const isArray = (value) => {
            return typeof value === 'object' && value !== null &&
                   typeof value.length === 'number' &&
                   typeof value.splice === 'function' &&
                   typeof value.slice === 'function' &&
                   value.constructor.name === 'Array';
        }

        // validate the `value` using `schema` and collect all `RemoteObject`
        // specified by `schema` from `value`
        //
        // arguments:
        // - `value`: the source value
        // - `schema`: the schema
        // - `currentPath`: the current path
        //
        // returns:
        // {
        //     // the error information when validation failed
        //     error?: {
        //         // the value causing the error
        //         value?: any,
        //
        //         // the path where validation failed
        //         path: (string|number)[],
        //
        //         // the error message when validation failed
        //         message: string,
        //     },
        //
        //     // the collected specials and their paths
        //     specials?: ({
        //         // the path of the special value
        //         path: (string|number)[],
        //
        //         // the special value
        //         value: any,
        //     })[],
        // }
        //
        function validateSchemaAndCollectSpecials(
            value, schema, currentPath
        ) {
            if (typeof schema !== 'boolean' && typeof schema !== 'object') {
                return {
                    error: {
                        value: value,
                        path: currentPath,
                        message: 'invalid schema: not a boolean or object'
                    }
                };
            }

            if (typeof schema === 'boolean') {
                if (schema) {
                    return {
                        specials: []  // Always return empty array for successful validation
                    };
                } else {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'schema is `false`'
                        }
                    };
                }
            }

            let oneOfSchemas = schema.oneOf || [];
            let anyOfSchemas = schema.anyOf || [];
            let allOfSchemas = schema.allOf || [];
            
            if (Array.isArray(schema.type) && schema.type.length === 1) {
                schema.type = schema.type[0];
            }

            let collectedSpecials = [];
            if (typeof schema.type === 'string') {
                let result = validateSimpleSchema(value, schema, currentPath);
                if (result.error) {
                    return result;
                }
                if (result.specials) {
                    collectedSpecials.push(...result.specials);
                }
            } else {
                let schemaList = [];
                if (!Array.isArray(schema.type)) {
                    schema.type = ['object', 'array', 'string', 'number', 'integer', 'boolean', 'null'];
                }
                for (const schemaType of schema.type) {
                    if (typeof schemaType === 'string') {
                        let newSchema = structuredClone(schema);
                        newSchema.type = schemaType;
                        newSchema.oneOf = undefined;
                        newSchema.anyOf = undefined;
                        newSchema.allOf = undefined;
                        schemaList.push(newSchema);
                    } else {
                        return {
                            error: {
                                path: currentPath,
                                message: `invalid schema: unknown type ${schemaType}`
                            }
                        };
                    }
                }

                const result = validateSchemaList('any', value, schemaList, currentPath);
                if (result.error) {
                    return result;
                }
                if (result.specials) {
                    collectedSpecials.push(...result.specials);
                }
            }

            const oneOfResult = validateSchemaList('one', value, oneOfSchemas, currentPath);
            if (oneOfResult.error) {
                return oneOfResult;
            }
            if (oneOfResult.specials) {
                collectedSpecials.push(...oneOfResult.specials);
            }

            const anyOfResult = validateSchemaList('any', value, anyOfSchemas, currentPath);
            if (anyOfResult.error) {
                return anyOfResult;
            }
            if (anyOfResult.specials) {
                collectedSpecials.push(...anyOfResult.specials);
            }

            const allOfResult = validateSchemaList('all', value, allOfSchemas, currentPath);
            if (allOfResult.error) {
                return allOfResult;
            }
            if (allOfResult.specials) {
                collectedSpecials.push(...allOfResult.specials);
            }

            return {
                specials: collectedSpecials
            };
        }

        // validate the `value` using `schema` and collect all `RemoteObject`
        //
        // schema is a simple schema, which has `type` with no list of schemas,
        // and discards `anyOf`, `oneOf`, `allOf`.
        function validateSimpleSchema(value, schema, currentPath) {
            let collectedSpecials = [];
            if (schema.type === 'object') {
                const allowedSpecialTypes = getSchemaSpecialType(schema);
                if (allowedSpecialTypes !== null) {
                    // validate for remote object
                    const objectSpecialType = getObjectSpecialType(value);
                    if (allowedSpecialTypes.includes(objectSpecialType)) {
                        collectedSpecials.push({
                            path: currentPath,
                            value: value
                        });
                    } else {
                        return {
                            error: {
                                value: value,
                                path: currentPath,
                                message: `not an allowed special type: ${objectSpecialType} (allowed: ${allowedSpecialTypes.join(', ')})`
                            }
                        };
                    }
                } else {
                    // validate for normal object
                    if (typeof value !== 'object' || value === null || isArray(value)) {
                        return {
                            error: {
                                value: value,
                                path: currentPath,
                                message: 'not an object'
                            }
                        };
                    }

                    let processedProperties = new Set();
                    let requiredProperties = new Set(schema.required || []);

                    if (typeof schema.properties === 'object') {
                        for (const [key, subSchema] of Object.entries(schema.properties)) {
                            if (!requiredProperties.has(key)) {
                                if (typeof subSchema.type === 'string') {
                                    subSchema.type = [subSchema.type];
                                }
                                subSchema.type.push('null');
                            }

                            const result = validateSchemaAndCollectSpecials(
                                value[key],
                                subSchema,
                                currentPath.concat([key])
                            );
                            if (result.error) {
                                return result;
                            }
                            if (result.specials) {
                                collectedSpecials.push(...result.specials);
                            }
                            processedProperties.add(key);
                        }
                    }
                    if (schema.additionalProperties !== undefined) {
                        for (const [key, val] of Object.entries(value)) {
                            if (!processedProperties.has(key)) {
                                const result = validateSchemaAndCollectSpecials(
                                    val,
                                    schema.additionalProperties,
                                    currentPath.concat([key])
                                );
                                if (result.error) {
                                    return result;
                                }
                                if (result.specials) {
                                    collectedSpecials.push(...result.specials);
                                }
                            }
                        }
                    }
                }
            } else if (schema.type === 'array') {
                if (!isArray(value)) {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not an array'
                        }
                    };
                }

                let itemsIndex = 0;
                if (Array.isArray(schema.prefixItems)) {
                    for (let i = 0; i < schema.prefixItems.length; i++) {
                        const result = validateSchemaAndCollectSpecials(
                            value[i],
                            schema.prefixItems[i],
                            currentPath.concat([i])
                        );
                        if (result.error) {
                            return result;
                        }
                        if (result.specials) {
                            collectedSpecials.push(...result.specials);
                        }
                    }
                    itemsIndex = schema.prefixItems.length;
                }

                if (typeof schema.items !== 'undefined') {
                    for (let i = itemsIndex; i < value.length; i++) {
                        const result = validateSchemaAndCollectSpecials(
                            value[i],
                            schema.items,
                            currentPath.concat([i])
                        );
                        if (result.error) {
                            return result;
                        }
                        if (result.specials) {
                            collectedSpecials.push(...result.specials);
                        }
                    }
                }
            } else if (schema.type === 'string') {
                if (typeof value !== 'string' && !(value instanceof String)) {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not a string'
                        }
                    };
                }
            } else if (schema.type === 'number') {
                if (typeof value !== 'number') {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not a number'
                        }
                    };
                }
            } else if (schema.type === 'integer') {
                if (!Number.isInteger(value)) {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not an integer'
                        }
                    };
                }
            } else if (schema.type === 'boolean') {
                if (typeof value !== 'boolean') {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not a boolean'
                        }
                    };
                }
            } else if (schema.type === 'null') {
                if (value !== null && value !== undefined) {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'not a null'
                        }
                    };
                }
            } else {
                return {
                    error: {
                        path: currentPath,
                        message: `invalid schema: unknown type ${schema.type}`
                    }
                };
            }

            return {
                specials: collectedSpecials
            };
        }

        // `kind` is one of the following:
        // - 'any'
        // - 'all'
        // - 'one'
        function validateSchemaList(kind, value, schemaList, currentPath) {
            if (schemaList.length === 0) {
                return { 
                    specials: []
                };
            }

            if (kind === 'any') {
                let firstErr = null;
                // at least one of the schemas should match
                for (const subSchema of schemaList) {
                    const result = validateSchemaAndCollectSpecials(
                        value, 
                        subSchema, 
                        currentPath
                    );
                    if (!result.error) {
                        return result;
                    } else if (firstErr === null) {
                        firstErr = result;
                    }
                }
                return firstErr || {
                    error: {
                        value: value,
                        path: currentPath,
                        message: 'value does not match any schema'
                    }
                };
            }

            let oks = [];
            let firstErr = null;
            for (let i = 0; i < schemaList.length; i++) {
                const result = validateSchemaAndCollectSpecials(
                    value,
                    schemaList[i],
                    currentPath.concat([i])
                );
                if (!result.error) {
                    oks.push(result);
                } else if (firstErr === null) {
                    firstErr = result;
                }
            }

            if (kind === 'all') {
                // all of the schemas should match
                if (firstErr !== null) {
                    return firstErr;
                } else {
                    let allSpecials = [];
                    for (const ok of oks) {
                        allSpecials.push(...ok.specials);
                    }
                    return {
                        specials: allSpecials
                    };
                }
            } else {
                // only one of the schemas should match
                if (oks.length === 1) {
                    return oks[0];
                } else if (oks.length > 1) {
                    return {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'value matches more than one schema'
                        }
                    };
                } else {
                    return firstErr || {
                        error: {
                            value: value,
                            path: currentPath,
                            message: 'value matches no schema'
                        }
                    };
                }
            }
        }

        function sortAndUniqueSpecials(specials) {
            if (specials.length === 0) {
                return specials;
            }

            function compareSpecials(a, b) {
                if (a.path.length !== b.path.length) {
                    return a.path.length - b.path.length;
                }
                for (let i = 0; i < a.path.length; i++) {
                    if (a.path[i] !== b.path[i]) {
                        if (typeof a.path[i] === 'number' && typeof b.path[i] === 'number') {
                            return a.path[i] - b.path[i];
                        } else {
                            return String(a.path[i]).localeCompare(String(b.path[i]));
                        }
                    }
                }
                return 0;
            }
            function isParentPath(parent, child) {
                if (parent.length >= child.length) {
                    return false;
                }
                for (let i = 0; i < parent.length; i++) {
                    if (parent[i] !== child[i]) {
                        return false;
                    }
                }
                return true;
            }

            specials.sort(compareSpecials);

            const uniqueSpecials = [specials[0]];
            for (let i = 1; i < specials.length; i++) {
                const last = specials[i - 1];
                const current = specials[i];
                if (compareSpecials(current, last) !== 0) {
                    if (isParentPath(last.path, current.path)) {
                        continue;
                    }
                    uniqueSpecials.push(current);
                }
            }
            return uniqueSpecials;
        }

        function pathString(path) {
            return path.map(p => typeof p === 'number' ? `[${p}]` : `['${p}']`).join('');
        }

        return (function (value) {
            let schema = this;
            let result = validateSchemaAndCollectSpecials(value, schema, []);
            if (result.error) {
                let { path, value, message } = result.error;
                let pathStr = pathString(path);
                throw new Error(
                    `schema validation failed ${pathStr}: ${message}`,
                    { cause: { value } }
                );
            }

            let paths = [];
            let specials = [];

            if (result.specials.length !== 0) {
                result.specials = sortAndUniqueSpecials(result.specials);

                paths = result.specials.map(s => s.path);
                specials = result.specials.map(s => s.value);

                // replace the container of the special value with a new container
                // for replacing the special value with `{}`
                const cloneValue = (value) => {
                    if (typeof value === 'object' && value !== null) {
                        if (isArray(value)) {
                            return Array.from(value);
                        } else {
                            return Object.assign({}, value);
                        }
                    }
                    throw new Error('encountered non-object container type for special value', { cause: value });
                };

                const replaceSpecialValues = () => {
                    const replacedPaths = new Set();
                    if (paths[0].length === 0) {
                        // the root value is a special value
                        return {};
                    }
                    const replacedValue = cloneValue(value);
                    const replaceByPath = (path) => {
                        let prefixPath = [];
                        let parentValue = replacedValue;
                        for (let i = 0; i < path.length - 1; i++) {
                            let pathSegment = path[i];
                            prefixPath.push(pathSegment);
                            let prefixPathStr = pathString(prefixPath);
                            if (!replacedPaths.has(prefixPathStr)) {
                                replacedPaths.add(prefixPathStr);
                                parentValue[pathSegment] = cloneValue(parentValue[pathSegment]);
                            }
                            parentValue = parentValue[pathSegment];
                        }
                        let lastPathSegment = path[path.length - 1];
                        parentValue[lastPathSegment] = {};
                    }
                    for (let i = 0; i < paths.length; i++) {
                        replaceByPath(paths[i]);
                    }
                    return replacedValue;
                };

                value = replaceSpecialValues(value);
            }

            return {
                descriptor: { value, paths },
                specials,
            };
        });
    })().bind($chromiumoxideEvaluatorContext$.resultSchema);

    const funcArgs = mergeSpecials(
        $chromiumoxideEvaluatorContext$.descriptors,
        $chromiumoxideEvaluatorContext$.specials,
        $chromiumoxideEvaluatorContext$.exprs
    );

    const result = $chromiumoxideEvaluatorContext$.func.call(
        $chromiumoxideEvaluatorContext$.funcThis,
        ...funcArgs
    );

    if ($chromiumoxideEvaluatorContext$.awaitPromise) {
        return Promise.resolve(result)
            .then(splitSpecials);
    } else {
        return splitSpecials(result);
    }
}
