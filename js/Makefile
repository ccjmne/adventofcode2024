.PHONY: run watch edit clear
.ONESHELL:

day  ?= $(shell date +%d)
DEPS := $(day).txt $(day).mjs

run: $(DEPS)
	node $(day).mjs

watch: $(DEPS)
	entr -dc node /_ <<< $(day).mjs

edit: $(DEPS)
	$$EDITOR $(day).mjs

clear:
	rm -f $(DEPS)

%.txt:
	curl --silent -b session=$$(cat .session) 'https://adventofcode.com/2024/day/$*/input' > $@

%.mjs:
	cat > $*.mjs <<-EOF
		import { readFile } from 'fs/promises'
		const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
		const data = String(await readFile(__dirname + '/$*.txt')).trim()
		console.log(data)
	EOF
