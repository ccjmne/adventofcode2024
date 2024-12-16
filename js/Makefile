.ONESHELL:
.PHONY: default % all get set open

DAY ?= $(shell date +%d)

default:
	[ -f '$(DAY).txt' ] || $(MAKE) get DAY=$(DAY)
	[ -f '$(DAY).mjs' ] || $(MAKE) set DAY=$(DAY)
	fd '$(DAY)[.](mjs|txt)' | entr -dc -- node $(DAY).mjs

%:
	$(MAKE) default DAY=$*

all:
	while sleep 1 ; do clear; fd [.]m?js | entr -dcp -- node /_ ; echo 'Press ^C again to quit before respawn' ; done

get:
	curl --silent -b session=$$(cat .session) 'https://adventofcode.com/2024/day/$(DAY)/input' > '$(DAY).txt'

set:
	cat > '$(DAY).mjs' <<-EOF
		import { readFile } from 'fs/promises'
		const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
		const data = String(await readFile(__dirname + '/$(DAY).txt'))
		console.log(data)
	EOF

open:
	[ -f '$(DAY).txt' ] || $(MAKE) get DAY=$(DAY)
	[ -f '$(DAY).mjs' ] || $(MAKE) set DAY=$(DAY)
	$$EDITOR '$(DAY).mjs'
