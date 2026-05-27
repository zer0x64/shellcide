use crate::app::{ShellcideApp, TargetArch};
use eframe::egui;

pub(crate) struct InstructionInfo {
    pub(crate) name: &'static str,
    pub(crate) syntax_intel: &'static str,
    pub(crate) syntax_att: &'static str,
    pub(crate) description: &'static str,
    pub(crate) template_intel: &'static str,
    pub(crate) template_att: &'static str,
    pub(crate) shellcode_tip: &'static str,
}

impl InstructionInfo {
    pub(crate) const fn new(
        name: &'static str,
        syntax_intel: &'static str,
        description: &'static str,
        template_intel: &'static str,
        shellcode_tip: &'static str,
    ) -> Self {
        Self {
            name,
            syntax_intel,
            syntax_att: "",
            description,
            template_intel,
            template_att: "",
            shellcode_tip,
        }
    }

    pub(crate) const fn new_att(
        name: &'static str,
        syntax_intel: &'static str,
        syntax_att: &'static str,
        description: &'static str,
        template_intel: &'static str,
        template_att: &'static str,
        shellcode_tip: &'static str,
    ) -> Self {
        Self {
            name,
            syntax_intel,
            syntax_att,
            description,
            template_intel,
            template_att,
            shellcode_tip,
        }
    }

    pub(crate) fn syntax(&self, att: bool) -> &'static str {
        if att && !self.syntax_att.is_empty() {
            self.syntax_att
        } else {
            self.syntax_intel
        }
    }

    pub(crate) fn template(&self, att: bool) -> &'static str {
        if att && !self.template_att.is_empty() {
            self.template_att
        } else {
            self.template_intel
        }
    }
}

