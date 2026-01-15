
globalThis.Sapphillon = {
    Package: {
        meta: {
            name: "filePlugin",
            version: "1.0.0",
            description: "A file plugin for permission testing",
            author_id: "com.sapphillon.test",
            package_id: "com.sapphillon.test.filePlugin"
        },
        functions: {
            read_file: {
                description: "Reads a file (requires FilesystemRead permission)",
                permissions: [
                    {
                        type: "FilesystemRead",
                        resource: "/tmp/test.txt"
                    }
                ],
                parameters: [
                    { idx: 0, name: "path", type: "string", description: "File path" }
                ],
                returns: [{
                    idx: 0,
                    type: "string",
                    description: "File content"
                }],
                handler: (path) => {
                    console.log(`[JS] Reading file: ${path}`);
                    return Deno.readTextFileSync(path);
                }
            },
            simple_function: {
                description: "A simple function without permission requirements",
                permissions: [],
                parameters: [
                    { idx: 0, name: "text", type: "string", description: "Text to echo" }
                ],
                returns: [{
                    idx: 0,
                    type: "string",
                    description: "Echoed text"
                }],
                handler: (text) => {
                    console.log(`[JS] Echo: ${text}`);
                    return `Echo: ${text}`;
                }
            }
        }
    }
};
