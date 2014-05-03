SRCDIR := src
BINDIR := bin

progs := $(patsubst $(SRCDIR)/%.rs,$(BINDIR)/%,$(wildcard $(SRCDIR)/*.rs))

all: $(progs)

$(progs): $(BINDIR)/% : $(SRCDIR)/%.rs | $(BINDIR)
	rustc $< -o $@

$(BINDIR):
	mkdir -p $(BINDIR)

.PHONY: clean

clean:
	rm -rf $(BINDIR)