static X86_64_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new_att(
        "mov",
        "mov dest, src",
        "movq src, dest",
        "Move data between registers or memory",
        "mov rax, rdi\n",
        "movq %rdi, %rax\n",
        "Avoid large immediate moves which introduce null bytes. Use sub-register movs (like mov al, imm) or register math instead.",
    ),
    InstructionInfo::new_att(
        "movzx",
        "movzx dest, src",
        "movzbq src, dest",
        "Move with zero-extend",
        "movzx rax, byte ptr [rsi]\n",
        "movzbq (%rsi), %rax\n",
        "Excellent for reading bytes or words into a 64-bit register while automatically zero-extending, which avoids leaving dirty values in the destination register without generating null bytes.",
    ),
    InstructionInfo::new_att(
        "movsx",
        "movsx dest, src",
        "movsbq src, dest",
        "Move with sign-extend",
        "movsx rax, byte ptr [rsi]\n",
        "movsbq (%rsi), %rax\n",
        "Moves a byte or word and sign-extends it. Useful when loading signed offsets or values in loop counters while avoiding manual sign extension instructions.",
    ),
    InstructionInfo::new_att(
        "movsxd",
        "movsxd dest, src",
        "movsxd src, dest",
        "Move doubleword to quadword with sign-extend",
        "movsxd rax, ecx\n",
        "movsxd %ecx, %rax\n",
        "Sign-extends a 32-bit register/memory to a 64-bit register. Can be used in address calculations in 64-bit mode where offsets might be negative.",
    ),
    InstructionInfo::new_att(
        "xor",
        "xor dest, src",
        "xorq src, dest",
        "Exclusive OR (often used to zero a register)",
        "xor rax, rax\n",
        "xorq %rax, %rax\n",
        "Ideal for clearing registers without producing null bytes. Note that 'xor eax, eax' also clears the upper 32 bits of rax.",
    ),
    InstructionInfo::new_att(
        "xadd",
        "xadd dest, src",
        "xaddq src, dest",
        "Exchange and add",
        "xadd [rdi], rax\n",
        "xaddq %rax, (%rdi)\n",
        "Atomic exchange and add. Can be used in multi-threaded shellcode or to fetch and increment an index in a single instruction without locking.",
    ),
    InstructionInfo::new_att(
        "cmpxchg",
        "cmpxchg dest, src",
        "cmpxchgq src, dest",
        "Compare and exchange",
        "cmpxchg [rdi], rsi\n",
        "cmpxchgq %rsi, (%rdi)\n",
        "Compares AL/AX/EAX/RAX with dest; if equal, src is loaded into dest. Useful for building mutexes or spinlocks in complex shellcode payloads.",
    ),
    InstructionInfo::new_att(
        "push",
        "push val",
        "pushq val",
        "Push value onto stack",
        "push rax\n",
        "pushq %rax\n",
        "Pushing a small immediate and popping it (push 0x3b; pop rax) is shorter and avoids nulls compared to a 64-bit mov.",
    ),
    InstructionInfo::new_att(
        "pop",
        "pop dest",
        "popq dest",
        "Pop value from stack",
        "pop rax\n",
        "popq %rax\n",
        "Retrieve values or adjust stack pointer. Can also be used to fetch the current Instruction Pointer (RIP) via a call-pop sequence.",
    ),
    InstructionInfo::new(
        "jmp",
        "jmp label",
        "Unconditional jump to label",
        "jmp label\n",
        "Relative short jumps (jmp short) are position-independent and use only 2 bytes. Avoid absolute/far jumps which hardcode addresses.",
    ),
    InstructionInfo::new(
        "call",
        "call func",
        "Call a procedure",
        "call function\n",
        "Pushes return address onto stack. In shellcode, call-pop is commonly used to load the address of strings or payload data into registers.",
    ),
    InstructionInfo::new(
        "ret",
        "ret",
        "Return from procedure",
        "ret\n",
        "Returns to address on top of stack. Essential for ROP (Return-Oriented Programming) chains and cleaner control flow termination.",
    ),
    InstructionInfo::new(
        "syscall",
        "syscall",
        "Fast system call to kernel",
        "syscall\n",
        "Primary method to transition to kernel mode. Ensure RAX holds the syscall number and arguments are loaded into RDI, RSI, RDX, R10, R8, R9.",
    ),
    InstructionInfo::new_att(
        "inc",
        "inc dest",
        "incq dest",
        "Increment operand by 1",
        "inc rax\n",
        "incq %rax\n",
        "Saves bytes when adjusting loop counters or setting syscall numbers (e.g. starting from zero and incrementing to avoid nulls).",
    ),
    InstructionInfo::new_att(
        "dec",
        "dec dest",
        "decq dest",
        "Decrement operand by 1",
        "dec rax\n",
        "decq %rax\n",
        "Perfect for decrementing loop counters in shellcode decoders. Changes flags like Zero Flag (ZF) for conditional jumps.",
    ),
    InstructionInfo::new_att(
        "neg",
        "neg dest",
        "negq dest",
        "Two's complement negation",
        "neg rax\n",
        "negq %rax\n",
        "Useful for bypass techniques where you load a negative value (e.g., -5) and negate it to get a positive value, avoiding null bytes.",
    ),
    InstructionInfo::new_att(
        "not",
        "not dest",
        "notq dest",
        "One's complement negation (bitwise NOT)",
        "not rax\n",
        "notq %rax\n",
        "Bitwise negation. Often used to construct target values (like 0xffffffffffffffff) or to obscure payloads to bypass AV signatures.",
    ),
    InstructionInfo::new_att(
        "add",
        "add dest, src",
        "addq src, dest",
        "Add operands",
        "add rax, rbx\n",
        "addq %rbx, %rax\n",
        "Used for general arithmetic and stack pointer adjustments. Watch out for large immediate operands that contain null bytes.",
    ),
    InstructionInfo::new_att(
        "sub",
        "sub dest, src",
        "subq src, dest",
        "Subtract operands",
        "sub rax, rbx\n",
        "subq %rbx, %rax\n",
        "Can be used to allocate stack space dynamically or clear registers by subtracting them from themselves without null bytes.",
    ),
    InstructionInfo::new_att(
        "shl",
        "shl dest, count",
        "shlq count, dest",
        "Shift left (multiply by 2^count)",
        "shl rax, 4\n",
        "shlq $4, %rax\n",
        "Shifts bits to the left. Useful for reconstructing memory addresses, encoding values, or multiplying registers quickly.",
    ),
    InstructionInfo::new_att(
        "shr",
        "shr dest, count",
        "shrq count, dest",
        "Shift right (unsigned division by 2^count)",
        "shr rax, 4\n",
        "shrq $4, %rax\n",
        "Unsigned shift right. Often used to clear high bits or unpack bytes into individual variables in decoders.",
    ),
    InstructionInfo::new_att(
        "sar",
        "sar dest, count",
        "sarq count, dest",
        "Shift arithmetic right (signed division by 2^count)",
        "sar rax, 4\n",
        "sarq $4, %rax\n",
        "Signed shift right. Preserves the sign bit, which is useful when dealing with negative offsets or signed integers in shellcode logic.",
    ),
    InstructionInfo::new_att(
        "rol",
        "rol dest, count",
        "rolq count, dest",
        "Rotate left",
        "rol rax, 4\n",
        "rolq $4, %rax\n",
        "Rotates bits to the left. Excellent for custom encryption/decryption routines (obfuscation) to bypass antivirus detections.",
    ),
    InstructionInfo::new_att(
        "ror",
        "ror dest, count",
        "rorq count, dest",
        "Rotate right",
        "ror rax, 4\n",
        "rorq $4, %rax\n",
        "Rotates bits to the right. Useful for hashing algorithms (like ROR13 API hashing) to resolve functions dynamically in Windows shellcode.",
    ),
    InstructionInfo::new_att(
        "lea",
        "lea dest, src",
        "leaq src, dest",
        "Load effective address",
        "lea rax, [rip + label]\n",
        "leaq label(%rip), %rax\n",
        "Extremely useful for Position-Independent Code (PIC) to resolve string and data addresses relative to the current Instruction Pointer.",
    ),
    InstructionInfo::new_att(
        "cmp",
        "cmp op1, op2",
        "cmpq op2, op1",
        "Compare operands and set flags",
        "cmp rax, rbx\n",
        "cmpq %rbx, %rax\n",
        "Sets CPU flags for conditional branching. To avoid null bytes, compare with registers instead of comparing with zero directly.",
    ),
    InstructionInfo::new_att(
        "test",
        "test op1, op2",
        "testq op2, op1",
        "Logical compare (AND) and set flags",
        "test rax, rax\n",
        "testq %rax, %rax\n",
        "Tests if a register is zero. 'test rax, rax' is faster and consumes fewer bytes than 'cmp rax, 0', and it generates no null bytes.",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "A one-byte instruction (0x90). Used to pad code blocks or build NOP sleds to increase exploit reliability under instruction pointer jitter.",
    ),
    InstructionInfo::new(
        "cdq",
        "cdq",
        "Sign-extend EAX into EDX:EAX",
        "cdq\n",
        "Sign-extends EAX to EDX:EAX. If EAX is positive (e.g. syscall number or success return), this zeroes EDX in just 1 byte (0x99).",
    ),
    InstructionInfo::new(
        "cqo",
        "cqo",
        "Sign-extend RAX into RDX:RAX",
        "cqo\n",
        "Sign-extends RAX to RDX:RAX. If RAX is positive, this zeroes RDX in 2 bytes (REX.W + 0x99), avoiding a longer mov or xor.",
    ),
    InstructionInfo::new_att(
        "xchg",
        "xchg dest, src",
        "xchgq src, dest",
        "Exchange register values",
        "xchg rax, rdi\n",
        "xchgq %rdi, %rax\n",
        "Swaps two register values. Can save space by avoiding temporary registers or when copying RAX to another register in 1 or 2 bytes.",
    ),
    InstructionInfo::new(
        "lodsb",
        "lodsb",
        "Load byte at [RSI] into AL and update RSI",
        "lodsb\n",
        "Loads a byte from [RSI] into AL and automatically increments/decrements RSI. Essential 1-byte instruction (0xac) for decoder loops.",
    ),
    InstructionInfo::new(
        "stosb",
        "stosb",
        "Store byte in AL at [RDI] and update RDI",
        "stosb\n",
        "Stores AL to [RDI] and increments/decrements RDI. Highly efficient 1-byte instruction (0xaa) for writing decoded shellcode bytes.",
    ),
    InstructionInfo::new(
        "rep movsb",
        "rep movsb",
        "Repeat Move String Byte",
        "rep movsb\n",
        "Copies RCX bytes from [RSI] to [RDI]. Extremely powerful for bulk payload relocation or memory copying in 2 bytes (0xf3 0xa4).",
    ),
    InstructionInfo::new(
        "rep stosb",
        "rep stosb",
        "Repeat Store String Byte",
        "rep stosb\n",
        "Fills RCX bytes of memory at [RDI] with AL. Often used in shellcode to zero out buffers or generate repeated byte sequences (0xf3 0xaa).",
    ),
    InstructionInfo::new(
        "repne scasb",
        "repne scasb",
        "Repeat String Scan Byte while not equal",
        "repne scasb\n",
        "Scans memory at [RDI] for byte AL. Extremely useful for finding null-terminators to calculate string length dynamically.",
    ),
    InstructionInfo::new(
        "cld",
        "cld",
        "Clear direction flag",
        "cld\n",
        "Sets direction flag to 0 (incrementing strings). Essential before string operations like lodsb/stosb/movsb to ensure predictable direction.",
    ),
    InstructionInfo::new(
        "std",
        "std",
        "Set direction flag",
        "std\n",
        "Sets direction flag to 1 (decrementing strings). Occasionally used in reverse-decoders that decode payloads from high to low memory.",
    ),
    InstructionInfo::new(
        "leave",
        "leave",
        "High-level procedure exit",
        "leave\n",
        "Releases stack frame: moves RBP to RSP and pops RBP. Shorter than manual rsp/rbp restoration, saving precious bytes (1 byte: 0xc9).",
    ),
    InstructionInfo::new_att(
        "enter",
        "enter alloc, nesting",
        "enter alloc, nesting",
        "High-level procedure entry",
        "enter 16, 0\n",
        "enter $16, $0\n",
        "Creates a stack frame. Although usually avoided in shellcode due to size and potential null bytes in immediate arguments, it is good to know.",
    ),
    InstructionInfo::new(
        "int3",
        "int3",
        "Breakpoint interrupt",
        "int3\n",
        "Triggers a breakpoint trap (0xcc). Very useful for debugging shellcode, finding relative execution offsets, or pausing execution.",
    ),
    InstructionInfo::new(
        "ud2",
        "ud2",
        "Undefined instruction",
        "ud2\n",
        "Generates an invalid opcode exception. Used in shellcode to force immediate termination or crash detection, or to confuse disassemblers.",
    ),
    InstructionInfo::new(
        "cpuid",
        "cpuid",
        "CPU identification",
        "cpuid\n",
        "Returns processor info in EAX, EBX, ECX, EDX. Commonly used for anti-sandbox detection, checking virtualization flags, or profiling CPU support.",
    ),
    InstructionInfo::new(
        "rdtsc",
        "rdtsc",
        "Read time-stamp counter",
        "rdtsc\n",
        "Loads TSC into EDX:EAX. Crucial for anti-debugging (detecting step delays), anti-sandbox timing checks, or seeding PRNGs.",
    ),
    InstructionInfo::new(
        "rdtscp",
        "rdtscp",
        "Read time-stamp counter and processor ID",
        "rdtscp\n",
        "Serialized version of RDTSC. Additionally loads the logical processor ID into ECX. Used to prevent CPU reordering in timing checks.",
    ),
    InstructionInfo::new_att(
        "rdrand",
        "rdrand dest",
        "rdrand dest",
        "Read random number",
        "rdrand rax\n",
        "rdrand %rax\n",
        "Generates a hardware random number into dest. Excellent for generating encryption keys or initialization vectors on the fly without system calls.",
    ),
    InstructionInfo::new(
        "lahf",
        "lahf",
        "Load status flags into AH register",
        "lahf\n",
        "Copies the low byte of the EFLAGS register into AH. Useful for saving flags without pushing onto the stack or for custom flag-manipulation loops.",
    ),
    InstructionInfo::new(
        "sahf",
        "sahf",
        "Store AH register into status flags",
        "sahf\n",
        "Loads AH into low byte of EFLAGS. Useful for restoring flags or custom flag adjustments during code decoding / evasion.",
    ),
    InstructionInfo::new_att(
        "bt",
        "bt dest, bit",
        "btq bit, dest",
        "Bit test",
        "bt rax, 3\n",
        "btq $3, %rax\n",
        "Tests a single bit at the specified index and copies it to the Carry Flag (CF). Great for compact bitmask checks.",
    ),
    InstructionInfo::new_att(
        "bts",
        "bts dest, bit",
        "btsq bit, dest",
        "Bit test and set",
        "bts rax, 3\n",
        "btsq $3, %rax\n",
        "Tests a bit, saves it to CF, and sets that bit in the destination. Useful for constructing bitmasks or setting permissions flags dynamically.",
    ),
    InstructionInfo::new_att(
        "btr",
        "btr dest, bit",
        "btrq bit, dest",
        "Bit test and reset",
        "btr rax, 3\n",
        "btrq $3, %rax\n",
        "Tests a bit, saves it to CF, and clears (zeroes) that bit in destination. Saves space when masking out specific status flags.",
    ),
    InstructionInfo::new_att(
        "btc",
        "btc dest, bit",
        "btcq bit, dest",
        "Bit test and complement",
        "btc rax, 3\n",
        "btcq $3, %rax\n",
        "Tests a bit, saves it to CF, and toggles (complements) that bit in destination. Perfect for bitwise encryption loops.",
    ),
    InstructionInfo::new_att(
        "bsf",
        "bsf dest, src",
        "bsfq src, dest",
        "Bit scan forward",
        "bsf rax, rbx\n",
        "bsfq %rbx, %rax\n",
        "Scans src for the least significant set bit (LSB) and stores index in dest. Helpful in custom allocators or encoding loops.",
    ),
    InstructionInfo::new_att(
        "bsr",
        "bsr dest, src",
        "bsrq src, dest",
        "Bit scan reverse",
        "bsr rax, rbx\n",
        "bsrq %rbx, %rax\n",
        "Scans src for the most significant set bit (MSB) and stores index in dest. Used to quickly locate the highest set bit in integer math.",
    ),
    InstructionInfo::new_att(
        "popcnt",
        "popcnt dest, src",
        "popcntq src, dest",
        "Population count (count set bits)",
        "popcnt rax, rbx\n",
        "popcntq %rbx, %rax\n",
        "Counts the number of bits set to 1 in the source. Can be useful in custom parity checks, cryptography, or hashing routines.",
    ),
    InstructionInfo::new_att(
        "lzcnt",
        "lzcnt dest, src",
        "lzcntq src, dest",
        "Leading zero count",
        "lzcnt rax, rbx\n",
        "lzcntq %rbx, %rax\n",
        "Counts the number of leading zero bits in the source register. Useful for alignment calculations or dynamic compression/encoding.",
    ),
    InstructionInfo::new_att(
        "tzcnt",
        "tzcnt dest, src",
        "tzcntq src, dest",
        "Trailing zero count",
        "tzcnt rax, rbx\n",
        "tzcntq %rbx, %rax\n",
        "Counts the number of trailing zero bits in the source. Very similar to BSF, but handles zero sources predictably (setting destination to operand size).",
    ),
    InstructionInfo::new(
        "loop",
        "loop label",
        "Decrement RCX and jump if not zero",
        "loop label\n",
        "Decrements RCX/ECX and jumps to target if non-zero. Useful for short loop decoders, but check the relative offset bytes for nulls.",
    ),
    InstructionInfo::new(
        "fldz",
        "fldz",
        "Push +0.0 onto FPU register stack",
        "fldz\n",
        "Pushes +0.0 onto FPU stack. Part of the FPU getpc technique to trigger FPU state saving without calling.",
    ),
    InstructionInfo::new(
        "fnstenv",
        "fnstenv [addr]",
        "Store FPU environment to memory",
        "fnstenv [rsp - 12]\n",
        "Saves FPU environment to stack. Writes the instruction pointer of the last FPU instruction (fldz), revealing RIP/EIP for position-independent code.",
    ),
];

