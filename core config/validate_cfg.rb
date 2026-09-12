require 'udb'
require 'pathname'
require 'yaml'
require 'tempfile'

config_path = ARGV[0] or abort 'Usage: expand_config.rb CONFIG_PATH'
resolver = Udb::Resolver.new
cfg_arch = resolver.cfg_arch_for(Pathname.new(config_path))
unless cfg_arch.fully_configured?
  warn "Config '#{cfg_arch.name}' is not fully configured; aborting."
  exit 1
end

original = cfg_arch.implemented_extension_versions
expanded = cfg_arch.expand_implemented_extension_list(original)

original_set = original.map { |ev| [ev.name, ev.version_spec.to_s] }.to_set
expanded_set = expanded.map { |ev| [ev.name, ev.version_spec.to_s] }.to_set

unless expanded_set == original_set
  puts "Expanded extensions:"
  expanded.sort_by(&:name).each do |ev|
    puts "  #{ev.name} #{ev.version_spec}"
  end
end

result = cfg_arch.valid?
if result.valid
  warn 'config is VALID.'
else
  warn 'config is INVALID:'
  result.reasons.each { |r| warn "  - #{r}" }
end


require 'fileutils'

out_dir = "output"
FileUtils.mkdir_p(out_dir)

csrs_path = File.join(out_dir, "csrs.txt")
instructions_path = File.join(out_dir, "instructions.txt")
exceptions_path = File.join(out_dir, "exceptions.txt")
# ---------------------------------------------------------------------------
#  CSRs
# ---------------------------------------------------------------------------
File.write(
  csrs_path,
  cfg_arch.implemented_csrs.map(&:name).join("\n") + "\n"
)

warn "Wrote #{csrs_path} (#{cfg_arch.implemented_csrs.size} CSRs)"

# ---------------------------------------------------------------------------
#  Instructions
# ---------------------------------------------------------------------------
instructions = cfg_arch.implemented_extension_versions.flat_map do |extension|
  extension.all_instructions_that_must_be_implemented.map(&:name)
end

File.write(
  instructions_path,
  instructions.join("\n") + "\n"
)

warn "Wrote #{instructions_path} (#{instructions.size} instructions)"

# TODO: this may return more than actually required I believe
# ---------------------------------------------------------------------------
#  Interrupts
# ---------------------------------------------------------------------------
File.write(
  exceptions_path,
  cfg_arch.implemented_interrupt_codes.map(&:name).join("\n") + "\n"
)

warn "Wrote #{exceptions_path} (#{cfg_arch.implemented_interrupt_codes.size} exceptions)"
