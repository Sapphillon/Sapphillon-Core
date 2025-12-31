// Simple test to verify Deno runtime works with fetch and console.log

async function main() {
    console.log("Starting fetch test...");

    try {
        const response = await fetch("https://httpbin.org/get");
        const data = await response.json();
        console.log("Fetch result:", JSON.stringify(data, null, 2));
    } catch (error) {
        console.error("Fetch error:", error);
    }

    console.log("Test completed!");
}

main();