static X86_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new_att(
        "mov",
        "mov dest, src",
        "movl src, dest",
        "Move data between registers or memory",
        "mov eax, ecx\n",
        "movl %ecx, %eax\n",
        "Avoid moving large immediate values containing null bytes. Use push/pop or sub-register operations to build values.",
    ),
    InstructionInfo::new_att(
        "movzx",
        "movzx dest, src",
        "movzbl src, dest",
        "Move with zero-extend",
        "movzx eax, byte ptr [esi]\n",
        "movzbl (%esi), %eax\n",
        "Zero-extends a byte/word from memory or register to destination. Excellent for avoiding dirty registers or clearing high bits of EAX without nulls.",
    ),
    InstructionInfo::new_att(
        "movsx",
        "movsx dest, src",
        "movsbl src, dest",
        "Move with sign-extend",
        "movsx eax, byte ptr [esi]\n",
        "movsbl (%esi), %eax\n",
        "Sign-extends a byte/word to doubleword. Useful for converting small negative offsets or index variables while avoiding extra sign-extension instructions.",
    ),
    InstructionInfo::new_att(
        "xor",
        "xor dest, src",
        "xorl src, dest",
        "Exclusive OR (often used to zero a register)",
        "xor eax, eax\n",
        "xorl %eax, %eax\n",
        "Safely zeroes out a register without null bytes. Produces a compact 2-byte opcode (e.g. 31 c0).",
    ),
    InstructionInfo::new_att(
        "push",
        "push val",
        "pushl val",
        "Push value onto stack",
        "push eax\n",
        "pushl %eax\n",
        "Pushing a small immediate and popping it (push 0xb; pop eax) is shorter and avoids nulls compared to a mov instruction.",
    ),
    InstructionInfo::new_att(
        "pop",
        "pop dest",
        "popl dest",
        "Pop value from stack",
        "pop eax\n",
        "popl %eax\n",
        "Used to retrieve arguments or stack addresses. Combined with call, pop is used to obtain the current EIP.",
    ),
    InstructionInfo::new(
        "jmp",
        "jmp label",
        "Unconditional jump to label",
        "jmp label\n",
        "Relative short jumps (jmp short) are position-independent, use only 2 bytes, and avoid hardcoding target addresses.",
    ),
    InstructionInfo::new(
        "call",
        "call func",
        "Call a procedure",
        "call function\n",
        "Pushes the return address onto the stack. Used in the classic Call-Pop technique to find the address of payload strings.",
    ),
    InstructionInfo::new(
        "ret",
        "ret",
        "Return from procedure",
        "ret\n",
        "Pops the return address from the stack and jumps to it. Used in functions or ROP chains for control flow redirects.",
    ),
    InstructionInfo::new_att(
        "int 0x80",
        "int 0x80",
        "int $0x80",
        "System call via software interrupt",
        "int 0x80\n",
        "int $0x80\n",
        "The standard 32-bit Linux system call mechanism. EAX contains the syscall number, and arguments are passed in EBX, ECX, EDX, ESI, EDI.",
    ),
    InstructionInfo::new(
        "sysenter",
        "sysenter",
        "Fast transition to system kernel",
        "sysenter\n",
        "An alternative fast system call mechanism. Mostly used on Windows or older x86 systems. Setup requires setting ESP/EIP registers in MSRs.",
    ),
    InstructionInfo::new_att(
        "inc",
        "inc dest",
        "incl dest",
        "Increment operand by 1",
        "inc eax\n",
        "incl %eax\n",
        "In 32-bit x86, single-byte inc instructions (0x40-0x47) exist, making them highly space-efficient for loops and offsets.",
    ),
    InstructionInfo::new_att(
        "dec",
        "dec dest",
        "decl dest",
        "Decrement operand by 1",
        "dec eax\n",
        "decl %eax\n",
        "In 32-bit x86, single-byte dec instructions (0x48-0x4F) exist, saving space in loop counters or conditional checks.",
    ),
    InstructionInfo::new_att(
        "neg",
        "neg dest",
        "negl dest",
        "Two's complement negation",
        "neg eax\n",
        "negl %eax\n",
        "Negates the destination. Great for evasion where a negative value is loaded and then negated to bypass bad characters/null byte restrictions.",
    ),
    InstructionInfo::new_att(
        "not",
        "not dest",
        "notl dest",
        "One's complement negation (bitwise NOT)",
        "not eax\n",
        "notl %eax\n",
        "Toggles all bits of the register. Often used to create masks (like 0xffffffff) or to obscure opcodes to evade intrusion detection systems (IDS).",
    ),
    InstructionInfo::new_att(
        "add",
        "add dest, src",
        "addl src, dest",
        "Add operands",
        "add eax, ebx\n",
        "addl %ebx, %eax\n",
        "Used for math and pointer adjustments. Ensure immediate values added do not introduce bad characters or null bytes.",
    ),
    InstructionInfo::new_att(
        "sub",
        "sub dest, src",
        "subl src, dest",
        "Subtract operands",
        "sub eax, ebx\n",
        "subl %ebx, %eax\n",
        "Useful for adjusting the stack pointer (ESP) or register values. Can also be used to clear registers without nulls.",
    ),
    InstructionInfo::new_att(
        "shl",
        "shl dest, count",
        "shll count, dest",
        "Shift left (multiply by 2^count)",
        "shl eax, 4\n",
        "shll $4, %eax\n",
        "Shifts bits to the left. Frequently used to multiply register values or reconstruct pointers in multi-byte address math.",
    ),
    InstructionInfo::new_att(
        "shr",
        "shr dest, count",
        "shrl count, dest",
        "Shift right (unsigned division by 2^count)",
        "shr eax, 4\n",
        "shrl $4, %eax\n",
        "Performs unsigned shift right. Useful for dividing values by powers of two or parsing individual bytes out of a dword.",
    ),
    InstructionInfo::new_att(
        "sar",
        "sar dest, count",
        "sarl count, dest",
        "Shift arithmetic right (signed division by 2^count)",
        "sar eax, 4\n",
        "sarl $4, %eax\n",
        "Arithmetic shift right. Preserves the sign bit, making it useful when adjusting signed offsets in decoders.",
    ),
    InstructionInfo::new_att(
        "rol",
        "rol dest, count",
        "roll count, dest",
        "Rotate left",
        "rol eax, 4\n",
        "roll $4, %eax\n",
        "Rotates bits left. Highly effective for simple encoding/decoding loops designed to mask shellcode payload signatures from AV.",
    ),
    InstructionInfo::new_att(
        "ror",
        "ror dest, count",
        "rorl count, dest",
        "Rotate right",
        "ror eax, 4\n",
        "rorl $4, %eax\n",
        "Rotates bits right. Widely used in Windows API hashing (like ROR13) to dynamically locate external DLL functions in shellcode.",
    ),
    InstructionInfo::new_att(
        "lea",
        "lea dest, src",
        "leal src, dest",
        "Load effective address",
        "lea eax, [ebx + 4]\n",
        "leal 4(%ebx), %eax\n",
        "Allows address calculations and simple arithmetic without dereferencing memory. Very useful in position-independent code.",
    ),
    InstructionInfo::new_att(
        "cmp",
        "cmp op1, op2",
        "cmpl op2, op1",
        "Compare operands and set flags",
        "cmp eax, ebx\n",
        "cmpl %ebx, %eax\n",
        "Sets CPU flags for branching. To avoid null bytes, compare with registers or small non-zero immediate values.",
    ),
    InstructionInfo::new_att(
        "test",
        "test op1, op2",
        "testl op2, op1",
        "Logical compare (AND) and set flags",
        "test eax, eax\n",
        "testl %eax, %eax\n",
        "Checks if a register is zero. Smaller and faster than cmp to zero, and does not generate null bytes (85 c0).",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "A one-byte instruction (0x90). Essential for creating NOP sleds to slide execution into the shellcode target.",
    ),
    InstructionInfo::new(
        "cdq",
        "cdq",
        "Sign-extend EAX into EDX:EAX",
        "cdq\n",
        "Sign-extends EAX to EDX:EAX. If EAX is positive (e.g. system call number), EDX is zeroed in 1 byte (0x99), avoiding null bytes.",
    ),
    InstructionInfo::new(
        "cltd",
        "cltd",
        "Sign-extend EAX into EDX:EAX (AT&T)",
        "cltd\n",
        "AT&T name for CDQ. Sign-extends %eax into %edx:%eax. Very efficient way to clear %edx (1 byte: 0x99).",
    ),
    InstructionInfo::new_att(
        "xchg",
        "xchg dest, src",
        "xchgl src, dest",
        "Exchange register values",
        "xchg eax, ebx\n",
        "xchgl %ebx, %eax\n",
        "In 32-bit x86, exchanging any register with EAX is a single-byte instruction (0x91-0x97). Extremely space-efficient.",
    ),
    InstructionInfo::new(
        "lodsb",
        "lodsb",
        "Load byte at [ESI] into AL and update ESI",
        "lodsb\n",
        "Loads a byte from [ESI] into AL and increments/decrements ESI. 1-byte opcode (0xac), ideal for string/decoder loops.",
    ),
    InstructionInfo::new(
        "stosb",
        "stosb",
        "Store byte in AL at [EDI] and update EDI",
        "stosb\n",
        "Stores AL to [EDI] and increments/decrements EDI. 1-byte opcode (0xaa), ideal for decoder loops writing to memory.",
    ),
    InstructionInfo::new(
        "rep movsb",
        "rep movsb",
        "Repeat Move String Byte",
        "rep movsb\n",
        "Copies ECX bytes from memory at [ESI] to [EDI]. Extremely short instruction sequence for payload copy/relocation.",
    ),
    InstructionInfo::new(
        "rep stosb",
        "rep stosb",
        "Repeat Store String Byte",
        "rep stosb\n",
        "Fills ECX bytes of memory at [EDI] with AL. Commonly used to quickly zero out local variable structures or buffers.",
    ),
    InstructionInfo::new(
        "repne scasb",
        "repne scasb",
        "Repeat String Scan Byte while not equal",
        "repne scasb\n",
        "Scans memory at [EDI] for byte AL. Perfect for calculating string lengths dynamically in memory without hardcoding values.",
    ),
    InstructionInfo::new(
        "cld",
        "cld",
        "Clear direction flag",
        "cld\n",
        "Clears DF (DF=0), ensuring that string ops like lodsb/stosb progress forward. Always good practice to run before decoder loops.",
    ),
    InstructionInfo::new(
        "std",
        "std",
        "Set direction flag",
        "std\n",
        "Sets DF (DF=1). String ops progress backward (decrementing addresses). Useful for reverse decoder loops or overwriting memory backwards.",
    ),
    InstructionInfo::new(
        "leave",
        "leave",
        "High-level procedure exit",
        "leave\n",
        "Equivalent to `mov esp, ebp` followed by `pop ebp`. Extremely compact 1-byte opcode (0xc9) for restoring stack frames.",
    ),
    InstructionInfo::new_att(
        "enter",
        "enter alloc, nesting",
        "enter alloc, nesting",
        "High-level procedure entry",
        "enter 16, 0\n",
        "enter $16, $0\n",
        "Creates a stack frame. Although usually avoided in shellcode due to size and potential null bytes in immediate arguments, it is good to know.",
    ),
    InstructionInfo::new(
        "int3",
        "int3",
        "Breakpoint interrupt",
        "int3\n",
        "Triggers a breakpoint trap (0xcc). Very useful for debugging shellcode, finding relative execution offsets, or pausing execution.",
    ),
    InstructionInfo::new(
        "ud2",
        "ud2",
        "Undefined instruction",
        "ud2\n",
        "Forces an invalid opcode exception. Can be used for anti-disassembly, forcing immediate termination, or triggering custom crash handlers.",
    ),
    InstructionInfo::new(
        "cpuid",
        "cpuid",
        "CPU identification",
        "cpuid\n",
        "Returns processor info in EAX, EBX, ECX, EDX. Commonly used for anti-sandbox detection, checking virtualization flags, or profiling CPU support.",
    ),
    InstructionInfo::new(
        "rdtsc",
        "rdtsc",
        "Read time-stamp counter",
        "rdtsc\n",
        "Loads TSC into EDX:EAX. Crucial for anti-debugging (detecting step delays), anti-sandbox timing checks, or seeding PRNGs.",
    ),
    InstructionInfo::new_att(
        "rdrand",
        "rdrand dest",
        "rdrand dest",
        "Read random number",
        "rdrand eax\n",
        "rdrand %eax\n",
        "Generates a hardware random number into dest. Excellent for generating encryption keys or initialization vectors on the fly without system calls.",
    ),
    InstructionInfo::new(
        "lahf",
        "lahf",
        "Load status flags into AH register",
        "lahf\n",
        "Copies the low byte of the EFLAGS register into AH. Useful for saving flags without pushing onto the stack or for custom flag-manipulation loops.",
    ),
    InstructionInfo::new(
        "sahf",
        "sahf",
        "Store AH register into status flags",
        "sahf\n",
        "Loads AH into low byte of EFLAGS. Useful for restoring flags or custom flag adjustments during code decoding / evasion.",
    ),
    InstructionInfo::new_att(
        "bt",
        "bt dest, bit",
        "btl bit, dest",
        "Bit test",
        "bt eax, 3\n",
        "btl $3, %eax\n",
        "Tests a single bit at the specified index and copies it to the Carry Flag (CF). Great for compact bitmask checks.",
    ),
    InstructionInfo::new_att(
        "bts",
        "bts dest, bit",
        "btsl bit, dest",
        "Bit test and set",
        "bts eax, 3\n",
        "btsl $3, %eax\n",
        "Tests a bit, saves it to CF, and sets that bit in the destination. Useful for constructing bitmasks or setting permissions flags dynamically.",
    ),
    InstructionInfo::new_att(
        "btr",
        "btr dest, bit",
        "btrl bit, dest",
        "Bit test and reset",
        "btr eax, 3\n",
        "btrl $3, %eax\n",
        "Tests a bit, saves it to CF, and clears (zeroes) that bit in destination. Saves space when masking out specific status flags.",
    ),
    InstructionInfo::new_att(
        "btc",
        "btc dest, bit",
        "btcl bit, dest",
        "Bit test and complement",
        "btc eax, 3\n",
        "btcl $3, %eax\n",
        "Tests a bit, saves it to CF, and toggles (complements) that bit in destination. Perfect for bitwise encryption loops.",
    ),
    InstructionInfo::new_att(
        "bsf",
        "bsf dest, src",
        "bsfl src, dest",
        "Bit scan forward",
        "bsf eax, ebx\n",
        "bsfl %ebx, %eax\n",
        "Scans src for the least significant set bit (LSB) and stores index in dest. Helpful in custom allocators or encoding loops.",
    ),
    InstructionInfo::new_att(
        "bsr",
        "bsr dest, src",
        "bsrl src, dest",
        "Bit scan reverse",
        "bsr eax, ebx\n",
        "bsrl %ebx, %eax\n",
        "Scans src for the most significant set bit (MSB) and stores index in dest. Used to quickly locate the highest set bit in integer math.",
    ),
    InstructionInfo::new_att(
        "popcnt",
        "popcnt dest, src",
        "popcntl src, dest",
        "Population count (count set bits)",
        "popcnt eax, ebx\n",
        "popcntl %ebx, %eax\n",
        "Counts the number of bits set to 1 in the source. Can be useful in custom parity checks, cryptography, or hashing routines.",
    ),
    InstructionInfo::new(
        "loop",
        "loop label",
        "Decrement ECX and jump if not zero",
        "loop label\n",
        "Decrements ECX and jumps if non-zero. Compact 2-byte branch instruction for decoder/unpacker loops.",
    ),
    InstructionInfo::new(
        "fldz",
        "fldz",
        "Push +0.0 onto FPU register stack",
        "fldz\n",
        "Pushes +0.0 onto FPU stack. Part of the FPU getpc technique to trigger FPU state saving without calling.",
    ),
    InstructionInfo::new(
        "fnstenv",
        "fnstenv [addr]",
        "Store FPU environment to memory",
        "fnstenv [esp - 12]\n",
        "Saves FPU environment to stack. Writes the instruction pointer of the last FPU instruction (fldz), revealing EIP.",
    ),
];

