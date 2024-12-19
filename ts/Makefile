.PHONY: run watch edit clear
.ONESHELL:

day  ?= $(shell date +%d)
DEPS := $(day).ts $(day).txt node_modules

run: $(DEPS)
	tsx $<

watch: $(DEPS)
	tr ' ' '\n' <<< '$^' | entr -c tsx $<

edit: $(DEPS)
	$$EDITOR $<

clear:
	rm -rf $(DEPS)

%.txt:
	curl --silent -b session=$(AOC_SESSION) https://adventofcode.com/2024/day/$*/input > $@

%.ts:
	cat > $*.ts <<-EOF
		// https://adventofcode.com/2024/day/$*
		import { readFile } from 'fs/promises'
		const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
		const data = String(await readFile(__dirname + '/$*.txt')).trim()
		console.log(data)
	EOF

node_modules: package.json
	pnpm install
