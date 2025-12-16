
function add(a, b) {
  return a + b;
}

function mul(a, b) {
  return a * b;
}

Sapphillon.Package =  {
  // Information from package.toml
  meta: {
    name: "math-plugin",
    version: "1.0.0",
    description: "",
    package_id: "com.example"
  },
  // Schema information generated from JSDoc
  functions: {
    add: {       // Function name
      handler: add,         // Actual function reference
      permissions: [{type: "FileSystemRead", resource: "/etc"}],
      description: "Adds two numbers.", // JSDoc summary
      parameters: [         // Parsed result of @param
        { name: "a", type: "number", description: "The number to be added to" },
        { name: "b", type: "number", description: "The number to add" }
      ],
      returns: [ // Parsed result of @returns
        { type: "number", description: "The sum" }
      ]
    },
    mul: {       // Function name
      handler: mul,         // Actual function reference
      permissions: [{type: "FileSystemRead", resource: "/etc"}],
      description: "Multiplies two numbers.", // JSDoc summary
      parameters: [         // Parsed result of @param
        { name: "a", type: "number", description: "The first factor" },
        { name: "b", type: "number", description: "The second factor" }
      ],
      returns: [ // Parsed result of @returns
        { type: "number", description: "The product" }
      ]
    }
  }
};