static ARM_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new(
        "mov",
        "mov dest, src",
        "Move register or immediate value",
        "mov r0, r1\n",
        "Avoid large immediate moves as they may contain null bytes. Use 8-bit immediate values or arithmetic to build values.",
    ),
    InstructionInfo::new(
        "eor",
        "eor dest, src1, src2",
        "Bitwise Exclusive OR",
        "eor r0, r0, r0\n",
        "Clear registers without null bytes. 'eor r0, r0, r0' is the standard way to set a register to zero in ARM.",
    ),
    InstructionInfo::new(
        "add",
        "add dest, src1, src2",
        "Add operands",
        "add r0, r1, r2\n",
        "Can be used to modify pointers or add small values. Helpful for constructing dynamic values and switching to Thumb mode.",
    ),
    InstructionInfo::new(
        "sub",
        "sub dest, src1, src2",
        "Subtract operands",
        "sub r0, r1, r2\n",
        "Subtracts operands. Can be used to clear registers (sub r0, r0, r0) or adjust the stack pointer without nulls.",
    ),
    InstructionInfo::new(
        "ldr",
        "ldr dest, [addr]",
        "Load register from memory",
        "ldr r0, [r1]\n",
        "Load register. In position-independent shellcode, use PC-relative loads (e.g. ldr r0, [pc, #offset]) to read constants.",
    ),
    InstructionInfo::new(
        "str",
        "str src, [addr]",
        "Store register to memory",
        "str r0, [r1]\n",
        "Stores register values to memory. Useful for writing syscall argument structures or modifying local stack data.",
    ),
    InstructionInfo::new(
        "b",
        "b label",
        "Branch unconditionally to label",
        "b label\n",
        "PC-relative unconditional branch. Extremely useful for position-independent relative jumps in shellcode.",
    ),
    InstructionInfo::new(
        "bl",
        "bl label",
        "Branch with link (call function)",
        "bl function\n",
        "Branch with Link. Saves the return address in the Link Register (LR). Used to execute subroutines in shellcode.",
    ),
    InstructionInfo::new(
        "bx",
        "bx reg",
        "Branch and exchange (indirect jump / return)",
        "bx lr\n",
        "Branch and Exchange. Crucial for switching between 32-bit ARM and 16-bit Thumb execution states (by setting LSB of target address).",
    ),
    InstructionInfo::new(
        "cmp",
        "cmp op1, op2",
        "Compare operands and set status flags",
        "cmp r0, r1\n",
        "Compares register values. Updates the CPSR register status flags, enabling subsequent conditional branches.",
    ),
    InstructionInfo::new(
        "svc",
        "svc imm",
        "Supervisor call (system call)",
        "svc #0\n",
        "Supervisor call (formerly SWI). Triggers a system call. Syscall number goes in R7, arguments in R0-R6.",
    ),
    InstructionInfo::new(
        "push",
        "push {regs}",
        "Push registers onto the stack",
        "push {r0, lr}\n",
        "Pushes a set of registers onto the stack. Extremely compact way to save state or allocate temporary buffers on the stack.",
    ),
    InstructionInfo::new(
        "pop",
        "pop {regs}",
        "Pop registers from the stack",
        "pop {r0, pc}\n",
        "Pops registers from the stack. Popping into PC acts as a return from subroutine, possibly exchanging state (ARM/Thumb).",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "No operation. In ARM, this is typically encoded as 'mov r0, r0'. Used for padding or delay loops.",
    ),
    InstructionInfo::new(
        "adr",
        "adr rd, label",
        "Form PC-relative address of label",
        "adr r0, label\n",
        "Form PC-relative address of label. Essential for position-independent shellcode to find data and strings without nulls.",
    ),
    InstructionInfo::new(
        "add",
        "add rd, pc, #1",
        "Add 1 to PC for Thumb transition",
        "add r3, pc, #1\n",
        "Adds 1 to PC. Setting the least significant bit (LSB) of the target address signals 'bx' to switch execution to Thumb mode.",
    ),
];

