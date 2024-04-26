Due to limitations of cargo workspaces, there is no single workspace here.
Instead, platforms get their own workspace(s) to make rust-analyzer work.
Do not otherwise rely on workspace features like patches or workspace wide deps, they will NOT WORK.