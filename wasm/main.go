package main

import (
	"github.com/syncfuture/go/sconfig"
	log "github.com/syncfuture/go/slog"
	"github.com/syncfuture/host/sfasthttp"
)

func main() {
	cp := sconfig.NewJsonConfigProvider()
	log.Init(cp)
	h := sfasthttp.NewFHWebHost(cp)

	h.ServeFiles("/{filepath:*}", "./www")

	log.Fatal(h.Run())
}