static THUMB_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new(
        "mov",
        "mov dest, src",
        "Move register or immediate value",
        "mov r0, r1\n",
        "Thumb instructions are 16-bit, saving space and naturally reducing the frequency of null bytes in shellcode.",
    ),
    InstructionInfo::new(
        "eor",
        "eor dest, src",
        "Bitwise Exclusive OR",
        "eor r0, r1\n",
        "Exclusive OR. Clear a register using the 2-register format to save space and avoid nulls.",
    ),
    InstructionInfo::new(
        "add",
        "add dest, src",
        "Add operands",
        "add r0, r1\n",
        "Adds register or immediate. Extremely useful for stack manipulation and address adjustments in compact space.",
    ),
    InstructionInfo::new(
        "sub",
        "sub dest, src",
        "Subtract operands",
        "sub r0, r1\n",
        "Subtracts registers. Often used to adjust the stack or counter registers in compact shellcode loops.",
    ),
    InstructionInfo::new(
        "ldr",
        "ldr dest, [addr]",
        "Load register from memory",
        "ldr r0, [r1]\n",
        "Loads from memory. Thumb uses compact 16-bit PC-relative loads for constant strings or pointers.",
    ),
    InstructionInfo::new(
        "str",
        "str src, [addr]",
        "Store register to memory",
        "str r0, [r1]\n",
        "Stores registers to memory. Used to write function arguments or build payloads in memory structures.",
    ),
    InstructionInfo::new(
        "b",
        "b label",
        "Branch unconditionally to label",
        "b label\n",
        "Short relative branch. Saves space in compact loops and conditional execution blocks.",
    ),
    InstructionInfo::new(
        "bl",
        "bl label",
        "Branch with link (call function)",
        "bl label\n",
        "Branch with Link. Relative call to a label using 32-bit instruction encoding in Thumb mode.",
    ),
    InstructionInfo::new(
        "blx",
        "blx reg",
        "Branch with link and exchange",
        "blx r0\n",
        "Branch with Link and Exchange. Calls the address in a register and switches to ARM mode if the target's LSB is 0.",
    ),
    InstructionInfo::new(
        "bx",
        "bx reg",
        "Branch and exchange (indirect jump / return)",
        "bx lr\n",
        "Branch and Exchange. Typically used to return (bx lr) or transition back to 32-bit ARM mode (if address LSB is 0).",
    ),
    InstructionInfo::new(
        "cmp",
        "cmp op1, op2",
        "Compare operands and set status flags",
        "cmp r0, r1\n",
        "Compares registers. Sets status flags for subsequent conditional branches in compact code.",
    ),
    InstructionInfo::new(
        "svc",
        "svc imm",
        "Supervisor call (system call)",
        "svc #1\n",
        "Supervisor call. On Thumb, triggers a system call. Syscall number goes in R7, and arguments in R0-R4.",
    ),
    InstructionInfo::new(
        "push",
        "push {regs}",
        "Push registers onto the stack",
        "push {r0, lr}\n",
        "Pushes registers. Very compact 16-bit instruction for stack allocation and saving state.",
    ),
    InstructionInfo::new(
        "pop",
        "pop {regs}",
        "Pop registers from the stack",
        "pop {r0, pc}\n",
        "Pops registers. Popping PC returns from Thumb subroutine, possibly switching back to ARM mode.",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "No-op. Typically encoded as 'mov r8, r8' in Thumb to avoid modifying execution state.",
    ),
    InstructionInfo::new(
        "adr",
        "adr rd, label",
        "Form PC-relative address of label",
        "adr r0, label\n",
        "Form PC-relative address. Crucial for position-independent string referencing in compact Thumb mode.",
    ),
];

