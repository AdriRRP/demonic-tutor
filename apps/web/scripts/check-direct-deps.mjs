import { execFileSync } from "node:child_process";

function outdatedOutput() {
  try {
    return execFileSync("npm", ["outdated", "--json", "--depth=0"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
  } catch (error) {
    if (error instanceof Error && "status" in error && error.status === 1) {
      const stdout = "stdout" in error ? error.stdout : "";
      return typeof stdout === "string" ? stdout : "";
    }

    throw error;
  }
}

const raw = outdatedOutput().trim();
const outdated = raw === "" ? {} : JSON.parse(raw);

const entries = Object.entries(outdated).sort(([left], [right]) => left.localeCompare(right));

const blocking = entries.filter(([, info]) => info.current !== info.wanted);
const majorsAvailable = entries.filter(
  ([, info]) => info.current === info.wanted && info.current !== info.latest,
);

if (blocking.length > 0) {
  console.error("Found direct web dependencies that are behind their declared range:");

  for (const [name, info] of blocking) {
    console.error(`- ${name}: current=${info.current} wanted=${info.wanted} latest=${info.latest}`);
  }

  process.exit(1);
}

if (majorsAvailable.length === 0) {
  console.log("All direct web dependencies satisfy the pinned manifest.");
  process.exit(0);
}

console.log("Direct web dependencies satisfy the pinned manifest.");
console.log("Newer releases are available and should be handled by Dependabot:");

for (const [name, info] of majorsAvailable) {
  console.log(`- ${name}: pinned=${info.current} latest=${info.latest}`);
}
