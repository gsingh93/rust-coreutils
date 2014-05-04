SRCDIR := src
BINDIR := bin

# Compile all programs in the src/ folder except library files
LIB := $(SRCDIR)/util.rs
PROGS := $(patsubst $(SRCDIR)/%.rs,$(BINDIR)/%,$(filter-out $(LIB),$(wildcard $(SRCDIR)/*.rs)))

all: $(PROGS)

$(PROGS): $(BINDIR)/% : $(SRCDIR)/%.rs $(LIB) | $(BINDIR)
	rustc $< -o $@

$(BINDIR):
	mkdir -p $(BINDIR)

.PHONY: clean

clean:
	rm -rf $(BINDIR)