static AARCH64_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new(
        "mov",
        "mov dest, src",
        "Move register or immediate value",
        "mov x0, x1\n",
        "Move register/immediate. Use 32-bit W-registers if 64-bit range is not needed to avoid high-byte zeroes in opcodes.",
    ),
    InstructionInfo::new(
        "eor",
        "eor dest, src1, src2",
        "Bitwise Exclusive OR",
        "eor x0, x1, x2\n",
        "Exclusive OR. Used to clear registers. Aarch64 instructions are fixed 32-bit, so clearing registers is clean and null-free.",
    ),
    InstructionInfo::new(
        "add",
        "add dest, src1, src2",
        "Add operands",
        "add x0, x1, x2\n",
        "Adds registers or immediates. Useful for offset calculations, stack allocation, and modifying the stack pointer (SP).",
    ),
    InstructionInfo::new(
        "sub",
        "sub dest, src1, src2",
        "Subtract operands",
        "sub x0, x1, x2\n",
        "Subtracts operands. Commonly used to allocate local stack space or decrement offsets in loop counters.",
    ),
    InstructionInfo::new(
        "ldr",
        "ldr dest, [addr]",
        "Load register from memory",
        "ldr x0, [x1]\n",
        "Load register. In PIC shellcode, use literal PC-relative loads (e.g. 'ldr x0, label') to fetch data pointers.",
    ),
    InstructionInfo::new(
        "str",
        "str src, [addr]",
        "Store register to memory",
        "str x0, [x1]\n",
        "Stores register to memory. Essential for writing syscall argument blocks or saving register states.",
    ),
    InstructionInfo::new(
        "ldp",
        "ldp dest1, dest2, [addr]",
        "Load pair of registers from memory",
        "ldp x0, x1, [sp]\n",
        "Loads a pair of registers in one instruction. Saves bytes and execution time in stack frame tear-downs.",
    ),
    InstructionInfo::new(
        "stp",
        "stp src1, src2, [addr]",
        "Store pair of registers to memory",
        "stp x0, x1, [sp, #-16]!\n",
        "Stores a pair of registers and updates the stack pointer. Essential for stack allocation and function prologues.",
    ),
    InstructionInfo::new(
        "b",
        "b label",
        "Branch unconditionally to label",
        "b label\n",
        "PC-relative unconditional branch. Position-independent and does not require absolute memory addresses.",
    ),
    InstructionInfo::new(
        "bl",
        "bl label",
        "Branch with link (call function)",
        "bl function\n",
        "Branch with Link. Saves the return address in Link Register (X30). Standard function call mechanism in Aarch64.",
    ),
    InstructionInfo::new(
        "cmp",
        "cmp op1, op2",
        "Compare operands and set status flags",
        "cmp x0, x1\n",
        "Compares registers (alias of 'subs'). Sets condition flags for subsequent conditional branches (b.eq, b.ne, etc.).",
    ),
    InstructionInfo::new(
        "svc",
        "svc imm",
        "Supervisor call (system call)",
        "svc #0\n",
        "Supervisor call. Triggers a system call. Syscall number goes in X8, and arguments are passed in X0-X7.",
    ),
    InstructionInfo::new(
        "ret",
        "ret [reg]",
        "Return from subroutine",
        "ret\n",
        "Returns from subroutine by jumping to the address in X30 (LR) or a custom specified register.",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "No operation. Encoded as a fixed 32-bit word (0xd503201f). Used for code padding or instruction alignment.",
    ),
    InstructionInfo::new(
        "adr",
        "adr rd, label",
        "Form PC-relative address of label",
        "adr x0, label\n",
        "Form PC-relative address within +/- 1MB. Highly critical for loading string and data addresses in PIC shellcode.",
    ),
    InstructionInfo::new(
        "adrp",
        "adrp rd, label",
        "Form PC-relative page address of label",
        "adrp x0, label\n",
        "Form PC-relative page address within +/- 4GB. Usually paired with 'add' to resolve absolute pointers of symbols.",
    ),
];

