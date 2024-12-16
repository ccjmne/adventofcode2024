.PHONY: default %

default:
	while sleep 1 ; do fd \.m?js | entr -dcp -- node /_ ; done

%:
	fd '$*.m?js' | entr -dc -- node /_
