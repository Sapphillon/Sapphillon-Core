
globalThis.Sapphillon = {
    Package: {
        meta: {
            name: "error-plugin",
            version: "1.0.0",
            description: "A plugin that always fails for testing",
            author_id: "com.sapphillon.test",
            package_id: "com.sapphillon.test.error-plugin"
        },
        functions: {
            throw_immediate: {
                description: "Throws an error immediately",
                permissions: [],
                parameters: [],
                returns: [],
                handler: () => {
                    throw new Error("This is an immediate error");
                }
            },
            throw_async: {
                description: "Throws an error nicely from an async function",
                permissions: [],
                parameters: [],
                returns: [],
                handler: async () => {
                    await new Promise(resolve => setTimeout(resolve, 10));
                    throw new Error("This is an async error");
                }
            },
            async_success: {
                description: "An async function that succeeds",
                permissions: [],
                parameters: [
                    { idx: 0, name: "value", type: "string", description: "A value to transform" }
                ],
                returns: [{ idx: 0, type: "string", description: "Transformed value" }],
                handler: async (value) => {
                    await new Promise(resolve => setTimeout(resolve, 10));
                    return `async: ${value}`;
                }
            },
            return_null: {
                description: "A function that returns null",
                permissions: [],
                parameters: [],
                returns: [],
                handler: () => {
                    return null;
                }
            },
            return_undefined: {
                description: "A function that returns undefined",
                permissions: [],
                parameters: [],
                returns: [],
                handler: () => {
                    return undefined;
                }
            },
            no_op: {
                description: "An empty function that does nothing",
                permissions: [],
                parameters: [],
                returns: [],
                handler: () => {
                    // Intentionally empty
                }
            }
        }
    }
};