static RISCV_INSTRUCTIONS: &[InstructionInfo] = &[
    InstructionInfo::new(
        "li",
        "li rd, imm",
        "Load immediate value",
        "li a0, 1\n",
        "Load immediate. Translates to addi or lui depending on value size. Keep immediate small to minimize code size.",
    ),
    InstructionInfo::new(
        "la",
        "la rd, symbol",
        "Load address of symbol",
        "la a0, label\n",
        "Load address. Pseudo-instruction that generates PC-relative address sequences, critical for PIC shellcode.",
    ),
    InstructionInfo::new(
        "mv",
        "mv rd, rs",
        "Copy register values",
        "mv a0, a1\n",
        "Copy register value. Pseudo-instruction for 'addi rd, rs, 0'. Copies values without memory accesses.",
    ),
    InstructionInfo::new(
        "add",
        "add rd, rs1, rs2",
        "Add registers",
        "add a0, a1, a2\n",
        "Adds register values. Useful for offset calculations and dynamic memory index addressing.",
    ),
    InstructionInfo::new(
        "sub",
        "sub rd, rs1, rs2",
        "Subtract registers",
        "sub a0, a1, a2\n",
        "Subtracts register values. Useful for pointer adjustments and loop counter decrements.",
    ),
    InstructionInfo::new(
        "xor",
        "xor rd, rs1, rs2",
        "Exclusive OR",
        "xor a0, a0, a0\n",
        "Exclusive OR. Can be used to clear registers, though 'li rd, 0' is the standard way to clear in RISC-V.",
    ),
    InstructionInfo::new(
        "lw",
        "lw rd, offset(rs1)",
        "Load word from memory",
        "lw a0, 0(a1)\n",
        "Loads a 32-bit word from memory. Ensure the memory address is aligned to avoid hardware alignment faults.",
    ),
    InstructionInfo::new(
        "sw",
        "sw rs2, offset(rs1)",
        "Store word to memory",
        "sw a0, 0(a1)\n",
        "Stores a 32-bit word to memory. Essential for building syscall structure arguments on the stack.",
    ),
    InstructionInfo::new(
        "jal",
        "jal rd, offset",
        "Jump and link (call subroutine)",
        "jal ra, label\n",
        "Jump and Link. PC-relative jump up to +/- 1MB, storing return address in RA. Used for PIC function calls.",
    ),
    InstructionInfo::new(
        "jalr",
        "jalr rd, offset(rs1)",
        "Jump and link register",
        "jalr ra, 0(t0)\n",
        "Jump and Link Register. Indirect jump to register, enabling dynamic function calls and returns (jalr zero, 0(ra)).",
    ),
    InstructionInfo::new(
        "beq",
        "beq rs1, rs2, offset",
        "Branch if registers equal",
        "beq a0, a1, label\n",
        "Branch if equal. Used for conditional flow control, loop termination, and verification checks.",
    ),
    InstructionInfo::new(
        "bne",
        "bne rs1, rs2, offset",
        "Branch if registers not equal",
        "bne a0, a1, label\n",
        "Branch if not equal. Frequently used to loop back in string-processing or shellcode decoders.",
    ),
    InstructionInfo::new(
        "ecall",
        "ecall",
        "Environment call (system call)",
        "ecall\n",
        "Environment call. Triggers a system call. Syscall number goes in A7, arguments in A0-A5, return value in A0.",
    ),
    InstructionInfo::new(
        "nop",
        "nop",
        "No operation",
        "nop\n",
        "No operation. Pseudo-instruction for 'addi x0, x0, 0'. Used for padding or alignment.",
    ),
    InstructionInfo::new(
        "auipc",
        "auipc rd, imm",
        "Add Upper Immediate to PC",
        "auipc a0, 0\n",
        "Add Upper Immediate to PC. Extremely critical for finding position-independent data offsets.",
    ),
    InstructionInfo::new(
        "c.li",
        "c.li rd, imm",
        "Compressed load immediate (16-bit)",
        "c.li a0, 1\n",
        "Compressed load immediate. Uses 16-bit RVC encoding, saving space and avoiding null bytes in shellcode payloads.",
    ),
    InstructionInfo::new(
        "c.mv",
        "c.mv rd, rs",
        "Compressed register move (16-bit)",
        "c.mv a0, a1\n",
        "Compressed register move. Uses 16-bit RVC encoding, allowing register copying in very restricted environments.",
    ),
];

