.PHONY: run watch edit clear
.ONESHELL:

day  ?= $(shell date +%d)
DEPS := $(day).ts $(day).txt node_modules tsconfig.json

run: $(DEPS)
	tsx $<

watch: $(DEPS)
	tr ' ' '\n' <<< '$^' | entr -c tsx $<

edit: $(DEPS)
	$$EDITOR $<

clean:
	git clean -Xdf

%.txt:
	curl --silent -b session=$(AOC_SESSION) https://adventofcode.com/2024/day/$*/input > $@

%.ts:
	cat > $@ <<-EOF
		// https://adventofcode.com/2024/day/$*
		import { readFile } from 'fs/promises'
		const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
		const data = String(await readFile(__dirname + '/$*.txt')).trim()
		console.log(data)
	EOF

node_modules: package.json
	pnpm install

package.json:
	cat > $@ <<< '{ "type": "module", "dependencies": { "@types/node": "latest" } }'

tsconfig.json:
	cat > $@ <<< '{ "compilerOptions": { "target": "ESNext", "module": "ESNext" } }'
