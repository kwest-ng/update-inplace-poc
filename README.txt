# Windows in-place Proof-of-Concept

Minimal conditions:

- reset.exe exists: used to restore pre-update conditions.
- new-version.exe exists: used to restore replace.exe.new
- replace.exe.bak exists: used to restore replace.exe

How to run:

Run `reset` to restore pre-update conditions
Run `replace` to perform the update
Run `replace` again to perform the post-update cleanup
Run `replace` a third time to see no-cleanup is necessary
Any further `replace` runs will repeat the third.
Run `reset` again to restore pre-update conditions.