pub(crate) fn get_instructions(arch: TargetArch) -> &'static [InstructionInfo] {
    match arch {
        TargetArch::X86_64 => X86_64_INSTRUCTIONS,
        TargetArch::X86 => X86_INSTRUCTIONS,
        TargetArch::Arm => ARM_INSTRUCTIONS,
        TargetArch::Thumb => THUMB_INSTRUCTIONS,
        TargetArch::Aarch64 => AARCH64_INSTRUCTIONS,
        TargetArch::Riscv => RISCV_INSTRUCTIONS,
    }
}

pub fn render_instructions_panel(app: &mut ShellcideApp, ui: &mut egui::Ui) {
    let arch_name = app.target_arch.display_name();
    ui.heading(format!("{} Instruction Reference", arch_name));

    ui.horizontal(|ui| {
        ui.label("🔍");
        ui.add(
            egui::TextEdit::singleline(&mut app.instructions_search)
                .hint_text("Search...")
                .desired_width(120.0),
        );
        if ui.button("Clear").clicked() {
            app.instructions_search.clear();
        }
    });
    ui.separator();

    let search = app.instructions_search.trim().to_string();
    let instructions = get_instructions(app.target_arch);

    egui::ScrollArea::both()
        .id_salt("instructions_scroll")
        .show(ui, |ui| {
            egui::Grid::new("instructions_grid")
                .striped(true)
                .num_columns(6)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    let headers = [
                        "Drag",
                        "Action",
                        "Instruction",
                        "Syntax",
                        "Description",
                        "Shellcode Tip",
                    ];
                    for h in headers {
                        crate::ui::theme::header_label(ui, h);
                    }
                    ui.end_row();

                    for inst in instructions {
                        if !search.is_empty() {
                            let matches = crate::ui::contains_case_insensitive(inst.name, &search)
                                || crate::ui::contains_case_insensitive(inst.syntax_intel, &search)
                                || crate::ui::contains_case_insensitive(inst.syntax_att, &search)
                                || crate::ui::contains_case_insensitive(inst.description, &search)
                                || crate::ui::contains_case_insensitive(
                                    inst.shellcode_tip,
                                    &search,
                                );
                            if !matches {
                                continue;
                            }
                        }

                        // Drag handle
                        let syntax_text = inst.syntax(app.att_syntax);
                        let template_text = inst.template(app.att_syntax);

                        // Drag source payload. Prefix with "inst:" to let editor drop zone recognize it.
                        let payload = format!("inst:{}", template_text);
                        let item_id =
                            egui::Id::new(format!("dnd_inst_{}_{}", arch_name, inst.name));
                        ui.dnd_drag_source(item_id, payload, |ui| {
                            ui.label(
                                egui::RichText::new("⠿")
                                    .monospace()
                                    .color(crate::ui::theme::CYBER_CYAN),
                            );
                        });

                        // Action / Insert & Copy buttons
                        ui.horizontal(|ui| {
                            if ui.button("➕").on_hover_text("Insert at cursor").clicked() {
                                app.insert_into_editor(ui.ctx(), template_text);
                                app.log(&format!(
                                    "[Editor] Inserted template: {}",
                                    template_text.trim()
                                ));
                            }
                            if ui.button("📋").on_hover_text("Copy template").clicked() {
                                ui.ctx().copy_text(template_text.to_string());
                            }
                        });

                        // Name
                        ui.label(egui::RichText::new(inst.name).strong());

                        // Syntax (monospace)
                        ui.label(egui::RichText::new(syntax_text).monospace());

                        // Description
                        ui.label(inst.description);

                        // Shellcode Tip (wrapped)
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(inst.shellcode_tip)
                                    .color(crate::ui::theme::SOFT_GREEN),
                            )
                            .wrap(),
                        );

                        ui.end_row();
                    }
                });
        });
}
