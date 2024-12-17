.ONESHELL:
.PHONY: default % all get set open

DAY ?= $(shell date +%d)

default:
	[ -f '$(DAY).txt' ] || $(MAKE) get DAY=$(DAY)
	[ -f '$(DAY).mjs' ] || $(MAKE) set DAY=$(DAY)
	# Don't use fd here, so that we can start entr
	# with the right watch before the input is downloaded.
	echo -e '$(DAY).mjs\n$(DAY).txt' | entr -c -- node $(DAY).mjs

%:
	$(MAKE) default DAY=$*

all:
	# TODO: Also watch .txt files but still run node on its mjs
	while sleep 1 ; do clear; fd [.]m?js | entr -dcp -- node /_ ; echo 'Press ^C again to quit before respawn' ; done

get:
	curl --silent -b session=$$(cat .session) 'https://adventofcode.com/2024/day/$(DAY)/input' > '$(DAY).txt'

set:
	cat > '$(DAY).mjs' <<-EOF
		import { readFile } from 'fs/promises'
		const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
		const data = String(await readFile(__dirname + '/$(DAY).txt')).trim()
		console.log(data)
	EOF

open:
	[ -f '$(DAY).txt' ] || $(MAKE) get DAY=$(DAY)
	[ -f '$(DAY).mjs' ] || $(MAKE) set DAY=$(DAY)
	$$EDITOR '$(DAY).mjs'
