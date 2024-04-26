# Early alloc
The kernel global allocator starts pre-initialized with a 32KiB span of memory, enough for initial bootstrap (i.e. allocating the memory map table)