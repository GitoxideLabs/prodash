# Repository Development Rules for Prodash

## AI Agent Communication

- AI agents communicating through a person's account must identify themselves, for example in issue or PR descriptions and comments.
  Clearly distinguish agent-authored statements from the person's own speech.
- Agents creating a PR must state the underlying motivation in its description, such as the project that will benefit
  or where the bug was first discovered. Any motivation is valid; explain why the work was pursued rather than
  restating what the PR does. "Want to fix a bug" is insufficient because that is already evident from the PR itself.
- AI assistance that does not replace the person as the speaker, such as proofreading or wording polish, does not require identification.
- Attributing AI assistance in commit metadata, for example with an `Assisted-by:` or `Co-authored-by:` trailer, is welcome but not required.
