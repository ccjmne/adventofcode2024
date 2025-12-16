.PHONY: run watch edit clean
.ONESHELL:

year ?= $(shell TZ=America/New_York date +%Y)
day  ?= $(shell TZ=America/New_York date +%-d)
DEPS := $(year)-$(day).ts $(year)-$(day).txt node_modules tsconfig.json

run: $(DEPS)
	tsx $< < $(word 2,$^)

watch: $(DEPS)
	tr ' ' '\n' <<< '$^' | entr -c -- sh -c 'tsx $< < $(word 2,$^)'

edit: $(DEPS)
	$$EDITOR $<

clean:
	git clean -Xdf

%.txt:
	curl --silent --cookie session=$(AOC_SESSION) https://adventofcode.com/$(word 1,$*)/day/$(word 2,$*)/input > $@

%.ts:
	cat > $@ <<-EOF
		// https://adventofcode.com/$(word 1,$*)/day/$(word 2,$*)
		import { text } from 'node:stream/consumers'
		const data = (await text(process.stdin)).trim()
		console.log(data)
	EOF

node_modules: package.json
	pnpm install

package.json:
	cat > $@ <<< '{ "type": "module", "dependencies": { "@types/node": "latest" } }'

tsconfig.json:
	cat > $@ <<< '{ "compilerOptions": { "target": "ESNext", "module": "ESNext" } }'
