SOURCES = $(wildcard *.rs)
BINARIES = $(patsubst %.rs,%,$(SOURCES))
INTERMEDIATES = $(patsubst %.rs,%.bc,$(SOURCES))

all: $(BINARIES)

%: %.rs
	rustc $< --emit-llvm --cfg libc -O
	clang $@.bc -o $@ -O2 -lpthread -lrustrt
	./$@

clean:
	rm -f $(BINARIES) $(INTERMEDIATES)
