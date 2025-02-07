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
            } else if (typeof segment === 'number' && !Array.isArray(value)) {
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
                return schema.properties[JS_REMOTE_KEY].type;
            }
            if (schema.properties && schema.properties.hasOwnProperty(JS_BIGINT_KEY)) {
                return 'bigint';
            }
            if (schema.properties && schema.properties.hasOwnProperty(JS_UNDEFINED_KEY)) {
                return 'undefined';
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

        // validate the `value` using `schema` and collect all `RemoteObject`
        // specified by `schema` from `value`
        //
        // arguments:
        // - `value`: the source value
        // - `schema`: the schema
        // - `currentPath`: the current path
        // - `outSpecials`: the collected specials
        // - `outPaths`: the paths of the collected specials
        //
        // returns:
        // - boolean: whether the `value` matches the `schema`
        //
        function validateSchemaAndCollectSpecials(
            value, schema, currentPath, outSpecials, outPaths
        ) {
            if (typeof schema !== 'boolean' && typeof schema !== 'object') {
                throw new Error(`schema is not a valid schema`);
            }

            if (typeof schema === 'boolean') {
                return schema;
            }

            let oneOfSchemas = schema.oneOf || [];
            let anyOfSchemas = schema.anyOf || [];
            let allOfSchemas = schema.allOf || [];
            
            if (Array.isArray(schema.type) && schema.type.length === 1) {
                schema.type = schema.type[0];
            }

            let typesSpecials = [];  
            let typesPaths = [];
            if (typeof schema.type === 'string') {
                if (schema.type === 'object') {
                    const schemaSpecialType = getSchemaSpecialType(schema);
                    if (schemaSpecialType !== null) {
                        // validate for remote object
                        const objectSpecialType = getObjectSpecialType(value);
                        if (schemaSpecialType === objectSpecialType) {
                            typesPaths.push(currentPath);
                            typesSpecials.push(value);
                        } else {
                            return false;
                        }
                    } else {
                        // validate for normal object
                        if (typeof value !== 'object') {
                            return false;
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

                                if (!validateSchemaAndCollectSpecials(
                                    value[key],
                                    subSchema,
                                    currentPath.concat([key]),
                                    typesSpecials,
                                    typesPaths
                                )) {
                                    return false;
                                }
                                processedProperties.add(key);
                            }
                        }
                        if (schema.additionalProperties !== undefined) {
                            for (const [key, val] of Object.entries(value)) {
                                if (!processedProperties.has(key)) {
                                    if (
                                        !(typeof schema.additionalProperties === 'boolean') &&
                                        !(typeof schema.additionalProperties === 'object')
                                    ) {
                                        return false;
                                    }

                                    if (!validateSchemaAndCollectSpecials(
                                        val,
                                        schema.additionalProperties,
                                        currentPath.concat([key]),
                                        typesSpecials,
                                        typesPaths
                                    )) {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                } else if (schema.type === 'array') {
                    if (!Array.isArray(value)) {
                        return false;
                    }
                    let index = 0;
                    if (typeof schema.minItems === 'number') {
                        if (value.length < schema.minItems) {
                            return false;
                        }
                    }
                    if (typeof schema.maxItems === 'number') {
                        if (value.length > schema.maxItems) {
                            return false;
                        }
                    }
                    if (Array.isArray(schema.prefixItems)) {
                        for (let i = 0; i < schema.prefixItems.length; i++) {
                            if (!validateSchemaAndCollectSpecials(
                                value[i],
                                schema.prefixItems[i],
                                currentPath.concat([i]),
                                typesSpecials,
                                typesPaths
                            )) {
                                return false;
                            }
                        }
                        index = schema.prefixItems.length;
                    }
                    if (typeof schema.items !== 'undefined') {
                        for (let i = index; i < value.length; i++) {
                            if (
                                !(typeof schema.items === 'boolean') &&
                                !(typeof schema.items === 'object')
                            ) {
                                return false;
                            }

                            if (!validateSchemaAndCollectSpecials(
                                value[i],
                                schema.items,
                                currentPath.concat([i]),
                                typesSpecials,
                                typesPaths
                            )) {
                                return false;
                            }
                        }
                    }
                } else if (schema.type === 'string') {
                    if (typeof value !== 'string' && !(value instanceof String)) {
                        return false;
                    }
                } else if (schema.type === 'number') {
                    if (typeof value !== 'number') {
                        return false;
                    }
                } else if (schema.type === 'integer') {
                    if (!Number.isInteger(value)) {
                        return false;
                    }
                } else if (schema.type === 'boolean') {
                    if (typeof value !== 'boolean') {
                        return false;
                    }
                } else if (schema.type === 'null') {
                    if (value !== null && value !== undefined) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                let typesSchemas = [];
                if (!Array.isArray(schema.type)) {
                    schema.type = ['object', 'array', 'string', 'number', 'integer', 'boolean', 'null'];
                }
                for (const type of schema.type) {
                    if (typeof type === 'string') {
                        let newSchema = structuredClone(schema);
                        newSchema.type = type;
                        typesSchemas.push(newSchema);
                    } else {
                        throw new Error(`unknown type ${type}`);
                    }
                }

                if (!validateSchemaList('any', value, typesSchemas, currentPath, typesSpecials, typesPaths)) {
                    return false;
                }
            }

            let oneOfSpecials = [];
            let oneOfPaths = [];
            if (!validateSchemaList('one', value, oneOfSchemas, currentPath, oneOfSpecials, oneOfPaths)) {
                return false;
            }

            let anyOfSpecials = [];
            let anyOfPaths = [];
            if (!validateSchemaList('any', value, anyOfSchemas, currentPath, anyOfSpecials, anyOfPaths)) {
                return false;
            }

            let allOfSpecials = [];
            let allOfPaths = [];
            if (!validateSchemaList('all', value, allOfSchemas, currentPath, allOfSpecials, allOfPaths)) {
                return false;
            }

            outSpecials.push(...typesSpecials);
            outPaths.push(...typesPaths);
            outSpecials.push(...oneOfSpecials);
            outPaths.push(...oneOfPaths);
            outSpecials.push(...anyOfSpecials);
            outPaths.push(...anyOfPaths);
            outSpecials.push(...allOfSpecials);
            outPaths.push(...allOfPaths);
            return true;
        }

        // `kind` is one of the following:
        // - 'any'
        // - 'all'
        // - 'one'
        function validateSchemaList(kind, value, schemaList, currentPath, outSpecials, outPaths) {
            if (schemaList.length === 0) {
                return true;
            }

            if (kind === 'any') {
                // at least one of the schemas should match
                for (const subSchema of schemaList) {
                    let tmpSpecials = [];
                    let tmpPaths = [];
                    if (validateSchemaAndCollectSpecials(value, subSchema, currentPath, tmpSpecials, tmpPaths)) {
                        outSpecials.push(...tmpSpecials);
                        outPaths.push(...tmpPaths);
                        return true;
                    }
                }
                return false;
            }

            let results = [];
            for (let i = 0; i < schemaList.length; i++) {
                let tmpSpecials = [];
                let tmpPaths = [];
                if (validateSchemaAndCollectSpecials(value, schemaList[i], currentPath.concat([i]), tmpSpecials, tmpPaths)) {
                    results.push({
                        matches: true,
                        specials: tmpSpecials,
                        paths: tmpPaths,
                    });
                } else {
                    results.push({
                        matches: false,
                    });
                }
            }

            if (kind === 'all') {
                // all of the schemas should match
                if (results.every(result => result.matches)) {
                    outSpecials.push(...results.map(result => result.specials).flat());
                    outPaths.push(...results.map(result => result.paths).flat());
                    return true;
                }
                return false;
            } else {
                // only one of the schemas should match
                let index = null;
                for (let i = 0; i < results.length; i++) {
                    if (results[i].matches && index === null) {
                        index = i;
                    } else {
                        return false;
                    }
                }
                outSpecials.push(...results[index].specials);
                outPaths.push(...results[index].paths);
                return true;
            }
        }

        return (function (value) {
            let schema = this;
            let specials = [];
            let paths = [];

            let matches = validateSchemaAndCollectSpecials(
                value,
                schema,
                [],
                specials,
                paths,
            );
            if (!matches) {
                throw new Error(`schema validation failed`);
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
