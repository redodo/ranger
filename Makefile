dump:
	@objdump \
		--disassembler-color=on \
		--source \
		--source-comment="▌ " \
		--no-addresses \
		--no-show-raw-insn \
		--line-numbers \
		target/release/ranger | rustfilt | less -R
