import { readFile, writeFile } from "node:fs/promises";

import semanticRelease from "semantic-release";

const expectedVersion = process.argv[2];
const notesFile = process.env.RELEASE_NOTES_FILE;

if (!expectedVersion || !notesFile) {
  throw new Error("Expected version and RELEASE_NOTES_FILE are required");
}

const result = await semanticRelease({
  dryRun: true,
  plugins: [
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "conventionalcommits",
      },
    ],
    [
      "@semantic-release/release-notes-generator",
      {
        preset: "conventionalcommits",
      },
    ],
  ],
});

if (!result?.nextRelease) {
  throw new Error("semantic-release found no version to prepare");
}

if (result.nextRelease.version !== expectedVersion) {
  throw new Error(
    `Expected version ${expectedVersion}, but semantic-release selected ${result.nextRelease.version}`,
  );
}

const notes = result.nextRelease.notes ?? "";
const changelogTitle = "# Changelog";
const changelog = await readFile("CHANGELOG.md", "utf8");

if (!changelog.startsWith(changelogTitle)) {
  throw new Error("CHANGELOG.md does not start with the expected title");
}

const currentContent = changelog.slice(changelogTitle.length).replace(/^\s*/, "");

await writeFile(
  "CHANGELOG.md",
  `${changelogTitle}\n\n${notes.trim()}\n\n${currentContent}`,
);

await writeFile(notesFile, notes);
