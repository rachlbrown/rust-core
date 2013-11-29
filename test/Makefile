SOURCES = $(wildcard *.rs)
BINARIES = $(patsubst %.rs,%,$(SOURCES))

all: $(BINARIES)

%: %.rs
	rustc $< --emit-llvm --cfg libc -O
	clang $@.bc -o $@ -O2 -lpthread -lrustrt
	./$@
